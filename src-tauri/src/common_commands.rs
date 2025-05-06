use std::fs;
use tauri::{AppHandle, Manager, Runtime};

#[tauri::command]
pub async fn clear_storage<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    println!("Clearing storage!");
    // Get app data dir
    let app_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| "Failed to get app data directory".to_string())?;

    // Clear the directory
    fs::remove_dir_all(&app_data_dir).map_err(|e| format!("Failed to clear app data: {}", e))?;

    // Create the directory again
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to recreate app data dir: {}", e))?;

    // Restart the app
    app_handle.restart()
}
