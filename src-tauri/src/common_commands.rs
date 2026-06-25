use std::fs;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_opener::OpenerExt;

use crate::common_error::CommonError;

#[tauri::command]
pub async fn clear_storage<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    println!("Clearing storage!");
    // Get app data dir
    let app_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| "Failed to get app data directory".to_string())?;

    // Clear the directory
    fs::remove_dir_all(&app_data_dir).map_err(|e| format!("Failed to clear app data: {e}"))?;

    // Create the directory again
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to recreate app data dir: {e}"))?;

    // Restart the app
    app_handle.restart()
}

// Called when a download is finished, the user can ignore the modal or accept and open the file
#[tauri::command]
pub async fn user_download_action<R: Runtime>(
    app_handle: AppHandle<R>,
    filename: String,
) -> Result<(), CommonError> {
    // Validate filename, should not contain slashs and hidden file
    if filename.contains('/')
        || filename.contains('\\')
        || filename.contains("..")
        || filename.starts_with('.')
    {
        app_handle
            .dialog()
            .message("Nom de fichier invalide")
            .kind(MessageDialogKind::Error)
            .title("Erreur")
            .show(|_| {});
        return Ok(());
    }
    // Get the downloads directory
    let downloads_dir = match app_handle.path().download_dir() {
        Ok(dir) => dir,
        Err(_) => {
            app_handle
                .dialog()
                .message("Impossible d'accéder au dossier de téléchargements")
                .kind(MessageDialogKind::Error)
                .title("Erreur")
                .show(|_| {});
            return Ok(());
        }
    };

    // Construct the full path safely
    let full_path = downloads_dir.join(&filename);

    // Canonicalize and verify path containment (prevents symlink escapes)
    let canonical_full_path = match fs::canonicalize(&full_path) {
        Ok(path) => path,
        Err(_) => {
            app_handle
                .dialog()
                .message(format!(
                    "Impossible de résoudre le chemin du fichier: {}",
                    filename
                ))
                .kind(MessageDialogKind::Error)
                .title("Erreur")
                .show(|_| {});
            return Ok(());
        }
    };

    // Ensure the resolved path is still inside the downloads directory
    let canonical_downloads_dir = match fs::canonicalize(&downloads_dir) {
        Ok(dir) => dir,
        Err(_) => return Ok(()),
    };
    if !canonical_full_path.starts_with(&canonical_downloads_dir) {
        app_handle
            .dialog()
            .message("Le fichier se trouve en dehors du dossier de téléchargements")
            .kind(MessageDialogKind::Error)
            .title("Erreur")
            .show(|_| {});
        return Ok(());
    }

    if !canonical_full_path.is_file() {
        app_handle
            .dialog()
            .message("Le fichier n'existe pas")
            .kind(MessageDialogKind::Error)
            .title("Erreur")
            .show(|_| {});
        return Ok(());
    }

    let path_str = full_path.to_string_lossy().into_owned();
    println!("*** downloads_dir path_str {:?}", path_str);
    println!("*** in command user download action {:?}", filename);
    let message = format!("Voulez vous ouvrir le fichier {filename} ?");

    app_handle
        .dialog()
        .message(message)
        .kind(MessageDialogKind::Info)
        .title("Téléchargement réussi")
        .buttons(MessageDialogButtons::YesNo)
        .show(move |result| match result {
            true => {
                println!("in command user download action true");
                let _ = app_handle.opener().open_path(path_str, None::<&str>);
            }
            false => println!("in command user download action false"),
        });
    Ok(())
}

// Command to get or update user settings
#[tauri::command]
pub async fn settings_get_value<R: Runtime>(
    app_handle: AppHandle<R>,
    name: String,
) -> Result<String, CommonError> {
    println!("in get_setting {:?}", name);
    match name.as_str() {
        "Tauri.autoLaunch" => {
            let autostart_manager = app_handle.autolaunch();

            match autostart_manager.is_enabled() {
                Ok(enabled) => {
                    if enabled {
                        Ok("enabled".to_string())
                    } else {
                        Ok("disabled".to_string())
                    }
                }
                Err(e) => {
                    println!("Autolaunch error: {:?}", e);
                    Err(CommonError::String(format!(
                        "Failed to launch auto start: {e}"
                    ))) // adapt to your error type
                }
            }
        }
        _ => Ok("disabled".to_string()),
    }
}

// Command to get or update user settings
#[tauri::command]
pub async fn settings_set_value<R: Runtime>(
    app_handle: AppHandle<R>,
    name: String,
    value: String,
) -> Result<(), CommonError> {
    println!("in update_setting {:?}", name);
    if name.as_str() == "Tauri.autoLaunch" {
        let autostart_manager = app_handle.autolaunch();

        if value == "enabled" {
            autostart_manager
                .enable()
                .map_err(|e| CommonError::String(format!("Failed to launch auto start: {e}")))?;
        } else {
            autostart_manager
                .disable()
                .map_err(|e| CommonError::String(format!("Failed to launch auto start: {e}")))?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn can_self_update<R: Runtime>(_: AppHandle<R>) -> Result<bool, CommonError> {
    // Allow self update if auto updater version of the app
    if cfg!(not(feature = "no-updater")) {
        Ok(true)
    } else {
        Ok(false)
    }
}
