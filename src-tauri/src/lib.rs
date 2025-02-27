mod seshat_commands;
mod common_error;

use std::sync::{Arc, Mutex};


use seshat::Database;
use tauri::Manager;



#[derive(Clone)]
pub struct MyState {
    pub database: Option<Arc<Mutex<Database>>>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test() -> String {
    format!("Hello, just test! You've been greeted from Rust!")
}

#[tauri::command]
fn test_not_async() {
    println!("Hello You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // configure stronghold
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            Ok(())
        })
        .manage(MyState {
            database: None,
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
            greet,
            test,
            test_not_async])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
