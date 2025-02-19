
// use std::fs;
// use std::path::Path;
// use std::sync::{Arc, Mutex};
// use tokio::sync::oneshot;
// use tokio::task;
// use serde_json::Value;
// use async_trait::async_trait;
use std::{sync::{Arc, Mutex}, thread};
use lazy_static::lazy_static;
use seshat::{Database, Config};
use tauri::{AppHandle, Manager, Runtime};
use std::fs;

// lazy_static! {
//     static ref DATABASE: Mutex<Option<Database>>> = Mutex::new(None);
// }

// fn set_seshat_supported(supported: bool) {
//     let mut seshat_supported = SESHAT_SUPPORTED.lock().unwrap();
//     *seshat_supported = supported;
// }

// making the exported binding outside of src-tauri, otherwise tauri dev will make infinite loop
#[taurpc::procedures(event_trigger = ApiEventTrigger, path = "seshat", export_to = "../bindings/bindings.ts")]
pub trait TchapSeshat {
    async fn supportsEventIndexing() -> bool;
    async fn initEventIndex<R: Runtime>(app_handle: AppHandle<R>, passphrase: String);
}

#[derive(Clone)]
pub struct TchapSeshatImpl {
    database: Arc<Mutex<Database>>,
}

#[taurpc::resolvers]
impl TchapSeshat for TchapSeshatImpl {
    async fn supportsEventIndexing(self) -> bool{
        println!("Supports event indexing");
        true
    }

    async fn initEventIndex<R: Runtime>(self, app_handle: AppHandle<R>, passphrase: String) {

        let config = Config::new().set_passphrase(passphrase);

        let db_path = app_handle.path()
            .app_local_data_dir()
            .expect("could not resolve app local data path")
            .join("seshat_db");

        fs::create_dir_all(&db_path);
        let database = Database::new_with_config(&db_path, &config).unwrap();

        // Clone Arc to share ownership across threads
        let database_clone = Arc::clone(&self.database);

        // Spawn a new thread to update the variable
        let handle = thread::spawn(move || {
            let mut db = database_clone.lock().unwrap(); // Lock the mutex
            *db = database;
        });

        handle.join().unwrap();

    }
}

