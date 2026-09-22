use local_ip_address::local_ip;
use tauri::{command, Manager};
use unftp_sbe_fs::ServerExt;

// Solo se usa en Android para llamar al plugin nativo que mueve archivos a Documentos
#[cfg(target_os = "android")]
use crate::android_file;

// Vigila la carpeta privada y mueve cada archivo nuevo a la carpeta pública de Documentos (solo Android)
#[cfg(target_os = "android")]
fn watch_and_move_to_documents(app: tauri::AppHandle, watch_dir: std::path::PathBuf) {
    use notify::{RecursiveMode, Watcher};

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        watcher.watch(&watch_dir, RecursiveMode::NonRecursive).unwrap();

        // Escucha eventos de creación de archivos en la carpeta vigilada
        for res in rx {
            if let Ok(event) = res {
                if let notify::EventKind::Create(_) = event.kind {
                    for path in event.paths {
                        if path.is_file() {
                            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                                let file_path = path.to_string_lossy().to_string();

                                // Copia el archivo a Documentos usando el plugin nativo
                                match android_file::save_to_documents(&app, &file_path, file_name) {
                                    Ok(_) => {
                                        println!("Archivo movido a Documentos: {}", file_name);
                                        // Borra el archivo temporal de la carpeta privada
                                        let _ = std::fs::remove_file(&path);
                                    }
                                    Err(e) => eprintln!("Error moviendo a Documentos: {}", e),
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

#[command]
pub async fn ftp_server(app: tauri::AppHandle) -> Result<(), String> {
    //----- obtener carpeta base de recepción de archivos según el sistema operativo

    // En Android se usa una carpeta privada (sin permisos especiales) y luego se mueve a Documentos
    #[cfg(target_os = "android")]
    let mut documents_dir: std::path::PathBuf =
        app.path().app_local_data_dir().map_err(|e| e.to_string())?;

    // En desktop se usa directamente la carpeta de Documentos del sistema
    #[cfg(not(target_os = "android"))]
    let mut documents_dir: std::path::PathBuf =
        app.path().document_dir().map_err(|e| e.to_string())?;

    documents_dir.push("Remit");

    //----- Crear el directorio Remit si no existe
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
    let port: u16 = 2001;

    // Clon del AppHandle para usarlo dentro del hilo del watcher (solo Android)
    #[cfg(target_os = "android")]
    let app_for_watch = app.clone();

    //----- Encendido asíncrono del servidor ftp
    tauri::async_runtime::spawn(async move {
        let server = libunftp::Server::with_fs(documents_dir.clone())
            .greeting("Welcome to my FTP server")
            .passive_ports(50000..=50010)
            .build()
            .unwrap();

        // Activa el watcher que mueve archivos recibidos a Documentos (solo Android)
        #[cfg(target_os = "android")]
        watch_and_move_to_documents(app_for_watch, documents_dir.clone());

        println!("Servidor ftp iniciado en {}:{}", ip, port);
        println!(
            "Dirección de recepción de archivos: {}",
            &documents_dir.display()
        );
        server.listen(format!("0.0.0.0:{}", port)).await.unwrap();
    });

    Ok(())
}