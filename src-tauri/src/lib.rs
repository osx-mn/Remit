// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod backend_db;
#[cfg(target_os = "android")]
mod android_file;
mod find_devices;
mod ftp_client;
mod ftp_server;
mod utils;

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
    #[cfg_attr(not(target_os = "android"), allow(unused_mut))]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init());

    #[cfg(target_os = "android")]
    {
        builder = builder.plugin(android_file::init().expect("failed to initialize Android file plugin"));
    }

    builder
        .manage(MdnsState {
            daemon: Arc::new(Mutex::new(None)),
            service_full_name: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            backend_db::consultas_db,
            backend_db::user_app,
            backend_db::change_username,
            find_devices::find_devices,
            ftp_server::ftp_server,
            ftp_client::ftp_client,
            utils::show_custom_titlebar_in_os,
            #[cfg(target_os = "android")]
            android_file::get_file_display_name,
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

                // Clonar fuera del lock para no anidar locks.
                let full_name = state.service_full_name.lock().unwrap().clone();
                if let Some(full_name) = full_name.as_ref() {
                    println!("Desconectando el servicio: {}", full_name);
                    if let Ok(mut guard) = state.daemon.lock() {
                        if let Some(daemon) = guard.take() {
                            let _ = daemon.unregister(full_name);
                            let _ = daemon.shutdown();
                        }
                    }
                } else {
                    println!("Sin servicio mDNS registrado, nada que desconectar.");
                }
            }
        })
}
