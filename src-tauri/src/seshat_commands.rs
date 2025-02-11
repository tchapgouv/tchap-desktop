
// use std::fs;
// use std::path::Path;
// use std::sync::{Arc, Mutex};
// use tokio::sync::oneshot;
// use tokio::task;
// use serde_json::Value;
// use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use seshat::{Event};

lazy_static! {
    static ref SESHAT_SUPPORTED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref EVENT_INDEX: Arc<Mutex<Option<Event>>> = Arc::new(Mutex::new(None));
}

pub fn init_seshat() {
    set_seshat_supported(true);
}

fn is_seshat_supported() -> bool {
    let supported = SESHAT_SUPPORTED.lock().unwrap();
    *supported
}

fn set_seshat_supported(supported: bool) {
    let mut seshat_supported = SESHAT_SUPPORTED.lock().unwrap();
    *seshat_supported = supported;
}

fn get_or_create_passphrase(key: String) {

}

// making the exported binding outside of src-tauri, otherwise tauri dev will make infinite loop
#[taurpc::procedures(event_trigger = ApiEventTrigger, path = "seshat", export_to = "../bindings/bindings.ts")]
pub trait TchapSeshat {
    async fn supportsEventIndexing();
    async fn initEventIndex(_user_id: String, _device_id: String);
}

#[derive(Clone)]
pub struct SeshatImpl;

#[taurpc::resolvers]
impl TchapSeshat for SeshatImpl {
    async fn supportsEventIndexing(self) {
        println!("Supports event indexing");
        set_seshat_supported(true);
    }

    async fn initEventIndex(self, _user_id: String, _device_id: String) {

    }
}