
use std::sync::{Arc, Mutex};
use seshat::{Config, CrawlerCheckpoint, Database, DatabaseStats, Event, LoadConfig, Profile, SearchBatch, SearchConfig};
use tauri::{AppHandle, Manager, Runtime};
use std::fs;
use std::sync::mpsc;

use crate::common_error::CommonError as Error;



#[derive(Type)]
struct MyEvent(Event);
#[taurpc::ipc_type]
struct MyProfile(Profile);
#[taurpc::ipc_type]
struct MyCrawlerCheckpoint(CrawlerCheckpoint);
#[taurpc::ipc_type]
struct MyLoadConfig(LoadConfig);
#[taurpc::ipc_type]
struct MyDatabaseStats(DatabaseStats);

#[taurpc::ipc_type]
struct MySearchBatch(SearchBatch);


// making the exported binding outside of src-tauri, otherwise tauri dev will make infinite loop
#[taurpc::procedures(path = "seshat", export_to = "../bindings/seshat.ts")]
pub trait TchapSeshat {
    async fn supports_event_indexing() -> bool;
    async fn init_event_index<R: Runtime>(app_handle: AppHandle<R>, passphrase: String);
    async fn close_event_index();
    async fn delete_event_index();
    async fn add_event_to_index(event: MyEvent, profile: MyProfile);
    async fn delete_event(event_id: String);
    async fn is_event_index_empty() -> Result<bool, Error>;
    async fn is_room_indexed(room_id: String) -> Result<bool, Error>;
    async fn commit_live_events();
    async fn search_event_index(term: String) -> Result<MySearchBatch, Error>;
    async fn add_historic_events(events: Vec<(MyEvent, MyProfile)>, new_checkpoint: Option<MyCrawlerCheckpoint>, old_checkpoint: Option<MyCrawlerCheckpoint>) -> Result<bool, Error>;
    async fn add_crawler_checkpoint(checkpoint: MyCrawlerCheckpoint) -> Result<bool, Error>;
    async fn remove_crawler_checkpoint(checkpoint: MyCrawlerCheckpoint) -> Result<bool, Error>;
    async fn load_file_events(load_config: MyLoadConfig) -> Option<Result<Vec<(String, MyProfile)>, Error>>;
    async fn load_checkpoints() -> Option<Result<Vec<MyCrawlerCheckpoint>, Error>>;
    async fn get_stats() -> Option<Result<MyDatabaseStats, Error>>;
    async fn set_user_version(version: i64) -> Result<(), Error>;
    async fn get_user_version() -> Result<i64, Error>;
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

