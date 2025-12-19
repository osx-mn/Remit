// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod backend_db;
mod find_devices;

use mdns_sd::ServiceDaemon;
use std::sync::{Arc, Mutex};
use tauri::Manager;

// Estructura para almacenar el daemon de mdns y que funcione para acceder globalmente a él
pub struct MdnsState {
    pub daemon: Arc<Mutex<Option<ServiceDaemon>>>,
    pub service_full_name: Arc<Mutex<Option<String>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(MdnsState {
            daemon: Arc::new(Mutex::new(None)),
            service_full_name: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            backend_db::consultas_db,
            backend_db::user_app,
            backend_db::change_username,
            find_devices::find_devices
        ])
        .setup(|_app| {
            // Enable native window decorations in macOs and Linux
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                use tauri::Manager;
                if let Some(window) = _app.get_webview_window("main") {
                    window.set_decorations(true)?;
                }
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                println!("App cerrandose!");

                let state = app_handle.state::<MdnsState>();

                println!(
                    "Desconectando el servicio: {}",
                    &state.service_full_name.lock().unwrap().as_ref().unwrap()
                );
                if let Ok(mut guard) = state.daemon.lock() {
                    if let Some(daemon) = guard.take() {
                        let _ = daemon
                            .unregister(&state.service_full_name.lock().unwrap().as_ref().unwrap());
                        let _ = daemon.shutdown();
                    }
                };
            }
        })
}
