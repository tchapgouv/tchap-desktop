
use std::sync::{Arc, Mutex};
use serde_json::Value;
use seshat::{Config, CrawlerCheckpoint, Database, DatabaseStats, LoadConfig, LoadDirection, Profile};
use tauri::{AppHandle, Manager, Runtime, State};
use std::fs;
use std::sync::mpsc;

use crate::common_error::CommonError;
use crate::seshat_utils::{add_historic_events_helper, deserialize_event, parse_event, parse_profile, parse_search_object, profile_to_js, search_result_to_json};
use crate::MyState;


#[tauri::command]
pub async fn supports_event_indexing() -> bool{
    println!("[Command] Supports event indexing");
    true
}

#[tauri::command]
pub async fn init_event_index<R: Runtime>(app_handle: AppHandle<R>, state: State<'_, Mutex<MyState>>, passphrase: String) -> Result<(), CommonError> {
    println!("[Command] init_event_index");

    let mut state_lock = state.lock().unwrap();
    
    // Check if the database is already initialized
    if state_lock.database.is_some() {
        println!("[Command] Database is already initialized.");
        return Ok(()); // No need to reinitialize
    }

    println!("[Command] init_event_index passphrase {:?}", passphrase);
    let config = Config::new().set_passphrase(passphrase);

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
pub async fn close_event_index(state: State<'_, Mutex<MyState>>)-> Result<(), CommonError> {
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
pub async fn delete_event_index<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), CommonError> {
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
            Err(e) => return Err(e.into()), // For other InvokeErrors, convert and return
        }
        
        Ok(())
}

#[tauri::command]
pub async fn add_event_to_index(state: State<'_, Mutex<MyState>>, event: serde_json::Value, profile: Option<serde_json::Value>) -> Result<(), CommonError> {
    println!("[Command] add_event_to_index");
    println!("[Command] add_event_to_index event {:?}", event);
    println!("[Command] add_event_to_index profile {:?}", profile);
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let event = parse_event(&event)?;
        let profile= match profile {
            Some(p) => parse_profile(&p)?,
            None => Profile { 
                displayname: None,
                avatar_url: None
            }
        };
        db_lock.add_event(event, profile);
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_event(state: State<'_, Mutex<MyState>>, event_id: String) -> Result<(), CommonError>{
    println!("[Command] delete_event {:?}", event_id);
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        db_lock.delete_event(&event_id);
    }
    Ok(())
}

#[tauri::command]
pub async fn commit_live_events(state: State<'_, Mutex<MyState>>) -> Result<(), CommonError> {
    println!("[Command] commit_live_events");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let mut db_lock = db.lock().unwrap();
        db_lock.commit();
    }
    Ok(())
}

#[tauri::command]
pub async fn search_event_index(state: State<'_, Mutex<MyState>>, search_config: Value) -> Result<Value, CommonError> {
    println!("[Command] search_event_index");
    let state_guard = state.lock().unwrap();
    
    if let Some(ref db) = state_guard.database {
        let (term, config) = parse_search_object(&search_config)?;
        let db_lock = db.lock().unwrap();
        let result = db_lock.search(&term, &config)?;
        
        let results: Vec<serde_json::Value> = result.results
            .into_iter()
            .map(|element| {
                search_result_to_json(element)
                    .unwrap_or_else(|_| serde_json::json!(null))
            })
            .collect();
        
        println!("search_event_index results {:?}", results);
        let mut search_result = serde_json::json!({
            "count": result.count,
            "results": results,
            "highlights": [],
        });

        if let Some(next_batch) = result.next_batch {
            search_result["next_batch"] = serde_json::json!(next_batch.hyphenated().to_string());
        }

        println!("[Command] search_event_index result {:?}", search_result);  
        Ok(search_result)
    } else {
        println!("[Command] search_event_index result no database found");  
        Ok(serde_json::json!({
            "count": 0,
            "results": [],
            "highlights": [],
        }))
    }
}

#[tauri::command]
pub async fn is_room_indexed(state: State<'_, Mutex<MyState>>, room_id: String) -> Result<bool, CommonError> {
    println!("[Command] is_room_indexed");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection
            .is_room_indexed(&room_id)
            .map_err(|e| CommonError::from(e))
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn is_event_index_empty(state: State<'_, Mutex<MyState>>) -> Result<bool, CommonError>{
    println!("[Command] is_event_index_empty");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        let result = connection.is_empty()?;
        
        println!("[Command] is_event_index_empty {:?}", result);
        Ok(result)
    } else {
        println!("[Command] is_event_index_empty true");
        Ok(true)
    }
}

#[tauri::command]
pub async fn add_historic_events(state: State<'_, Mutex<MyState>>, events: Vec<Value>, new_checkpoint: Option<Value>, old_checkpoint: Option<Value>) -> Result<bool, CommonError> {
    println!("[Command] add_historic_events");
    println!("[Command] add_historic_events args events ${:?}", events);
    println!("[Command] add_historic_events args newcheckpoint ${:?}", new_checkpoint);
    println!("[Command] add_historic_events args oldcheckpoint ${:?}", old_checkpoint);

    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock: std::sync::MutexGuard<'_, Database> = db.lock().unwrap();
        let (events, new_cp, old_cp) = add_historic_events_helper(events.as_ref(), new_checkpoint.as_ref(), old_checkpoint.as_ref())?;
        let receiver = db_lock.add_historic_events(events, new_cp, old_cp);

        receiver
            .recv()
            .map(|r| r.map_err(|e| CommonError::from(e)))
            .map_err(|recv_err| CommonError::from(recv_err))
            .unwrap()

    } else {
         // Create a dummy channel to return the expected type
        let (tx, rx) = mpsc::channel();
        let _ = tx.send(Ok(false));

        rx.recv()
            .map_err(|recv_err| CommonError::from(recv_err))
            .unwrap()
    }
}

