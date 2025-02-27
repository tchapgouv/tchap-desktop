
use std::sync::{Arc, Mutex};
use seshat::{Config, CrawlerCheckpoint, Database, DatabaseStats, Event, LoadConfig, Profile, SearchBatch, SearchConfig};
use tauri::{AppHandle, Manager, Runtime, State};
use std::fs;
use std::sync::mpsc;

use crate::common_error::CommonError as Error;
use crate::MyState;


#[tauri::command]
pub async fn supports_event_indexing() -> bool{
    println!("Supports event indexing");
    true
}

#[tauri::command]
pub async fn init_event_index<R: Runtime>(app_handle: AppHandle<R>, state: State<'_, Mutex<MyState>>, passphrase: String) -> Result<(), Error> {

    let config = Config::new().set_passphrase(passphrase);

    // The app_handle is a method introduce by tauri
    let db_path = app_handle.path()
        .app_local_data_dir()
        .expect("could not resolve app local data path")
        .join("seshat_db");

    let _ = fs::create_dir_all(&db_path);
    let database = Arc::new(Mutex::new(Database::new_with_config(&db_path, &config).unwrap()));

    // Store the new database
    let mut state = state.lock().unwrap();
    state.database = Some(Arc::clone(&database));
    Ok(())
}

#[tauri::command]
// Closing the database
pub async fn close_event_index(state: State<'_, Mutex<MyState>>)-> Result<(), Error> {
    
    let mut state = state.lock().unwrap();
    if let Some(db) = state.database.take() {
        match Arc::try_unwrap(db) {
            Ok(mutex) => {
                let db_inner = mutex.into_inner().unwrap(); // Extract the database
                // The shutdown meethod needs to take ownership 
                db_inner.shutdown(); 
                // set the database to none
                state.database = None;
                Ok(())
            }
            Err(_arc) => {
                println!("Failed to take ownership: Database is still shared.");
                state.database = Some(_arc); // Restore the database if shutdown failed
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}

#[tauri::command]
// Deleting the database contents and files
pub async fn delete_event_index(state: State<'_, Mutex<MyState>>) -> Result<(), Error> {
    let mut state = state.lock().unwrap();
    if let Some(db) = state.database.take() {
        match Arc::try_unwrap(db) {
            Ok(mutex) => {
                let db_inner = mutex.into_inner().unwrap(); // Extract the database
                // The delete meethod needs to take ownership 
                db_inner.delete(); 
                // set the database to none
                state.database = None;
                Ok(())
            }
            Err(_arc) => {
                println!("Failed to take ownership: Database is still shared.");
                state.database = Some(_arc); // Restore the database if shutdown failed
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}

#[tauri::command]
pub async fn add_event_to_index(state: State<'_, MyState>, event: Event, profile: Profile) -> Result<(), Error> {
    if let Some(ref db) = state.database {
        let db_lock = db.lock().unwrap();
        db_lock.add_event(event, profile);
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_event(state: State<'_, MyState>, event_id: String) -> Result<(), Error>{
    if let Some(ref db) = state.database {
        let db_lock = db.lock().unwrap();
        db_lock.delete_event(&event_id);
    }
    Ok(())
}

#[tauri::command]
pub async fn commit_live_events(state: State<'_, MyState>) -> Result<(), Error> {
    if let Some(ref db) = state.database {
        let mut db_lock = db.lock().unwrap();
        db_lock.commit();
    }
    Ok(())
}

#[tauri::command]
pub async fn search_event_index(state: State<'_, MyState>, term: String) -> Result<SearchBatch, Error> {
    if let Some(ref db) = state.database {
        let search_config = SearchConfig::new();
        let db_lock = db.lock().unwrap();
        db_lock
            .search(&term, &search_config)
            .map_err(|e| Error::from(e))
    } else {
        Ok(SearchBatch::default())
    }
}

#[tauri::command]
pub async fn is_room_indexed(state: State<'_, MyState>, room_id: String) -> Result<bool, Error> {
    if let Some(ref db) = state.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection
            .is_room_indexed(&room_id)
            .map_err(|e| Error::from(e))
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn is_event_index_empty(state: State<'_, MyState>) -> Result<bool, Error>{
    if let Some(ref db) = state.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection
            .is_empty()
            .map_err(|e| Error::from(e))
    } else {
        Ok(true)
    }
}

#[tauri::command]
pub async fn add_historic_events(state: State<'_, MyState>, events: Vec<(Event, Profile)>, new_checkpoint: Option<CrawlerCheckpoint>, old_checkpoint: Option<CrawlerCheckpoint>) -> Result<bool, Error> {
    if let Some(ref db) = state.database {
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

#[tauri::command]
pub async fn get_stats(state: State<'_, MyState>) -> Result<DatabaseStats, Error> {
    if let Some(ref db) = state.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection.get_stats().map_err(|e| Error::from(e))
    } else {
        Err(Error::Unknown)
    }
}

#[tauri::command]
// There is no remove_crawler_checkpoint in the api, but we are only useing add_historic_events with the correct parameters
pub async fn remove_crawler_checkpoint(state: State<'_, MyState>, checkpoint: CrawlerCheckpoint)-> Result<bool, Error> {
    if let Some(ref db) = state.database {
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

#[tauri::command]
// There is no add_crawler_checkpoint in the api, but we are only useing add_historic_events with the correct parameters
pub async fn add_crawler_checkpoint(state: State<'_, MyState>, checkpoint: CrawlerCheckpoint)-> Result<bool, Error> {
    if let Some(ref db) = state.database {
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

#[tauri::command]
pub async fn load_file_events(state: State<'_, MyState>, load_config: LoadConfig) -> Result<Vec<(String, Profile)>, Error> {
    if let Some(ref db) = state.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection
            .load_file_events(&load_config)
            .map_err(|e| Error::from(e))
    } else {
        Err(Error::Unknown)
    } 
}

#[tauri::command]
pub async fn load_checkpoints(state: State<'_, MyState>) -> Result<Vec<CrawlerCheckpoint>, Error>{
    if let Some(ref db) = state.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection.load_checkpoints().map_err(|e| Error::from(e))
    } else {
        Err(Error::Unknown)
    }
}

#[tauri::command]
pub async fn set_user_version(state: State<'_, MyState>, version: i64) -> Result<(), Error>{
    if let Some(ref db) = state.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection
            .set_user_version(version)
            .map_err(|e| Error::from(e))
    } else {
        Ok(())
    }
}

#[tauri::command]
pub async fn get_user_version(state: State<'_, MyState>) -> Result<i64, Error>{
    if let Some(ref db) = state.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection
            .get_user_version()
            .map_err(|e| Error::from(e))
    } else {
        Ok(0)
    }
}