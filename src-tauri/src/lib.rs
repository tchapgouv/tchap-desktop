mod seshat_commands;
mod common_commands;
mod common_error;

use taurpc::Router;
use tauri::Manager;

use common_commands::{Common, CommonImpl};
use seshat_commands::{TchapSeshatImpl, TchapSeshat};


#[tokio::main]
pub async fn run() {
    let router = Router::new()
        .merge(CommonImpl.into_handler());

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

            Ok(())
        })
        .invoke_handler(router.into_handler())
        // we dont use the merge in the router like common commands because we have a state to initialize, the db
        .invoke_handler(taurpc::create_ipc_handler(
            TchapSeshatImpl {
                database: None,
            }.into_handler(),
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
