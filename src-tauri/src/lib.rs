use crate::action_handler::ActionHandler;
mod common_commands;
mod action_handler;

use std::{string::String, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::string::ToString;

use common_service::CommonService;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut handlers : dyn ActionHandler = HashMap::new();
    let service = CommonService{};
    handlers.insert(service.domain(), &service);

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::ipc_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn ipc_message(message: IpcMessage) -> IpcMessage {
    let message_handler = handlers.get(&*message.domain).unwrap(); 
    let response = message_handler.receive_action(message.action).unwrap();
    IpcMessage {
        domain: message_handler.domain().to_string(),
        action: response
    }
}


#[derive(Deserialize, Serialize)]
struct IpcMessage {
    domain: String,
    action: Value
} 