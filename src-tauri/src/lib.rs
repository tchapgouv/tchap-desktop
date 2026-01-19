mod common_commands;
mod common_error;
mod seshat_commands;
mod seshat_utils;

use std::fs;
use std::path::Path;
use std::sync::Mutex;

use blake2::{Blake2b512, Digest};
use rand::TryRngCore;
use seshat::Database;
use tauri::{
    Emitter, Manager,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    utils::config::WebviewUrl,
    webview::{DownloadEvent, WebviewWindowBuilder},
};

/// A state shared on Tauri.
#[derive(Clone)]
pub struct MyState {
    /// Seshat database.
    pub database: Option<Database>,
}

#[tauri::command]
fn welcome() {
    println!("Welcome on Tchap desktop app!")
}

fn user_agent(app: &tauri::AppHandle) -> String {
    let version = app.package_info().version.to_string();

    if cfg!(target_os = "windows") {
        format!("tchap-windows/{version} (Windows; Tauri)")
    } else if cfg!(target_os = "macos") {
        format!("tchap-macos/{version} (macOS; Tauri)")
    } else {
        format!("tchap-linux/{version} (Linux; Tauri)")
    }
}

fn get_or_create_salt(salt_path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if !salt_path.exists() {
        // Generate new salt
        let mut salt = vec![0u8; 32];
        let mut rng = rand::rngs::OsRng;
        rng.try_fill_bytes(&mut salt)?;
        // Ensure directory exists
        fs::create_dir_all(salt_path.parent().unwrap())?;
        // Write salt to file
        fs::write(salt_path, &salt)?;
        Ok(salt)
    } else {
        // Read existing salt
        Ok(fs::read(salt_path)?)
    }
}

fn create_stronghold_key(app: &tauri::AppHandle, password: &[u8]) -> Vec<u8> {
    let salt_path = app
        .path()
        .app_local_data_dir()
        .expect("could not resolve app local data path")
        .join("salt.txt");

    let salt = get_or_create_salt(&salt_path).unwrap();

    // Looks like hashing blake2b512 is faster than argo, so we use blake here
    // TODO : will need to check if it was the real issue when loading stronghold took long time
    let mut hasher = Blake2b512::new();
    hasher.update(salt);
    hasher.update(password);
    hasher.finalize().to_vec()[..32].to_vec()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_shell::init());

    // Instanciate single instance plugin, with focus on the main window
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }
    // doessnt initialize the updater plugin if the feature no-updater is applied during build

    #[cfg(not(feature = "no-updater"))]
    {
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    }

    builder
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Removing deeplink registration on macos for now, since it's not working and throwing an error on build
            // https://github.com/tchapgouv/tchap-desktop/issues/44
            #[cfg(all(desktop, not(target_os = "macos")))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                app.deep_link().register("tchap")?;
            }

            // Registerering stronghold plugin
            let app_handle = app.app_handle().clone();
            // Convert to Vec<u8> for Stronghold
            app.handle().plugin(
                tauri_plugin_stronghold::Builder::new(move |password| {
                    create_stronghold_key(&app_handle, password.as_ref())
                })
                .build(),
            )?;

            // Create the initial state
            let initial_state = MyState { database: None };

            // Register it with Tauri's state management
            app.manage(Mutex::new(initial_state));

            let show_hide =
                MenuItem::with_id(app, "show_hide", "Montrer / Cacher", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_hide, &quit])?;
            TrayIconBuilder::new()
                .show_menu_on_left_click(false)
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                // display or hide the app on left click in the tray icon
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.unminimize();
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    }
                })
                // Display the menu only on right click on the tray icon, not supported on linux
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show_hide" => {
                        if let Some(webview_window) = app.get_webview_window("main") {
                            if webview_window.is_visible().unwrap() {
                                webview_window.hide().unwrap();
                            } else {
                                let _ = webview_window.unminimize();
                                let _ = webview_window.set_focus();
                                let _ = webview_window.show();
                            }
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // Manually build windows, because on_download listenner can only be added on the build of the windows
            let product_name = match app.config().product_name.as_deref() {
                Some(other) => other.to_string(),
                None => "Tchap".to_string(), // fallback if missing
            };

            let handle = app.app_handle().clone();
            let handle_for_on_download = app.app_handle().clone();
            // Needs to remove app: {windows } from tauri conf, otherwise there will be two window creation
            WebviewWindowBuilder::new(&handle, "main", WebviewUrl::App("index.html".into()))
                .on_download(move |_webview, event| {
                    if let DownloadEvent::Finished { url, path, success } = event {
                        println!("downloaded {} to {:?}, success: {}", url, path, success);
                        let _ = handle_for_on_download.emit("download-finished", &path);
                    }
                    // let the download start
                    true
                })
                .disable_drag_drop_handler()
                .title(product_name)
                .user_agent(&user_agent(&handle))
                .inner_size(1000.0, 1000.0)
                .build()?;

            Ok(())
        })
        // When closing the app with the cross button, we only hide the app, and dont close it completly
        // The same behavior as Element app and apps on MacOS
        .on_window_event(|window, event| if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            window.hide().unwrap();
            api.prevent_close();
        })
        .invoke_handler(tauri::generate_handler![
            seshat_commands::supports_event_indexing,
            seshat_commands::init_event_index,
            seshat_commands::close_event_index,
            seshat_commands::delete_event_index,
            seshat_commands::add_event_to_index,
            seshat_commands::delete_event,
            seshat_commands::is_event_index_empty,
            seshat_commands::is_room_indexed,
            seshat_commands::commit_live_events,
            seshat_commands::search_event_index,
            seshat_commands::add_historic_events,
            seshat_commands::add_crawler_checkpoint,
            seshat_commands::remove_crawler_checkpoint,
            seshat_commands::load_file_events,
            seshat_commands::load_checkpoints,
            seshat_commands::get_stats,
            seshat_commands::set_user_version,
            seshat_commands::get_user_version,
            common_commands::clear_storage,
            common_commands::user_download_action,
            welcome
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
