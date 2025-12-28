use local_ip_address::local_ip;
use tauri::{command, Manager};
use unftp_sbe_fs::ServerExt;

//usar tokio aquí si se utiliza como código principa

#[command]
pub async fn ftp_server(app: tauri::AppHandle) -> Result<(), String> {
    //----- obtener la ruta de destino de archivos por defecto, en documentos/Remit
    let mut documents_dir: std::path::PathBuf =
        app.path().document_dir().map_err(|e| e.to_string())?;

    documents_dir.push("Remit");

    //----- Crear el directorio Remit en documentos si no existe
    if !documents_dir.exists() {
        let _ = std::fs::create_dir_all(&documents_dir);
        println!("Creando directorio: {}", format!("{:?}", &documents_dir));
    }

    //----- Obtener ip local
    let ip: String = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(e) => {
            println!("Error al obtener la IP: {}", e);
            "127.0.0.1".to_string()
        }
    };
    let port: u16 = 2121;

    //----- Encendido asíncrono del servidor ftp
    tauri::async_runtime::spawn(async move {
        let server = libunftp::Server::with_fs(documents_dir.clone())
            .greeting("Welcome to my FTP server")
            .passive_ports(50000..=65535)
            .build()
            .unwrap();

        println!("Servidor ftp iniciado en {}:{}", ip, port);
        println!(
            "Dirección de recepción de archivos: {}",
            &documents_dir.display()
        );
        server.listen(format!("{}:{}", ip, port)).await.unwrap();
    });

    Ok(())
}
