use local_ip_address::local_ip;
use std::{fs::File, path::Path};
use suppaftp::FtpStream;
use tauri::command;

#[command]
pub async fn ftp_client(file_path: String) -> Result<(), String> {
    let ip: String = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(e) => {
            println!("Error al obtener la IP: {}", e);
            println!("Conectandose a la red local: 127.0.0.1");
            "127.0.0.1".to_string()
        }
    };
    let port: u16 = 2121;

    //----- Conectarse al servidor ftp
    let mut ftp_stream = match FtpStream::connect(format!("{}:{}", ip, port)) {
        Ok(login_ok) => login_ok,
        Err(e) => return Err(format!("{}", e)),
    };

    assert!(ftp_stream.login("anonymous", "").is_ok());

    let mut load_file = match File::open(&file_path) {
        Ok(archivo) => archivo,
        Err(e) => return Err(format!("No se pudo abrir el archivo: {}", e)),
    };

    let file_name: String = Path::new(&file_path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    //----- añadir el documento a ftp_stream
    match ftp_stream.put_file(format!("{}", file_name), &mut load_file) {
        Ok(_) => {
            println!("Archivo enviado correctamente");
            let _ = ftp_stream.quit().map_err(|e| e.to_string());
        }
        Err(e) => println!("Error al subir el archivo: {}", e),
    };

    Ok(())
}
