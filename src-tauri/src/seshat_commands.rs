
use std::sync::{Arc, Mutex};
use seshat::{Config, Database, Error, Event, Profile, SearchBatch, SearchConfig};
use tauri::{AppHandle, Manager, Runtime};
use std::fs;

// making the exported binding outside of src-tauri, otherwise tauri dev will make infinite loop
#[taurpc::procedures(path = "seshat", export_to = "../bindings/seshat.ts")]
pub trait TchapSeshat {
    async fn supports_event_indexing() -> bool;
    async fn init_event_index<R: Runtime>(app_handle: AppHandle<R>, passphrase: String);
    async fn close_event_index();
    async fn delete_event_index();
    async fn add_event_to_index(event: Event, profile: Profile);
    async fn delete_event(event_id: String);
    async fn is_event_index_empty();
    async fn is_room_indexed(room_id: String);
    async fn commit_live_events();
    async fn search_event_index(term: String);
    async fn add_historic_events();
    async fn add_crawler_checkpoint();
    async fn remove_crawler_checkpoint();
    async fn load_file_events();
    async fn load_checkpoints();
    async fn get_stats();
    async fn set_user_version();
}

#[derive(Clone)]
pub struct TchapSeshatImpl {
    pub database: Option<Arc<Mutex<Database>>>,
}

#[taurpc::resolvers]
impl TchapSeshat for TchapSeshatImpl {
    async fn supports_event_indexing(self) -> bool{
        println!("Supports event indexing");
        true
    }

    async fn init_event_index<R: Runtime>(mut self, app_handle: AppHandle<R>, passphrase: String) {

        let config = Config::new().set_passphrase(passphrase);

        let db_path = app_handle.path()
            .app_local_data_dir()
            .expect("could not resolve app local data path")
            .join("seshat_db");

        let _ = fs::create_dir_all(&db_path);
        let database = Arc::new(Mutex::new(Database::new_with_config(&db_path, &config).unwrap()));

        // Store the new database
        self.database = Some(Arc::clone(&database));
    }

    // Closing the database
    async fn close_event_index(mut self) {
        
        if let Some(db) = self.database.take() {
            match Arc::try_unwrap(db) {
                Ok(mutex) => {
                    let db_inner = mutex.into_inner().unwrap(); // Extract the database
                    // The shutdown meethod needs to take ownership 
                    db_inner.shutdown(); 
                    // set the database to none
                    self.database = None;
                }
                Err(_arc) => {
                    println!("Failed to take ownership: Database is still shared.");
                    self.database = Some(_arc); // Restore the database if shutdown failed
                }
            }
        }
    }

    // Deleting the database contents and files
    async fn delete_event_index(self) {

    }

    async fn add_event_to_index(self, event: Event, profile: Profile) {
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            db_lock.add_event(event, profile);
        }
    }

    async fn delete_event(self, event_id: String) {
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            db_lock.delete_event(&event_id);
        }
    }

    async fn commit_live_events(self) {
        if let Some(ref db) = self.database {
            let mut db_lock = db.lock().unwrap();
            db_lock.commit();
        }
    }

    async fn search_event_index(self, term: String) -> Result<SearchBatch, Error> {
        if let Some(ref db) = self.database {
            let search_config = SearchConfig::new();
            let db_lock = db.lock().unwrap();
            db_lock.search(&term, &search_config)
        } else {
            Ok(SearchBatch::default())
        }
    }

    async fn is_room_indexed(self, room_id: String) -> Result<bool, Error> {
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let connection = db_lock.get_connection().unwrap();
            connection.is_room_indexed(&room_id)
        } else {
            Ok(false)
        }
    }
}

