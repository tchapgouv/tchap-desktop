mod common_commands;
mod common_error;
mod seshat_commands;
mod seshat_utils;

use std::sync::Mutex;
use seshat::Database;
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    utils::config::WebviewUrl,
    webview::{DownloadEvent, WebviewWindowBuilder},
    Manager,
    Emitter
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
        .plugin(tauri_plugin_keyring::init())
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

            // Create the initial state
            let initial_state = MyState { database: None };

            // Register it with Tauri's state management
            app.manage(Mutex::new(initial_state));

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                //focus on main window when clicking the tray icon
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
                .build(app)?;

            // Manually build windows, because on_download listenner can only be added on the build of the windows
            // Needs to remove app: {windows } from tauri conf, otherwise there will be two window creation
            let product_name = match app.config().product_name.as_deref() {
                Some("tchap_no_updater") => "Tchap".to_string(),
                Some(other) => other.to_string(),
                None => "Tchap".to_string(), // fallback if missing
              };

            let handle = app.app_handle().clone();
            let handle_for_on_download = app.app_handle().clone();
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
                .build()?;

            Ok(())
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
