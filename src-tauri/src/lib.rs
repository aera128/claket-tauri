mod audio;

use audio::AudioState;

use tauri::{Emitter, Listener, Manager};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            
            app.manage(AudioState::new(handle.clone()));

            app.listen_any("tauri://drag-drop", move |event| {
                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(event.payload()) {
                    if let Some(paths) = payload.get("paths").and_then(|p| p.as_array()) {
                        if let Some(first_path) = paths.first().and_then(|p| p.as_str()) {
                            if let Some(position) = payload.get("position").and_then(|p| p.get("x")).and_then(|x| x.as_f64()) {
                                let y = payload.get("position").and_then(|p| p.get("y")).and_then(|y| y.as_f64()).unwrap_or(0.0);
                                let _ = handle.emit("file-dropped", serde_json::json!({
                                    "path": first_path,
                                    "x": position,
                                    "y": y
                                }));
                            }
                        }
                    }
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            audio::list_audio_devices,
            audio::set_audio_device,
            audio::update_master_volume,
            audio::play_sound,
            audio::preload_sound,
            audio::toggle_pause_instance,
            audio::stop_instance,
            audio::seek_instance,
            audio::stop_all,
            audio::save_sound_file,
            audio::delete_sound_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
