use cpal::traits::{DeviceTrait, HostTrait};
use lofty::file::AudioFile;
use lofty::probe::Probe;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source, buffer::SamplesBuffer};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager, State};
use serde::Serialize;

struct SendWrapper<T>(T);
unsafe impl<T> Send for SendWrapper<T> {}
unsafe impl<T> Sync for SendWrapper<T> {}

#[derive(Clone)]
pub struct LevelData {
    pub peak: f32,
    pub rms: f32,
    pub volume: f32,
    pub last_update: Instant,
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            peak: 0.0,
            rms: 0.0,
            volume: 1.0,
            last_update: Instant::now(),
        }
    }
}

struct LevelMeter<S> {
    source: S,
    levels: Arc<Mutex<LevelData>>,
    window_size: usize,
    window_samples: Vec<f32>,
    window_index: usize,
}

impl<S> LevelMeter<S>
where
    S: Source<Item = f32>,
{
    fn new(source: S, levels: Arc<Mutex<LevelData>>) -> Self {
        Self {
            source,
            levels,
            window_size: 512,
            window_samples: vec![0.0; 512],
            window_index: 0,
        }
    }

    fn update_levels(&mut self) {
        if self.window_index < self.window_size {
            return;
        }

        let peak = self.window_samples
            .iter()
            .map(|s| s.abs())
            .fold(0.0f32, f32::max);

        let rms = (self.window_samples
            .iter()
            .map(|s| s * s)
            .sum::<f32>() / self.window_samples.len() as f32)
            .sqrt();

        if let Ok(mut levels) = self.levels.lock() {
            levels.peak = peak;
            levels.rms = rms;
            levels.last_update = Instant::now();
        }

        self.window_index = 0;
    }
}

impl<S> Iterator for LevelMeter<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.source.next() {
            let amplitude = sample;
            
            self.window_samples[self.window_index] = amplitude;
            self.window_index += 1;

            if self.window_index >= self.window_size {
                self.update_levels();
            }

            Some(sample)
        } else {
            None
        }
    }
}

impl<S> Source for LevelMeter<S>
where
    S: Source<Item = f32>,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.source.total_duration()
    }
}

#[derive(Clone, Serialize)]
pub struct MasterLevelEvent {
    pub peak: f32,
    pub rms: f32,
}

pub struct MeterManager {
    app_handle: tauri::AppHandle,
    active_meters: Arc<Mutex<Vec<(Arc<Mutex<LevelData>>, Arc<Sink>)>>>,
    master_volume: Arc<Mutex<f32>>,
}

impl MeterManager {
    pub fn new(app_handle: tauri::AppHandle, master_volume: Arc<Mutex<f32>>) -> Self {
        Self {
            app_handle,
            active_meters: Arc::new(Mutex::new(Vec::new())),
            master_volume,
        }
    }

    pub fn add_meter(&self, meter: Arc<Mutex<LevelData>>, sink: Arc<Sink>) {
        if let Ok(mut meters) = self.active_meters.lock() {
            meters.push((meter, sink));
        }
    }

