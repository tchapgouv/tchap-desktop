use taurpc::Router;
use tauri::Manager;

mod common_commands;
mod seshat_commands;

use crate::common_commands::CommonImpl;
use crate::common_commands::Common;



#[tokio::main]
pub async fn run() {
    let router = Router::new().merge(CommonImpl.into_handler());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // configure stronghold
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            // initialize seshat
            crate::seshat_commands::init_seshat();
            Ok(())
        })
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
