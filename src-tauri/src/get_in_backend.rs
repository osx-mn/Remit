use tauri::command;

#[command]
pub fn get_in_backend(file_path: String) {
    println!("variable llegada desde el frontend: {}", file_path);
}
