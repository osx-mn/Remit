use std::io::{Read, Write};
use std::{fs::File, path::Path};
use suppaftp::FtpStream;
use tauri::{command, Emitter};

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

    assert!(ftp_stream.login("anonymous", "").is_ok());

    //----- leer el archivo y almacenarlo en load_file
    let mut load_file = match File::open(&file_path) {
        Ok(archivo) => archivo,
        Err(e) => return Err(format!("No se pudo abrir el archivo: {}", e)),
    };

    let total_bytes = load_file.metadata().map_err(|e| e.to_string())?.len();
    println!(
        "El archivo cargado tiene un total de {:?} bytes.",
        total_bytes
    );

    //----- obtener el nombre del archivo para compartirlo con el mismo nombre
    let file_name: String = Path::new(&file_path)
        .file_name()
        .unwrap()
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

    ftp_stream
        .finalize_put_stream(ftp_writer)
        .map_err(|e| e.to_string())?;
    let _ = app_handle.emit("send_status", true).unwrap();

    Ok(())
}