        // The app_handle is a method introduce by tauri
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
    async fn delete_event_index(mut self) {
        if let Some(db) = self.database.take() {
            match Arc::try_unwrap(db) {
                Ok(mutex) => {
                    let db_inner = mutex.into_inner().unwrap(); // Extract the database
                    // The delete meethod needs to take ownership 
                    db_inner.delete(); 
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

    async fn add_event_to_index(self, event: MyEvent, profile: MyProfile) {
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

    async fn search_event_index(self, term: String) -> Result<MySearchBatch, Error> {
        if let Some(ref db) = self.database {
            let search_config = SearchConfig::new();
            let db_lock = db.lock().unwrap();
            db_lock
                .search(&term, &search_config)
                .map_err(|e| Error::from(e))
        } else {
            Ok(SearchBatch::default())
        }
    }

    async fn is_room_indexed(self, room_id: String) -> Result<bool, Error> {
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let connection = db_lock.get_connection().unwrap();
            connection
                .is_room_indexed(&room_id)
                .map_err(|e| Error::from(e))
        } else {
            Ok(false)
        }
    }

    async fn is_event_index_empty(self) -> Result<bool, Error>{
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let connection = db_lock.get_connection().unwrap();
            connection
                .is_empty()
                .map_err(|e| Error::from(e))
        } else {
            Ok(true)
        }
    }
    
    async fn add_historic_events(self, events: Vec<(MyEvent, MyProfile)>, new_checkpoint: Option<MyCrawlerCheckpoint>, old_checkpoint: Option<MyCrawlerCheckpoint>) -> Result<bool, Error> {
        if let Some(ref db) = self.database {
            let db_lock: std::sync::MutexGuard<'_, Database> = db.lock().unwrap();
            let receiver = db_lock.add_historic_events(events, new_checkpoint, old_checkpoint);

            receiver
                .recv()
                .map(|r| r.map_err(|e| Error::from(e)))
                .map_err(|recv_err| Error::from(recv_err))
                .unwrap()

        } else {
             // Create a dummy channel to return the expected type
            let (tx, rx) = mpsc::channel();
            let _ = tx.send(Ok(false));

            rx.recv()
                .map_err(|recv_err| Error::from(recv_err))
                .unwrap()
        }
    }

    async fn get_stats(self) -> Option<Result<MyDatabaseStats, Error>> {
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let connection = db_lock.get_connection().unwrap();
            Some(connection.get_stats().map_err(|e| Error::from(e)))
        } else {
            None
        } 
    }
    
    // There is no remove_crawler_checkpoint in the api, but we are only useing add_historic_events with the correct parameters
    async fn remove_crawler_checkpoint(self, checkpoint: MyCrawlerCheckpoint)-> Result<bool, Error> {
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let receiver = db_lock.add_historic_events(Vec::new(), None, Some(checkpoint));

            receiver
                .recv()
                .map(|r| r.map_err(|e| Error::from(e)))
                .map_err(|recv_err| Error::from(recv_err))
                .unwrap()
        } else {
            // Create a dummy channel to return the expected type
            let (tx, rx) = mpsc::channel();
            let _ = tx.send(Ok(false));

            rx
                .recv()
                .map_err(|recv_err| Error::from(recv_err))
                .unwrap()
        }
    }

    // There is no add_crawler_checkpoint in the api, but we are only useing add_historic_events with the correct parameters
    async fn add_crawler_checkpoint(self, checkpoint: MyCrawlerCheckpoint)-> Result<bool, Error> {
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let receiver = db_lock.add_historic_events(Vec::new(), Some(checkpoint), None);

            receiver
                .recv()
                .map(|r| r.map_err(|e| Error::from(e)))
                .map_err(|recv_err| Error::from(recv_err))
                .unwrap()
        } else {
            // Create a dummy channel to return the expected type
            let (tx, rx) = mpsc::channel();
            let _ = tx.send(Ok(false));

            rx
                .recv()
                .map_err(|recv_err| Error::from(recv_err))
                .unwrap()
        }
    }

    async fn load_file_events(self, load_config: MyLoadConfig) -> Option<Result<Vec<(String, MyProfile)>, Error>> {
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let connection = db_lock.get_connection().unwrap();
            Some(
                connection
                    .load_file_events(&load_config.0)
                    .map_err(|e| Error::from(e))
                    .iter()
                    .map(|(key, value)| (key, MyProfile(value)))
            )
        } else {
            None
        } 
    }

    async fn load_checkpoints(self) -> Option<Result<Vec<CrawlerCheckpoint>, Error>>{
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let connection = db_lock.get_connection().unwrap();
            Some(connection.load_checkpoints().map_err(|e| Error::from(e)))
        } else {
            None
        }
    }

    async fn set_user_version(self, version: i64) -> Result<(), Error>{
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let connection = db_lock.get_connection().unwrap();
            connection
                .set_user_version(version)
                .map_err(|e| Error::from(e))
        } else {
            Ok(())
        }
    }

    async fn get_user_version(self) -> Result<i64, Error>{
        if let Some(ref db) = self.database {
            let db_lock = db.lock().unwrap();
            let connection = db_lock.get_connection().unwrap();
            connection
                .get_user_version()
                .map_err(|e| Error::from(e))
        } else {
            Ok(0)
        }
    }

}

