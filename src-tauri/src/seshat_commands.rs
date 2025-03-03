
use std::sync::{Arc, Mutex};
use seshat::{Config, CrawlerCheckpoint, Database, DatabaseStats, Event, LoadConfig, Profile, SearchBatch, SearchConfig};
use tauri::{AppHandle, Manager, Runtime, State};
use std::fs;
use std::sync::mpsc;

use crate::common_error::CommonError as Error;
use crate::MyState;


#[tauri::command]
pub async fn supports_event_indexing() -> bool{
    println!("[Command] Supports event indexing");
    true
}

#[tauri::command]
pub async fn init_event_index<R: Runtime>(app_handle: AppHandle<R>, state: State<'_, Mutex<MyState>>, passphrase: String) -> Result<(), Error> {
    println!("[Command] init_event_index");
    println!("[Command] init_event_index passphrase ${:?}", passphrase);
    let config = Config::new().set_passphrase(passphrase);

    let mut state_lock = state.lock().unwrap();
    
    // Check if the database is already initialized
    if state_lock.database.is_some() {
        println!("[Command] Database is already initialized.");
        return Ok(()); // No need to reinitialize
    }


    // The app_handle is a method introduce by tauri
    let db_path = app_handle.path()
        .app_local_data_dir()
        .expect("could not resolve app local data path")
        .join("seshat_db");

    let _ = fs::create_dir_all(&db_path);
    let database = Arc::new(Mutex::new(Database::new_with_config(&db_path, &config).unwrap()));

    // Store the new database
    state_lock.database = Some(Arc::clone(&database));
    Ok(())
}

#[tauri::command]
// Closing the database
pub async fn close_event_index(state: State<'_, Mutex<MyState>>)-> Result<(), Error> {
    println!("[Command] close_event_index");
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
        println!("[Command] close_event_index no db found, already closed");
        Ok(())
    }
}


#[tauri::command]
// Deleting the database contents and files
pub async fn delete_event_index<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), Error> {
    println!("[Command] delete_event_index");
        // The app_handle is a method introduce by tauri
    let db_path = app_handle.path()
        .app_local_data_dir()
        .expect("could not resolve app local data path")
        .join("seshat_db");

        // Handle the case where the directory doesn't exist
        match fs::remove_dir_all(&db_path) {
            Ok(_) => println!("Successfully deleted index at: {:?}", db_path),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("Index directory not found at: {:?}, continuing anyway", db_path);
            },
            Err(e) => return Err(e.into()), // For other errors, convert and return
        }
        
        Ok(())
}

#[tauri::command]
pub async fn add_event_to_index(state: State<'_, Mutex<MyState>>, event: Event, profile: Profile) -> Result<(), Error> {
    println!("[Command] add_event_to_index");
    println!("[Command] add_event_to_index event {:?}", event);
    println!("[Command] add_event_to_index profile {:?}", profile);
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        db_lock.add_event(event, profile);
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_event(state: State<'_, Mutex<MyState>>, event_id: String) -> Result<(), Error>{
    println!("[Command] delete_event {:?}", event_id);
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        db_lock.delete_event(&event_id);
    }
    Ok(())
}

#[tauri::command]
pub async fn commit_live_events(state: State<'_, Mutex<MyState>>) -> Result<(), Error> {
    println!("[Command] commit_live_events");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let mut db_lock = db.lock().unwrap();
        db_lock.commit();
    }
    Ok(())
}

#[tauri::command]
pub async fn search_event_index(state: State<'_, Mutex<MyState>>, term: String, search_config: SearchConfig) -> Result<SearchBatch, Error> {
    println!("[Command] search_event_index {:?}", term);
    let state_guard = state.lock().unwrap();
    
    if let Some(ref db) = state_guard.database {
        let config = search_config.clone();
        let db_lock = db.lock().unwrap();
        let result = db_lock
            .search(&term, &config)
            .map_err(|e| Error::from(e));
    
        println!("[Command] search_event_index result {:?}", result);  
        result
    } else {
        println!("[Command] search_event_index result no database found");  
        Ok(SearchBatch::default())
    }
}

#[tauri::command]
pub async fn is_room_indexed(state: State<'_, Mutex<MyState>>, room_id: String) -> Result<bool, Error> {
    println!("[Command] is_room_indexed");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
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
pub async fn is_event_index_empty(state: State<'_, Mutex<MyState>>) -> Result<bool, Error>{
    println!("[Command] is_event_index_empty");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        let result = connection
            .is_empty()
            .map_err(|e| Error::from(e));
        
        println!("[Command] is_event_index_empty {:?}", result);
        result
    } else {
        println!("[Command] is_event_index_empty true");
        Ok(true)
    }
}

#[tauri::command]
pub async fn add_historic_events(state: State<'_, Mutex<MyState>>, events: Vec<(Event, Profile)>, new_checkpoint: Option<CrawlerCheckpoint>, old_checkpoint: Option<CrawlerCheckpoint>) -> Result<bool, Error> {
    println!("[Command] add_historic_events");
    println!("[Command] add_historic_events args events ${:?}", events);
    println!("[Command] add_historic_events args newcheckpoint ${:?}", new_checkpoint);
    println!("[Command] add_historic_events args oldcheckpoint ${:?}", old_checkpoint);
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
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
pub async fn get_stats(state: State<'_, Mutex<MyState>>) -> Result<DatabaseStats, Error> {
    println!("[Command] remove_crawler_checkpoint");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection.get_stats().map_err(|e| Error::from(e))
    } else {
        Err(Error::Unknown)
    }
}

#[tauri::command]
// There is no remove_crawler_checkpoint in the api, but we are only useing add_historic_events with the correct parameters
pub async fn remove_crawler_checkpoint(state: State<'_, Mutex<MyState>>, checkpoint: CrawlerCheckpoint)-> Result<bool, Error> {
    println!("[Command] remove_crawler_checkpoint");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
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
pub async fn add_crawler_checkpoint(state: State<'_, Mutex<MyState>>, checkpoint: CrawlerCheckpoint)-> Result<bool, Error> {
    println!("[Command] add_crawler_checkpoint ${:?}", checkpoint);
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
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
pub async fn load_file_events(state: State<'_, Mutex<MyState>>, load_config: LoadConfig) -> Result<Vec<(String, Profile)>, Error> {
    println!("[Command] load_file_events");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
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
pub async fn load_checkpoints(state: State<'_, Mutex<MyState>>) -> Result<Vec<CrawlerCheckpoint>, Error>{
    println!("[Command] load_checkpoints");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection.load_checkpoints().map_err(|e| Error::from(e))
    } else {
        Err(Error::Unknown)
    }
}

#[tauri::command]
pub async fn set_user_version(state: State<'_, Mutex<MyState>>, version: i64) -> Result<(), Error>{
    println!("[Command] set_user_version");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
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
pub async fn get_user_version(state: State<'_, Mutex<MyState>>) -> Result<i64, Error>{
    println!("[Command] get_user_version");
    let state_guard = state.lock().unwrap();
    
    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection
            .get_user_version()
            .map_err(|e| Error::from(e))
    } else {
        Ok(0)
    }
}