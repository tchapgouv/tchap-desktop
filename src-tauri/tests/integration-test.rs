
#[macro_use]
extern crate lazy_static;

use tauri::test::{mock_builder};
use serde_json::json;
use seshat::Event;
use tempfile::tempdir;
use fake::{faker::internet::raw::*, locales::*, Fake};
use serde_json::Value;
use seshat::{
    Config, Database, DatabaseStats, Error as SeshatError, LoadConfig, LoadDirection, Profile,
    RecoveryDatabase, EventType,
};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, Runtime, State};

use tchap_desktop_lib::MyState;
use tchap_desktop_lib::seshat_commands::*;



pub static EVENT_SOURCE: &str = "{
    content: {
        bodyp: Test message, msgtye: m.text
    },
    event_id: $15163622445EBvZJ:localhost,
    origin_server_ts: 1516362244026,
    sender: @example2:localhost,
    type: m.room.message,
    unsigned: {age: 43289803095},
    user_id: @example2:localhost,
    age: 43289803095
}";


lazy_static! {
    pub static ref EVENT: Event = Event::new(
        EventType::Message,
        "Test message",
        Some("m.text"),
        "$15ds63622445EBvZJ:localhost",
        "@example2:localhost",
        151636_2244026,
        "!test_room:localhost",
        EVENT_SOURCE,
    );
}

fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    builder
        .invoke_handler(tauri::generate_handler![delete_event_index, commit_live_events, init_event_index, add_event_to_index,search_event_index])
        // remove the string argument to use your app's config file
        .build(tauri::generate_context!("tauri.conf.json"))
        .expect("failed to build app")
}



// run in src-tauri : 
// RUST_BACKTRACE=0  RUST_LOG=debug cargo test --package tchap-desktop --test integration-test -- test_search_event_index --exact --show-output 
#[test]
fn test_search_event_index() {

    let body_passphrase = json!({ "passphrase": "London"});

    println!("body_passphrase : {}", serde_json::to_string_pretty(&body_passphrase).unwrap());

    
    //init db in state with command
    // tmp dir is deleted so fast
    /*
    let tmpdir = tempdir().unwrap();
    let path = tmpdir.path();
    println!("path : {}", &path.display());
    let mut db = Database::new(path).unwrap();
    let database_arc = Arc::new(Mutex::new(db));
    let initial_state = MyState { database: Some(Arc::clone(&database_arc)) };
    */

    
    let initial_state = MyState { database: None };
    
    let app = create_app(mock_builder());
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

    let matrix_event = json!({
                "content": {
                    "body": "coucou un pain au chocolat", 
                    "msgtype": "m.text"
                },
                "event_id": "$215163622445EBvZJ:localhost",
                "origin_server_ts": 100000,
                "sender": "@example2:localhost",
                "type": "m.room.message",
                "unsigned": {"age": 0},
                "user_id": "@example2:localhost",
                "room_id": "!TESTROOM",
                "age": 1000000});
        
    let profile = Profile::new("Alice", "");

    let body_event = json!({"event" : matrix_event, "profile": profile});

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


    let res = tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "commit_live_events".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            // alternatively use "tauri://localhost"
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::default(),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string()
        },
    );
    match &res {
            Ok(val) => { println!("got the T {:?}",val) }
            Err(e) => { println!("got the Err : {:?}", e) }
        }
    assert!(&res.is_ok());  

/*     let res = tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "reload_index".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            // alternatively use "tauri://localhost"
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::default(),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string()
        },
    );
    match &res {
            Ok(val) => { println!("got the T {:?}",val) }
            Err(e) => { println!("got the Err : {:?}", e) }
        }
    assert!(&res.is_ok());   */

    /* let state :State<'_, Mutex<MyState>>= app.state();
    let state_guard = state.lock().unwrap();
    if let Some(ref db) = state_guard.database {
        let mut db_lock = db.lock().unwrap();
        let _ = db_lock.commit();
    } */
    //search for literral "un" with command
    let body_search = 
        json!({"searchConfig": 
            {"search_term":"un",
//                "room_id": "!TESTROOM", 
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

    let clean = false;
    if clean {
    let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "delete_event_index".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                // alternatively use "tauri://localhost"
                url: "http://tauri.localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::default(),
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


fn fake_event() -> Event {
let domain: String = FreeEmailProvider(EN).fake();

Event::new(
    EventType::Message,
    "Hello world",
    Some("m.text"),
    &format!("${}:{}", (0..10).fake::<u8>(), &domain),
    &format!(
        "@{}:{}",
        Username(EN).fake::<String>(),
        FreeEmailProvider(EN).fake::<String>()
    ),
    151636_2244026,
    "!test_room:localhost",
    EVENT_SOURCE,
)
}

//#[test]
fn test_search_event_index_only() {
    
    let tmpdir = tempdir().unwrap();
    let mut db: Database = Database::new(tmpdir.path()).unwrap();
    
    let profile: Profile = Profile::new("Alice", "");

    db.add_event(EVENT.clone(), profile.clone());

    for i in 1..6 {
        let mut event: Event = fake_event();
        event.server_ts = EVENT.server_ts - i;
        event.source = format!("Hello before event {}", i);
        db.add_event(event, profile.clone());
    }

    //db.commit().unwrap();
    db.force_commit().unwrap();
    db.reload().unwrap();

    let database_arc = Arc::new(Mutex::new(db));
    let initial_state = MyState { database: Some(Arc::clone(&database_arc)) };

    let app = create_app(mock_builder());
    // Register it with Tauri's state management
    app.manage(Mutex::new(initial_state));

    let webview = 
        tauri::WebviewWindowBuilder::new(&app, "main", Default::default()).build().unwrap();


    let body_search = 
        json!({"searchConfig": 
            {"search_term":"Hello",
            "room_id": "!test_room:localhost",
            "limit": 10, 
            "before_limit": 1, 
            "after_limit": 1, 
            "order_by_recency": true, 
            "keys": []
        }});

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