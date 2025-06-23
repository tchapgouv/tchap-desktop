    use serde_json::Value;
use seshat::{
    Config, Database, DatabaseStats, Error as SeshatError, EventType, LoadConfig, LoadDirection, Profile, RecoveryDatabase
};
use std::fs;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::common_error::CommonError;
use crate::seshat_utils::{
    add_historic_events_helper, checkpoints_to_json, deserialize_event, parse_event, parse_profile,
    parse_search_object, perform_manual_reindex, profile_to_json, search_result_to_json,
};
use crate::MyState;

#[tauri::command]
pub async fn supports_event_indexing() -> bool {
    println!("[Command] Supports event indexing");
    true
}

#[tauri::command]
pub async fn init_event_index<R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, Mutex<MyState>>,
    passphrase: String,
) -> Result<(), CommonError> {
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
    let db_path = app_handle
        .path()
        .app_local_data_dir()
        .expect("could not resolve app local data path")
        .join("seshat_db");

    let _ = fs::create_dir_all(&db_path);

    let db_result = Database::new_with_config(&db_path, &config);

    let database = match db_result {
        Ok(db) => {
            println!("[Command] Database opened successfully on first attempt.");
            db // Use the successfully opened database
        }
        Err(SeshatError::ReindexError) => {
            println!("[Command] Database requires reindexing. Attempting recovery...");

            // --- Recovery Logic ---
            let recovery_config = config.clone(); // Clone config for recovery DB
            let recovery_db = RecoveryDatabase::new_with_config(&db_path, &recovery_config)
                .map_err(|e| {
                    CommonError::String(format!("Failed to open recovery database: {}", e))
                })?;

            let user_version = {
                // Scope the connection
                let connection = recovery_db.get_connection().map_err(|e| {
                    CommonError::String(format!("Failed to get recovery DB connection: {}", e))
                })?;
                connection.get_user_version().map_err(|e| {
                    CommonError::String(format!(
                        "Failed to get user version from recovery DB: {}",
                        e
                    ))
                })?
            };

            println!("[Command] Recovery DB user version: {}", user_version);

            if user_version == 0 {
                println!("[Command] User version is 0. Deleting database contents instead of reindexing.");
                // Drop recovery_db explicitly *before* deleting files to release file handles
                drop(recovery_db);
                fs::remove_dir_all(&db_path).map_err(|e| {
                    CommonError::String(format!("Failed to delete database for re-creation: {}", e))
                })?;
                // Re-create the directory after deletion
                fs::create_dir_all(&db_path).map_err(|e| {
                    CommonError::String(format!("Failed to re-create DB directory: {}", e))
                })?;
            } else {
                println!("[Command] Reindexing database...");
                // reindex() consumes the recovery_db
                perform_manual_reindex(recovery_db)
                    .map_err(|e| CommonError::String(format!("Manual reindexing failed: {}", e)))?;
                println!("[Command] Reindexing complete.");
            }

            // --- Retry opening the main database after recovery/deletion ---
            println!("[Command] Retrying to open main database after recovery/deletion...");
            Database::new_with_config(&db_path, &config).map_err(|e| {
                CommonError::String(format!(
                    "Failed to open database even after recovery attempt: {}",
                    e
                ))
            })?
        }
        Err(e) => {
            // Handle other database opening errors
            return Err(CommonError::String(format!(
                "Error opening the database: {:?}",
                e
            )));
        }
    };

    // --- Store the successfully opened database (either first try or after recovery) ---
    let database_arc = Arc::new(Mutex::new(database));
    state_lock.database = Some(Arc::clone(&database_arc));
    println!("[Command] init_event_index completed successfully.");

    Ok(())
}

#[tauri::command]
// Closing the database
pub async fn close_event_index(state: State<'_, Mutex<MyState>>) -> Result<(), CommonError> {
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
    let db_path = app_handle
        .path()
        .app_local_data_dir()
        .expect("could not resolve app local data path")
        .join("seshat_db");

    // Handle the case where the directory doesn't exist
    match fs::remove_dir_all(&db_path) {
        Ok(_) => println!("Successfully deleted index at: {:?}", db_path),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!(
                "Index directory not found at: {:?}, continuing anyway",
                db_path
            );
        }
        Err(e) => return Err(e.into()), // For other InvokeErrors, convert and return
    }

    Ok(())
}

