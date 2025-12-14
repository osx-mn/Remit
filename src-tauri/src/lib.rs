// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod backend_db;
mod find_devices;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            backend_db::consultas_db,
            backend_db::user_app,
            backend_db::change_username,
            find_devices::find_devices
        ])
        .setup(|app| {
            // Enable native window decorations only on macOS
            #[cfg(target_os = "macos")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    window.set_decorations(true)?;
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
