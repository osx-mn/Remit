#[tauri::command]
pub fn show_custom_titlebar_in_os() -> bool {
    !cfg!(any(target_os = "macos", target_os = "linux"))
}