    pub fn start_monitoring(&self) {
        let app_handle = self.app_handle.clone();
        let active_meters = Arc::clone(&self.active_meters);
        let master_vol_ref = Arc::clone(&self.master_volume);
        
        std::thread::spawn(move || {
            loop {
                let mut master_peak = 0.0f32;
                let mut master_rms = 0.0f32;
                let mut has_any_active_sink = false;

                let meters_snapshot = {
                    if let Ok(mut meters) = active_meters.lock() {
                        meters.retain(|(meter, sink)| {
                            if sink.empty() {
                                false
                            } else {
                                if let Ok(levels) = meter.lock() {
                                    Instant::now().duration_since(levels.last_update) < Duration::from_millis(200)
                                } else {
                                    false
                                }
                            }
                        });
                        
                        if meters.is_empty() {
                            None
                        } else {
                            Some(meters.iter().map(|(m, s)| (m.clone(), s.clone())).collect::<Vec<_>>())
                        }
                    } else {
                        None
                    }
                };

                if let Some(meters) = meters_snapshot {
                    let global_vol = *master_vol_ref.lock().unwrap();
                    for (meter, sink) in meters.iter() {
                        if let Ok(levels) = meter.lock() {
                            if !sink.is_paused() {
                                let vol = levels.volume * global_vol;
                                master_peak = master_peak.max(levels.peak * vol);
                                master_rms = master_rms.max(levels.rms * vol);
                            }
                            has_any_active_sink = true;
                        }
                    }
                }

                if has_any_active_sink {
                    let _ = app_handle.emit("master-level", MasterLevelEvent {
                        peak: master_peak,
                        rms: master_rms,
                    });
                    std::thread::sleep(Duration::from_millis(16));
                } else {
                    let _ = app_handle.emit("master-level", MasterLevelEvent {
                        peak: 0.0,
                        rms: 0.0,
                    });
                    std::thread::sleep(Duration::from_millis(250));
                }
            }
        });
    }
}

const CACHE_THRESHOLD_BYTES: u64 = 5 * 1024 * 1024; // 5MB

#[derive(Clone, Serialize)]
struct AudioProgress {
    id: String,
    instance_id: u32,
    name: String,
    position_ms: u64,
    duration_ms: u64,
    is_paused: bool,
}

#[derive(Clone)]
struct CachedSound {
    channels: u16,
    sample_rate: u32,
    samples: Option<Arc<Vec<f32>>>,
    duration: Duration,
}

pub struct AudioState {
    pub current_device_name: Arc<Mutex<String>>,
    pub master_volume: Arc<Mutex<f32>>,
    pub sinks: Arc<Mutex<HashMap<u32, (String, String, Arc<Sink>, f32, String, std::time::Instant, Duration)>>>,
    active_streams: Arc<Mutex<HashMap<String, (SendWrapper<OutputStream>, OutputStreamHandle)>>>,
    instance_counter: Arc<Mutex<u32>>,
    cache: Arc<Mutex<HashMap<String, CachedSound>>>,
    pub meter_manager: Arc<MeterManager>,
}

impl AudioState {
    pub fn new(app_handle: AppHandle) -> Self {
        let master_volume = Arc::new(Mutex::new(1.0));
        let meter_manager = Arc::new(MeterManager::new(app_handle, Arc::clone(&master_volume)));
        meter_manager.start_monitoring();

        Self {
            current_device_name: Arc::new(Mutex::new("Default".to_string())),
            master_volume,
            sinks: Arc::new(Mutex::new(HashMap::new())),
            active_streams: Arc::new(Mutex::new(HashMap::new())),
            instance_counter: Arc::new(Mutex::new(0)),
            cache: Arc::new(Mutex::new(HashMap::new())),
            meter_manager,
        }
    }

    pub fn get_or_create_stream_handle(&self, device_name: &str) -> Result<OutputStreamHandle, String> {
        let mut streams = self.active_streams.lock().map_err(|_| "Failed to lock active streams")?;
        
        if let Some((_, handle)) = streams.get(device_name) {
            return Ok(handle.clone());
        }

        let host = cpal::default_host();
        let device = if device_name == "Default" {
            host.default_output_device()
        } else {
            host.output_devices().map_err(|e| e.to_string())?
                .find(|d| d.name().map(|n| n == device_name).unwrap_or(false))
        }.ok_or("Audio device not found")?;

        let (stream, handle) = OutputStream::try_from_device(&device).map_err(|e| e.to_string())?;
        streams.insert(device_name.to_string(), (SendWrapper(stream), handle.clone()));
        
        Ok(handle)
    }

    pub fn cleanup_streams(&self, except_device: &str) {
        let mut streams = self.active_streams.lock().unwrap();
        streams.retain(|name, _| name == except_device);
    }

