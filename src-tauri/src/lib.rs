mod audio;

use audio::AudioState;

use tauri::{Emitter, Listener, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn register_global_shortcut(app: tauri::AppHandle, shortcut: String, button_id: u32) -> Result<(), String> {
    let shortcut_obj: Shortcut = shortcut.parse().map_err(|e| format!("Invalid shortcut: {}", e))?;
    
    app.global_shortcut().on_shortcut(shortcut_obj.clone(), move |app, _shortcut, event| {
        if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
            let _ = app.emit("global-shortcut-triggered", button_id);
        }
    }).map_err(|e| format!("Failed to register shortcut: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn unregister_global_shortcut(app: tauri::AppHandle, shortcut: String) -> Result<(), String> {
    let shortcut_obj: Shortcut = shortcut.parse().map_err(|e| format!("Invalid shortcut: {}", e))?;
    app.global_shortcut().unregister(shortcut_obj).map_err(|e| format!("Failed to unregister shortcut: {}", e))?;
    Ok(())
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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
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
            audio::update_button_volume,
            audio::save_sound_file,
            audio::delete_sound_file,
            register_global_shortcut,
            unregister_global_shortcut
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
