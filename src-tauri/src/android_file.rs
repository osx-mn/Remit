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

pub struct AndroidFile<R: Runtime>(PluginHandle<R>);

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

#[tauri::command]
pub fn get_file_display_name<R: Runtime>(
    app_handle: AppHandle<R>,
    file_path: String,
) -> Result<String, String> {
    display_name(&app_handle, &file_path)
}