    pub fn migrate_active_sinks(&self, handle: &OutputStreamHandle) {
        let mut sinks_guard = self.sinks.lock().unwrap();
        let cache_guard = self.cache.lock().unwrap();
        let master_vol = *self.master_volume.lock().unwrap();

        for (_instance_id, (_id, path, sink, volume, _name, start_time, base_offset)) in sinks_guard.iter_mut() {
            if let Some(data) = cache_guard.get(path) {
                // Calculate current position before stopping old sink
                let elapsed = if sink.is_paused() {
                    Duration::from_secs(0) // Simplification for paused migration
                } else {
                    start_time.elapsed()
                };
                let current_pos = elapsed + *base_offset;

                // Create new sink on the new device
                if let Ok(new_sink) = Sink::try_new(handle) {
                    let new_sink = Arc::new(new_sink);
                    
                    if let Some(samples) = &data.samples {
                        let source_buffered = SamplesBuffer::new(data.channels, data.sample_rate, (**samples).clone());
                        let skipped_source = source_buffered.skip_duration(current_pos);
                        
                        let levels = Arc::new(Mutex::new(LevelData {
                            peak: 0.0,
                            rms: 0.0,
                            volume: *volume,
                            last_update: Instant::now(),
                        }));
                        let metered_source = LevelMeter::new(skipped_source, levels.clone());
                        
                        self.meter_manager.add_meter(levels, Arc::clone(&new_sink));
                        
                        new_sink.set_volume(*volume * master_vol);
                        new_sink.append(metered_source);
                    } else {
                        // Streaming for large files during migration
                        if let Ok(file) = File::open(path) {
                            let reader = BufReader::new(file);
                            if let Ok(source) = Decoder::new(reader) {
                                let skipped_source = source.skip_duration(current_pos).convert_samples::<f32>();
                                
                                let levels = Arc::new(Mutex::new(LevelData {
                                    peak: 0.0,
                                    rms: 0.0,
                                    volume: *volume,
                                    last_update: Instant::now(),
                                }));
                                let metered_source = LevelMeter::new(skipped_source, levels.clone());
                                
                                self.meter_manager.add_meter(levels, Arc::clone(&new_sink));
                                
                                new_sink.set_volume(*volume * master_vol);
                                new_sink.append(metered_source);
                            }
                        }
                    }
                    
                    if sink.is_paused() {
                        new_sink.pause();
                    }

                    // Stop old sink and replace it
                    sink.stop();
                    *sink = new_sink;
                    *start_time = std::time::Instant::now();
                    *base_offset = current_pos;
                }
            }
        }
    }
}

#[tauri::command]
pub async fn list_audio_devices() -> Result<Vec<String>, String> {
    let host = cpal::default_host();
    let devices = host.output_devices().map_err(|e| e.to_string())?;
    let mut names: Vec<String> = devices.filter_map(|d| d.name().ok()).collect();
    
    names.retain(|name| {
        let n = name.to_lowercase();
        !n.starts_with("hw:") && 
        !n.starts_with("plughw:") && 
        !n.starts_with("dmix:") && 
        !n.starts_with("dsnoop:") &&
        !n.ends_with("rate") && 
        !n.starts_with("speex") &&
        !n.contains("surround") &&
        !n.contains("upmix") &&
        !n.contains("vdownmix")
    });
    
    names.sort();
    names.dedup();
    
    names.insert(0, "Default".to_string());
    Ok(names)
}

#[tauri::command]
pub async fn set_audio_device(state: State<'_, AudioState>, device_name: String) -> Result<(), String> {
    let old_device = {
        let mut device_name_guard = state
            .current_device_name
            .lock()
            .map_err(|_| "Failed to lock audio state")?;
        let old = device_name_guard.clone();
        *device_name_guard = device_name.clone();
        old
    };

    if old_device != device_name {
        // Pre-initialize stream for the new device
        let handle = state.get_or_create_stream_handle(&device_name)?;
        
        // Migrate all active sinks to the new device handle
        state.migrate_active_sinks(&handle);

        // Cleanup old device streams
        state.cleanup_streams(&device_name);
    }

    Ok(())
}

