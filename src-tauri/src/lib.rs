mod common_commands;
mod common_error;
mod seshat_commands;
mod seshat_utils;

use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use blake2::{Blake2b512, Digest};
use rand::TryRngCore;
use seshat::Database;
use tauri::Manager;
use tauri_plugin_deep_link::DeepLinkExt;
use tauri::tray::TrayIconBuilder;
use tauri::menu::{MenuBuilder, MenuItemBuilder};

#[derive(Clone)]
pub struct MyState {
    pub database: Option<Arc<Mutex<Database>>>,
}

#[tauri::command]
fn welcome() {
    println!("Welcome on Tchap desktop app!")
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
    let mut builder = tauri::Builder::default();
    
    // Instanciate single instance plugin, with focus on the main window
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main")
                       .expect("no main window")
                       .set_focus();
        }));
    }

    builder
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
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
            app.deep_link().register("tchap")?;

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


            // Create the tray icon
            let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
            let hide = MenuItemBuilder::new("Hide").id("hide").build(app).unwrap();
            let show = MenuItemBuilder::new("Show").id("show").build(app).unwrap();

            let menu = MenuBuilder::new(app)
              .items(&[&quit, &hide, &show])
              .build()
              .unwrap();
           
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => app.exit(0),
                    "hide" => {
                      dbg!("menu item hide clicked");
                      let window = app.get_webview_window("main").unwrap();
                      window.hide().unwrap();
                    }
                    "show" => {
                      dbg!("menu item show clicked");
                      let window = app.get_webview_window("main").unwrap();
                      window.show().unwrap();
                    }
                    _ => {}
                  })
                .build(app)?;

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
            welcome
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
