use std::io::{Read, Write};
use std::str::FromStr;
#[cfg(not(target_os = "android"))]
use std::path::Path;
use suppaftp::FtpStream;
use tauri::{command, Emitter};
use tauri_plugin_fs::{FilePath, FsExt, OpenOptions};

#[cfg(target_os = "android")]
use crate::android_file;

#[command]
pub async fn ftp_client(
    file_path: String,
    target_device: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let ip: String = target_device;
    let port: u16 = 2001;

    //----- Conectarse al servidor ftp
    let mut ftp_stream = match FtpStream::connect(format!("{}:{}", ip, port)) {
        Ok(login_ok) => login_ok,
        Err(e) => return Err(format!("{}", e)),
    };

    //----- Login ftp
    ftp_stream.login("anonymous", "")
        .map_err(|e| format!("Error login FTP: {}", e))?;

    //----- leer el archivo y almacenarlo en load_file
    let selected_path = FilePath::from_str(&file_path)
        .map_err(|e| format!("Ruta de archivo no valida: {}", e))?;
    let mut file_options = OpenOptions::new();
    file_options.read(true);

    let mut load_file = match app_handle.fs().open(selected_path, file_options) {
        Ok(archivo) => archivo,
        Err(e) => return Err(format!("No se pudo abrir el archivo: {}", e)),
    };

    let total_bytes = load_file.metadata().map_err(|e| e.to_string())?.len();
    println!(
        "El archivo cargado tiene un total de {:?} bytes.",
        total_bytes
    );

    // Android devuelve content:// URIs; su ultimo segmento no es el nombre real.
    #[cfg(target_os = "android")]
    let file_name = android_file::display_name(&app_handle, &file_path)?;

    #[cfg(not(target_os = "android"))]
    let file_name: String = Path::new(&file_path)
        .file_name()
        .ok_or_else(|| "No se pudo obtener el nombre del archivo".to_string())?
        .to_string_lossy()
        .into_owned();

    //----- enviar el archivo a travé de ftp_stream
    let mut ftp_writer = ftp_stream
        .put_with_stream(&file_name)
        .map_err(|e| e.to_string())?;

    let mut buffer = [0u8; 8192];
    let mut enviados: u64 = 0;

    let _ = app_handle.emit("send_status", true).unwrap();
    loop {
        let leidos = load_file.read(&mut buffer).map_err(|e| e.to_string())?;

        if leidos == 0 {
            break;
        }

        ftp_writer
            .write_all(&buffer[..leidos])
            .map_err(|e| e.to_string())?;

        enviados += leidos as u64;

        let porcentaje = (enviados as f64 / total_bytes as f64) * 100.0;
        println!("Envío de archivo en {}%", porcentaje);
        let _ = app_handle.emit("send_percentage", porcentaje).unwrap();
    }

    ftp_writer
        .finish()
        .map_err(|e| e.to_string())?;
    let _ = app_handle.emit("send_status", true).unwrap();

    Ok(())
}