#[tauri::command]
pub async fn get_stats(state: State<'_, Mutex<MyState>>) -> Result<DatabaseStats, CommonError> {
    println!("[Command] remove_crawler_checkpoint");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection.get_stats().map_err(|e| CommonError::from(e))
    } else {
        Err(CommonError::String(format!("No stats found")))
    }
}

#[tauri::command]
// There is no remove_crawler_checkpoint in the api, but we are only useing add_historic_events with the correct parameters
pub async fn remove_crawler_checkpoint(state: State<'_, Mutex<MyState>>, checkpoint: Option<Value>)-> Result<bool, CommonError> {
    println!("[Command] remove_crawler_checkpoint");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let (_, _, cp) = add_historic_events_helper(Vec::new().as_ref(), None, checkpoint.as_ref())?;
        let receiver = db_lock.add_historic_events(Vec::new(), None, cp);

        receiver
            .recv()
            .map(|r| r.map_err(|e| CommonError::from(e)))
            .map_err(|recv_err| CommonError::from(recv_err))
            .unwrap()
    } else {
        // Create a dummy channel to return the expected type
        let (tx, rx) = mpsc::channel();
        let _ = tx.send(Ok(false));

        rx
            .recv()
            .map_err(|recv_err| CommonError::from(recv_err))
            .unwrap()
    }
}

#[tauri::command]
// There is no add_crawler_checkpoint in the api, but we are only useing add_historic_events with the correct parameters
pub async fn add_crawler_checkpoint(state: State<'_, Mutex<MyState>>, checkpoint: Option<Value>)-> Result<bool, CommonError> {
    println!("[Command] add_crawler_checkpoint ${:?}", checkpoint);
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let (_, cp, _) = add_historic_events_helper(Vec::new().as_ref(), checkpoint.as_ref(), None)?;
        let receiver = db_lock.add_historic_events(Vec::new(), cp, None);

        receiver
            .recv()
            .map(|r| r.map_err(|e| CommonError::from(e)))
            .map_err(|recv_err| CommonError::from(recv_err))
            .unwrap()
    } else {
        // Create a dummy channel to return the expected type
        let (tx, rx) = mpsc::channel();
        let _ = tx.send(Ok(false));

        rx
            .recv()
            .map_err(|recv_err| CommonError::from(recv_err))
            .unwrap()
    }
}

#[tauri::command]
pub async fn load_file_events(state: State<'_, Mutex<MyState>>, load_config: Value) -> Result<Vec<(Value, Value)>, CommonError> {
    println!("[Command] load_file_events");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let room_id = load_config.get("roomId").unwrap();
        let mut config = LoadConfig::new(room_id.to_string());

        if let Some(e) = load_config.get("fromEvent") {
            config = config.from_event(e.to_string());
        };

        if let Some(d) = load_config.get("direction") {
            let d_string = d.to_string();
            let direction = match d_string.to_lowercase().as_str() {
                "backwards" | "backward" | "b" => LoadDirection::Backwards,
                "forwards" | "forward" | "f" => LoadDirection::Forwards,
                "" => LoadDirection::Backwards,
                _ => return Err(CommonError::String(format!("No direction found, could not load file event {:?}", d_string)))
            };

            config = config.direction(direction);
        }


        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        let result = connection.load_file_events(&config)?;
        let mut formatted_result = Vec::new();
        
        for (event_str, profile) in result {
            let event = match deserialize_event(&event_str) {
                Ok(event) => event,
                Err(e) => return Err(CommonError::String(e.to_string())), // Convert error if needed
            };

            let profile = match profile_to_js(profile) {
                Ok(event) => event,
                Err(e) => return Err(CommonError::String(e.to_string())), // Convert error if needed
            };

            formatted_result.push((event, profile));
        }

        Ok(formatted_result)
    } else {
        Err(CommonError::String(format!("No database found")))
    } 
}

#[tauri::command]
pub async fn load_checkpoints(state: State<'_, Mutex<MyState>>) -> Result<Vec<CrawlerCheckpoint>, CommonError>{
    println!("[Command] load_checkpoints");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection.load_checkpoints().map_err(|e| CommonError::from(e))
    } else {
        Err(CommonError::String(format!("No database found")))
    }
}

#[tauri::command]
pub async fn set_user_version(state: State<'_, Mutex<MyState>>, version: i64) -> Result<(), CommonError>{
    println!("[Command] set_user_version");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection
            .set_user_version(version)
            .map_err(|e| CommonError::from(e))
    } else {
        Ok(())
    }
}

#[tauri::command]
pub async fn get_user_version(state: State<'_, Mutex<MyState>>) -> Result<i64, CommonError>{
    println!("[Command] get_user_version");
    let state_guard = state.lock().unwrap();
    
    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        connection
            .get_user_version()
            .map_err(|e| CommonError::from(e))
    } else {
        Ok(0)
    }
}