#[tauri::command]
pub async fn update_master_volume(state: State<'_, AudioState>, volume: f32) -> Result<(), String> {
    let mut master_vol = state.master_volume.lock().unwrap();
    *master_vol = volume;
    
    let sinks = state.sinks.lock().unwrap();
    for (_, (_, _, sink, button_vol, _, _, _)) in sinks.iter() {
        sink.set_volume(button_vol * volume);
    }
    Ok(())
}

#[tauri::command]
pub async fn preload_sound(state: State<'_, AudioState>, path: String) -> Result<(), String> {
    let cache = Arc::clone(&state.cache);
    
    std::thread::spawn(move || {
        let mut cache_guard = cache.lock().unwrap();
        if !cache_guard.contains_key(&path) {
            if let Ok(file) = File::open(&path) {
                let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
                let reader = BufReader::new(file);
                
                if let Ok(source) = Decoder::new(reader) {
                    let duration = Probe::open(&path)
                        .ok()
                        .and_then(|probed| probed.read().ok())
                        .map(|tagged| tagged.properties().duration())
                        .unwrap_or_else(|| source.total_duration().unwrap_or(Duration::from_secs(0)));

                    let channels = source.channels();
                    let sample_rate = source.sample_rate();
                    
                    let samples = if file_size <= CACHE_THRESHOLD_BYTES {
                        Some(Arc::new(source.convert_samples().collect()))
                    } else {
                        None
                    };

                    cache_guard.insert(path, CachedSound {
                        channels,
                        sample_rate,
                        samples,
                        duration,
                    });
                }
            }
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn play_sound(
    app: AppHandle,
    state: State<'_, AudioState>,
    id: String,
    path: String,
    name: String,
    volume: f32,
) -> Result<u32, String> {
    let device_name = state.current_device_name.lock().map_err(|_| "Failed to lock device name")?.clone();
    let master_vol = *state.master_volume.lock().unwrap();
    
    // Get cached handle or create new one (eliminates initialization latency)
    let stream_handle = state.get_or_create_stream_handle(&device_name)?;

    let sinks = Arc::clone(&state.sinks);
    let cache = Arc::clone(&state.cache);
    
    let mut counter = state.instance_counter.lock().unwrap();
    *counter += 1;
    let instance_id = *counter;

    let id_clone = id.clone();
    let name_clone = name.clone();
    let path_clone = path.clone();
    let meter_manager = Arc::clone(&state.meter_manager);
    
    std::thread::spawn(move || {
        let sound_data = {
            let mut cache_guard = cache.lock().unwrap();
            if let Some(cached) = cache_guard.get(&path_clone) {
                Some(cached.clone())
            } else {
                if let Ok(file) = File::open(&path_clone) {
                    let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
                    let reader = BufReader::new(file);
                    if let Ok(source) = Decoder::new(reader) {
                        let duration = Probe::open(&path_clone)
                            .ok()
                            .and_then(|probed| probed.read().ok())
                            .map(|tagged| tagged.properties().duration())
                            .unwrap_or_else(|| source.total_duration().unwrap_or(Duration::from_secs(0)));

                        let channels = source.channels();
                        let sample_rate = source.sample_rate();
                        
                        let samples = if file_size <= CACHE_THRESHOLD_BYTES {
                            Some(Arc::new(source.convert_samples().collect()))
                        } else {
                            None
                        };

                        let cached = CachedSound {
                            channels,
                            sample_rate,
                            samples,
                            duration,
                        };
                        cache_guard.insert(path_clone.clone(), cached.clone());
                        Some(cached)
                    } else { None }
                } else { None }
            }
        };

        if let Some(data) = sound_data {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                let sink = Arc::new(sink);
                
                let levels = Arc::new(Mutex::new(LevelData {
                    peak: 0.0,
                    rms: 0.0,
                    volume,
                    last_update: Instant::now(),
                }));
                
                if let Some(samples) = data.samples {
                    let source_buffered = SamplesBuffer::new(data.channels, data.sample_rate, (*samples).clone());
                    let metered_source = LevelMeter::new(source_buffered, levels.clone());
                    sink.append(metered_source);
                } else {
                    // Streaming large file
                    if let Ok(file) = File::open(&path_clone) {
                        let reader = BufReader::new(file);
                        if let Ok(source) = Decoder::new(reader) {
                            let metered_source = LevelMeter::new(source.convert_samples::<f32>(), levels.clone());
                            sink.append(metered_source);
                        }
                    }
                }
                
                meter_manager.add_meter(levels.clone(), Arc::clone(&sink));
                sink.set_volume(volume * master_vol);
                
                let start_time = std::time::Instant::now();
                let base_offset = Duration::from_secs(0);

                {
                    let mut sinks_guard = sinks.lock().unwrap();
                    sinks_guard.insert(instance_id, (id_clone.clone(), path_clone.clone(), Arc::clone(&sink), volume, name_clone.clone(), start_time, base_offset));
                }
                
                let duration_ms = data.duration.as_millis() as u64;
                let mut paused_duration = Duration::from_secs(0);
                let mut last_pause_start = None;
                let mut last_processed_offset = base_offset;

                loop {
                    let (current_sink, current_start_time, current_base_offset) = {
                        let sinks_guard = sinks.lock().unwrap();
                        if let Some((_, _, s, _, _, st, bo)) = sinks_guard.get(&instance_id) {
                            (Arc::clone(s), *st, *bo)
                        } else {
                            break; // Instance was stopped/removed
                        }
                    };

                    if current_base_offset != last_processed_offset {
                        paused_duration = Duration::from_secs(0);
                        last_processed_offset = current_base_offset;
                        if current_sink.is_paused() {
                            last_pause_start = Some(std::time::Instant::now());
                        } else {
                            last_pause_start = None;
                        }
                    }

                    if current_sink.is_paused() {
                        if last_pause_start.is_none() {
                            last_pause_start = Some(std::time::Instant::now());
                        }
                    } else if let Some(pause_start) = last_pause_start {
                        paused_duration += pause_start.elapsed();
                        last_pause_start = None;
                    }

                    let elapsed = if let Some(pause_start) = last_pause_start {
                        pause_start.duration_since(current_start_time).saturating_sub(paused_duration)
                    } else {
                        current_start_time.elapsed().saturating_sub(paused_duration)
                    };

                    let position_ms = elapsed.as_millis() as u64 + current_base_offset.as_millis() as u64;

                    let final_position = std::cmp::min(position_ms, duration_ms);
                    
                    let _ = app.emit("audio-progress", AudioProgress {
                        id: id_clone.clone(),
                        instance_id,
                        name: name_clone.clone(),
                        position_ms: final_position,
                        duration_ms,
                        is_paused: current_sink.is_paused(),
                    });
                    
                    std::thread::sleep(Duration::from_millis(30));
                    
                    if current_sink.empty() { 
                        let sinks_check = sinks.lock().unwrap();
                        if !sinks_check.contains_key(&instance_id) {
                            break; 
                        }
                        
                        std::thread::sleep(Duration::from_millis(10));
                        if let Some((_, _, final_sink, _, _, _, _)) = sinks_check.get(&instance_id) {
                            if final_sink.empty() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }

                {
                    let mut sinks_guard = sinks.lock().unwrap();
                    sinks_guard.remove(&instance_id);
                }
                
                let _ = app.emit("audio-finished", instance_id);
            }
        }
    });

    Ok(instance_id)
}

#[tauri::command]
pub async fn toggle_pause_instance(state: State<'_, AudioState>, instance_id: u32) -> Result<bool, String> {
    let sinks = state.sinks.lock().map_err(|_| "Failed to lock sinks")?;
    if let Some((_, _, sink, _, _, _, _)) = sinks.get(&instance_id) {
        if sink.is_paused() {
            sink.play();
            Ok(false)
        } else {
            sink.pause();
            Ok(true)
        }
    } else {
        Err("Instance not found".to_string())
    }
}

#[tauri::command]
pub async fn stop_instance(state: State<'_, AudioState>, instance_id: u32) -> Result<(), String> {
    let mut sinks = state.sinks.lock().map_err(|_| "Failed to lock sinks")?;
    if let Some((_, _, sink, _, _, _, _)) = sinks.remove(&instance_id) {
        sink.stop();
    }
    Ok(())
}

#[tauri::command]
pub async fn seek_instance(state: State<'_, AudioState>, instance_id: u32, position_ms: u64) -> Result<(), String> {
    let mut sinks = state.sinks.lock().map_err(|_| "Failed to lock sinks")?;
    let cache_guard = state.cache.lock().unwrap();
    let master_vol = *state.master_volume.lock().unwrap();

    if let Some((_, path, sink, volume, _, start_time, base_offset)) = sinks.get_mut(&instance_id) {
        if let Some(data) = cache_guard.get(path) {
            let was_paused = sink.is_paused();
            sink.stop();
            
            let handle = state.get_or_create_stream_handle(&state.current_device_name.lock().unwrap())?;
            if let Ok(new_sink) = Sink::try_new(&handle) {
                let new_sink = Arc::new(new_sink);
                
                let levels = Arc::new(Mutex::new(LevelData {
                    peak: 0.0,
                    rms: 0.0,
                    volume: *volume,
                    last_update: Instant::now(),
                }));
                
                if let Some(samples) = &data.samples {
                    let source_buffered = SamplesBuffer::new(data.channels, data.sample_rate, (**samples).clone());
                    let skipped_source = source_buffered.skip_duration(Duration::from_millis(position_ms));
                    let metered_source = LevelMeter::new(skipped_source, levels.clone());
                    new_sink.append(metered_source);
                } else {
                    // Seek in streamed large file
                    if let Ok(file) = File::open(path) {
                        let reader = BufReader::new(file);
                        if let Ok(source) = Decoder::new(reader) {
                            let skipped_source = source.skip_duration(Duration::from_millis(position_ms)).convert_samples::<f32>();
                            let metered_source = LevelMeter::new(skipped_source, levels.clone());
                            new_sink.append(metered_source);
                        }
                    }
                }
                
                state.meter_manager.add_meter(levels, Arc::clone(&new_sink));
                
                new_sink.set_volume(*volume * master_vol);
                
                if was_paused {
                    new_sink.pause();
                }

                *sink = new_sink;
                *start_time = std::time::Instant::now();
                *base_offset = Duration::from_millis(position_ms);
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn stop_all(state: State<'_, AudioState>) -> Result<(), String> {
    let mut sinks = state.sinks.lock().map_err(|_| "Failed to lock sinks")?;
    for (_, (_, _, sink, _, _, _, _)) in sinks.iter() {
        sink.stop();
    }
    sinks.clear();
    Ok(())
}

#[tauri::command]
pub async fn save_sound_file(app: tauri::AppHandle, path: String) -> Result<String, String> {
    let source_path = Path::new(&path);
    if !source_path.exists() {
        return Err("Source file does not exist".to_string());
    }

    let file_name = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;

    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    let sounds_dir = app_data_dir.join("sounds");

    if !sounds_dir.exists() {
        fs::create_dir_all(&sounds_dir)
            .map_err(|e| format!("Failed to create sounds directory: {}", e))?;
    }

    let mut dest_path = sounds_dir.join(file_name);
    
    if dest_path.exists() {
        let stem = source_path.file_stem().and_then(|s| s.to_str()).unwrap_or("sound");
        let extension = source_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let new_file_name = format!("{}_{}.{}", stem, timestamp, extension);
        dest_path = sounds_dir.join(new_file_name);
    }

    fs::copy(source_path, &dest_path)
        .map_err(|e| format!("Failed to copy file: {}", e))?;

    Ok(dest_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn delete_sound_file(_app: tauri::AppHandle, path: String) -> Result<(), String> {
    let file_path = Path::new(&path);
    
    if !file_path.exists() {
        return Ok(());
    }

    fs::remove_file(file_path)
        .map_err(|e| format!("Failed to delete file: {}", e))?;

    Ok(())
}
