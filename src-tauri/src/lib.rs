use taurpc::{Router};

// mod common_commands;

// use crate::common_commands::CommonImpl;

#[taurpc::procedures(event_trigger = ApiEventTrigger, path = "common", export_to = "./bindings/bindings.ts")]
trait Common {
    async fn set_homeserver_url();
}

#[derive(Clone)]
pub struct CommonImpl;

#[taurpc::resolvers]
impl Common for CommonImpl {
    async fn set_homeserver_url(self) {
        println!("Hello world");
    }
}

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
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
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