#[tauri::command]
pub async fn add_event_to_index(
    state: State<'_, Mutex<MyState>>,
    event: serde_json::Value,
    profile: Option<serde_json::Value>,
) -> Result<(), CommonError> {
    println!("[Command] add_event_to_index");
    let state_guard = state.lock().unwrap();
    
    if let Some(ref db) = state_guard.database {
        println!("[Command]  add_event_to_index : event json : {}", serde_json::to_string_pretty(&event).unwrap());
        let db_lock = db.lock().unwrap();
        let event = parse_event(&event)?;
        println!("[Command] add_event_to_index event {:?}", event);
        let profile = match profile {
            Some(p) => parse_profile(&p)?,
            None => Profile {
                displayname: None,
                avatar_url: None,
            },
        };
        println!("[Command] add_event_to_index profile {:?}", profile);
        db_lock.add_event(event, profile);
    }else{
        println!("[Command] add_event_to_index, database is not init");
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_event(
    state: State<'_, Mutex<MyState>>,
    event_id: String,
) -> Result<(), CommonError> {
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
        let _ = db_lock.commit();
    }
    Ok(())
}

#[tauri::command]
pub async fn search_event_index(
    state: State<'_, Mutex<MyState>>,
    search_config: Value,
) -> Result<Value, CommonError> {
    println!("[Command] search_event_index");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let (term, config) = parse_search_object(&search_config)?;
        println!("---- search_event_index config {:?}", config);
        println!("---- search_event_index term {:?}", term);
        let db_lock: std::sync::MutexGuard<'_, Database> = db.lock().unwrap();
        let result = db_lock.search(&term, &config).unwrap();

        println!("---- search_event_index results before parse {:?}", result);
        println!("---- search_event_index results before count {:?}", result.count);
        println!("---- search_event_index results before lenght {:?}", result.results.len());
        let results: Vec<serde_json::Value> = result
            .results
            .into_iter()
            .map(|element| {
                search_result_to_json(element).unwrap_or_else(|_| serde_json::json!(null))
            })
            .collect();

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
pub async fn is_room_indexed(
    state: State<'_, Mutex<MyState>>,
    room_id: String,
) -> Result<bool, CommonError> {
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
pub async fn is_event_index_empty(state: State<'_, Mutex<MyState>>) -> Result<bool, CommonError> {
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
pub async fn add_historic_events(
    state: State<'_, Mutex<MyState>>,
    events: Vec<Value>,
    new_checkpoint: Option<Value>,
    old_checkpoint: Option<Value>,
) -> Result<bool, CommonError> {
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock: std::sync::MutexGuard<'_, Database> = db.lock().unwrap();
        let (events, new_cp, old_cp) = add_historic_events_helper(
            events.as_ref(),
            new_checkpoint.as_ref(),
            old_checkpoint.as_ref(),
        )?;

        let receiver = db_lock.add_historic_events(events, new_cp, old_cp);

        match receiver.recv() {
            Ok(result) => {
                let final_result = result.map_err(|e| CommonError::from(e))?;
                // Get stats after adding events
                let connection = db_lock.get_connection().map_err(|e| CommonError::from(e))?;
                let stats_after = connection.get_stats().map_err(|e| CommonError::from(e))?;
                println!(
                    "[Command] Stats after: event_count={}, room_count={}",
                    stats_after.event_count, stats_after.room_count
                );

                Ok(final_result)
            }
            Err(recv_err) => {
                println!("[Error] Failed to receive result: {:?}", recv_err);
                Err(CommonError::from(recv_err))
            }
        }
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
pub async fn remove_crawler_checkpoint(
    state: State<'_, Mutex<MyState>>,
    checkpoint: Option<Value>,
) -> Result<bool, CommonError> {
    println!("[Command] remove_crawler_checkpoint");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let (_, _, cp) =
            add_historic_events_helper(Vec::new().as_ref(), None, checkpoint.as_ref())?;
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

        rx.recv()
            .map_err(|recv_err| CommonError::from(recv_err))
            .unwrap()
    }
}

#[tauri::command]
// There is no add_crawler_checkpoint in the api, but we are only useing add_historic_events with the correct parameters
pub async fn add_crawler_checkpoint(
    state: State<'_, Mutex<MyState>>,
    checkpoint: Option<Value>,
) -> Result<bool, CommonError> {
    println!("[Command] add_crawler_checkpoint ${:?}", checkpoint);
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let (_, cp, _) =
            add_historic_events_helper(Vec::new().as_ref(), checkpoint.as_ref(), None)?;

        println!("[Debug] Processed checkpoint for adding: {:?}", cp);
        let receiver = db_lock.add_historic_events(Vec::new(), cp, None);

        // let result = receiver
        //     .recv()
        //     .map(|r| r.map_err(|e| CommonError::from(e)))
        //     .map_err(|recv_err| CommonError::from(recv_err))
        //     .unwrap();
        match receiver.recv() {
            Ok(result) => {
                let final_result = result.map_err(|e| CommonError::from(e))?;
                println!("[Debug] Result of adding checkpoint: {:?}", final_result);
                Ok(final_result)
            }
            Err(recv_err) => {
                println!("[Error] Failed to receive result: {:?}", recv_err);
                Err(CommonError::from(recv_err))
            }
        }
        // println!("[Debug] Result of adding checkpoint: {:?}", result);
        // result
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
pub async fn load_file_events(
    state: State<'_, Mutex<MyState>>,
    load_config: Value,
) -> Result<Vec<Value>, CommonError> {
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
                _ => {
                    return Err(CommonError::String(format!(
                        "No direction found, could not load file event {:?}",
                        d_string
                    )))
                }
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

            let profile = match profile_to_json(profile) {
                Ok(event) => event,
                Err(e) => return Err(CommonError::String(e.to_string())), // Convert error if needed
            };

            formatted_result.push(serde_json::json!({
                "event": event,
                "profile": profile
            }));
        }

        Ok(formatted_result)
    } else {
        Err(CommonError::String(format!("No database found")))
    }
}

#[tauri::command]
pub async fn load_checkpoints(state: State<'_, Mutex<MyState>>) -> Result<Vec<Value>, CommonError> {
    println!("[Command] load_checkpoints");
    let state_guard = state.lock().unwrap();

    if let Some(ref db) = state_guard.database {
        let db_lock = db.lock().unwrap();
        let connection = db_lock.get_connection().unwrap();
        let checkpoints = connection.load_checkpoints().unwrap();

        println!("---- load_checkpoints raw results count: {:?}", checkpoints);

        // Use the helper function to convert the Vec<CrawlerCheckpoint> to JSON Value
        let json_result = checkpoints_to_json(checkpoints)?;

        Ok(json_result)
    } else {
        Err(CommonError::String(format!("No database found")))
    }
}

#[tauri::command]
pub async fn set_user_version(
    state: State<'_, Mutex<MyState>>,
    version: i64,
) -> Result<(), CommonError> {
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
pub async fn get_user_version(state: State<'_, Mutex<MyState>>) -> Result<i64, CommonError> {
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

/*
#[tauri::command]
fn something(state: tauri::State<String>) -> String {
    state.inner().clone()
}
 */

#[cfg(test)]
mod tests {

    use super::*;
    use tauri::test::{mock_builder};
    use tauri::Manager;
    use serde_json::json;
    use seshat::Event;

    fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
        builder
            .invoke_handler(tauri::generate_handler![init_event_index, add_event_to_index,search_event_index])
            // remove the string argument to use your app's config file
            .build(tauri::generate_context!("tauri.conf.json"))
            .expect("failed to build app")
    }

    /*
    #[test]
    fn test_main() {
        // Use `tauri::Builder::default()` to use the default runtime rather than the `MockRuntime`;
        // let app = create_app(tauri::Builder::default());
        let app = create_app(mock_builder());
        let webview = 
            tauri::WebviewWindowBuilder::new(&app, "main", Default::default()).build().unwrap();

        // run the `ping` command and assert it returns `pong`
        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "ping".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                // alternatively use "tauri://localhost"
                url: "http://tauri.localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::default(),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        ).map(|b| b.deserialize::<String>().unwrap());

        let response = res.unwrap();

        print!("response {:?}", response);
    }
 */
/* 
    #[tauri::command]
    fn ping() -> &'static str {
        "pong"
    }


    #[test]
    pub fn test_something() {
        let app = tauri::test::mock_app();
        app.manage("something".to_string());
        assert_eq!(&something(app.state::<String>()), "something");
    }
*/

/* 
    pub static EVENT_SOURCE: &str = r#"{
        "event":{
            "content": {
                "body": "Test message, msgtype: m.text"
            },
            "event_id": "$15163622445EBvZJ:localhost",
            "origin_server_ts": 1516362244026,
            "sender": "@example2:localhost",
            "type": "m.room.message",
            "unsigned": {"age": 43289803095},
            "user_id": "@example2:localhost",
            "age": 43289803095
        }   
    }"#;
    */

    

/* 
    lazy_static! {
        pub static ref EVENT: Event = Event::new(
            EventType::Message,
            "Test message",
            Some("m.text"),
            "$15163622445EBvZJ:localhost",
            "@example2:localhost",
            151636_2244026,
            "!test_room:localhost",
            EVENT_SOURCE,
        );
    } */


    // run in src-tauri : 
    // RUST_BACKTRACE=0  RUST_LOG=debug cargo test test_search_event_index  -- --show-output
    #[test]
    fn test_search_event_index() {

        //let passphrase = "sdfsfsfds";
        let body_passphrase = json!({ "passphrase": "London"});

        println!("body_passphrase : {}", serde_json::to_string_pretty(&body_passphrase).unwrap());

        //init db in state with command
        let app = create_app(mock_builder());
        let initial_state = MyState { database: None };

        // Register it with Tauri's state management
        app.manage(Mutex::new(initial_state));
        
        let webview = 
            tauri::WebviewWindowBuilder::new(&app, "main", Default::default()).build().unwrap();

        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "init_event_index".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                // alternatively use "tauri://localhost"
                url: "http://tauri.localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(body_passphrase),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string()
            },
        );

        match &res {
                Ok(val) => { println!("got the T {:?}",val) }
                Err(e) => { println!("got the Err : {:?}", e) }
            }
       assert!(&res.is_ok());  

        let event_json = json!({
                    "content": {
                        "body": "coucou un pain au chocolat", 
                        "msgtype": "m.text"
                    },
                    "event_id": "$15163622445EBvZJ:localhost",
                    "origin_server_ts": 100000,
                    "sender": "@example2:localhost",
                    "type": "m.room.message",
                    "unsigned": {"age": 0},
                    "user_id": "@example2:localhost",
                    "room_id": "sdfsdfsdf",
                    "age": 1000000});
            

        let body_event = json!({"event" : event_json}
                );

        println!("body event : {}", serde_json::to_string_pretty(&body_event).unwrap());


         let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "add_event_to_index".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                // alternatively use "tauri://localhost"
                url: "http://tauri.localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(body_event),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string()
            },
        );
        match &res {
                Ok(val) => { println!("got the T {:?}",val) }
                Err(e) => { println!("got the Err : {:?}", e) }
            }
       assert!(&res.is_ok());  

        //search for literral "un" with command
        let body_search = 
            json!({"searchConfig": 
                {"search_term":"", 
                "limit": 10, "before_limit": 1, "after_limit": 1, 
                "order_by_recency": true, "keys": []}});

        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "search_event_index".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                // alternatively use "tauri://localhost"
                url: "http://tauri.localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(body_search),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string()
            },
        );

        match &res {
                Ok(val) => { println!("got the T {:?}",val) }
                Err(e) => { println!("got the Err : {:?}", e) }
            }
       assert!(&res.is_ok());  
    }

 
}