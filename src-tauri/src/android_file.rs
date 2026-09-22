use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, PluginHandle, TauriPlugin},
    AppHandle, Manager, Runtime,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DisplayNamePayload<'a> {
    uri: &'a str,
}

#[derive(Debug, Deserialize)]
struct DisplayNameResponse {
    name: Option<String>,
}

// Payload para pedirle al plugin Android que copie un archivo a la carpeta pública Documentos
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveToDocumentsPayload<'a> {
    file_path: &'a str,
    file_name: &'a str,
}

// Respuesta vacía esperada del plugin nativo (Kotlin resuelve con un JSObject vacío, no con unit)
#[derive(Debug, Deserialize)]
struct EmptyResponse {}

pub struct AndroidFile<R: Runtime>(PluginHandle<R>);

// Registra el plugin nativo de Android al iniciar la app
pub fn init<R: Runtime>() -> tauri::Result<TauriPlugin<R>> {
    Ok(Builder::new("android-file")
        .setup(|app, api| {
            let handle = api.register_android_plugin(
                "com.osxar.remit.file",
                "FilePlugin",
            )?;
            app.manage(AndroidFile(handle));
            Ok(())
        })
        .build())
}

// Pide al lado Kotlin el nombre real de un archivo a partir de su URI content://
pub fn display_name<R: Runtime>(
    app_handle: &AppHandle<R>,
    uri: &str,
) -> Result<String, String> {
    let plugin = app_handle.state::<AndroidFile<R>>();
    let response = plugin
        .0
        .run_mobile_plugin::<DisplayNameResponse>(
            "getDisplayName",
            DisplayNamePayload { uri },
        )
        .map_err(|e| e.to_string())?;

    response
        .name
        .filter(|name| !name.is_empty())
        .ok_or_else(|| "Android no devolvio el nombre del archivo".to_string())
}

// Pide al lado Kotlin que copie un archivo privado hacia la carpeta pública de Descargas (MediaStore)
pub fn save_to_documents<R: Runtime>(
    app_handle: &AppHandle<R>,
    file_path: &str,
    file_name: &str,
) -> Result<(), String> {
    let plugin = app_handle.state::<AndroidFile<R>>();
    plugin
        .0
        .run_mobile_plugin::<EmptyResponse>(
            "saveToDocuments",
            SaveToDocumentsPayload { file_path, file_name },
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_file_display_name<R: Runtime>(
    app_handle: AppHandle<R>,
    file_path: String,
) -> Result<String, String> {
    display_name(&app_handle, &file_path)
}