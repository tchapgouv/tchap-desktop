use seshat::{
    CheckpointDirection, CrawlerCheckpoint, Event, EventType, Profile, SearchConfig, SearchResult,
};

use anyhow::{Context, Result};
use serde_json::Value;
use uuid::Uuid;

pub(crate) fn parse_search_object(
    search_object: &serde_json::Value,
) -> Result<(String, SearchConfig), anyhow::Error> {
    let term = search_object
        .get("search_term")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing search_term"))?
        .to_string();

    let mut config = SearchConfig::new();

    // Parse limit
    if let Some(limit) = search_object.get("limit").and_then(|v| v.as_f64()) {
        config.limit(limit as usize);
    }

    // Parse before_limit
    if let Some(before_limit) = search_object.get("before_limit").and_then(|v| v.as_f64()) {
        config.before_limit(before_limit as usize);
    }

    // Parse after_limit
    if let Some(after_limit) = search_object.get("after_limit").and_then(|v| v.as_f64()) {
        config.after_limit(after_limit as usize);
    }

    // Parse order_by_recency
    if let Some(order_by_recency) = search_object
        .get("order_by_recency")
        .and_then(|v| v.as_bool())
    {
        config.order_by_recency(order_by_recency);
    }

    // Parse room_id
    if let Some(room_id) = search_object.get("room_id").and_then(|v| v.as_str()) {
        config.for_room(room_id);
    }

    // Parse next_batch
    if let Some(next_batch) = search_object.get("next_batch").and_then(|v| v.as_str()) {
        let token = next_batch
            .parse::<Uuid>()
            .map_err(|_| anyhow::anyhow!("Invalid next_batch token: {}", next_batch))?;
        config.next_batch(token);
    }

    // Parse keys
    if let Some(keys) = search_object.get("keys").and_then(|v| v.as_array()) {
        for key in keys {
            let key_str = key
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid key type"))?;

            match key_str {
                "content.body" => config.with_key(EventType::Message),
                "content.topic" => config.with_key(EventType::Topic),
                "content.name" => config.with_key(EventType::Name),
                _ => return Err(anyhow::anyhow!("Invalid search key: {}", key_str)),
            };
        }
    }

    Ok((term, config))
}

pub(crate) fn parse_checkpoint(checkpoint: Option<&Value>) -> Result<Option<CrawlerCheckpoint>> {
    match checkpoint {
        Some(Value::Object(obj)) => {
            let room_id = obj
                .get("roomId")
                .and_then(|v| v.as_str())
                .context("Missing roomId")?
                .to_string();

            let token = obj
                .get("token")
                .and_then(|v| v.as_str())
                .context("Missing token")?
                .to_string();

            let full_crawl = obj
                .get("fullCrawl")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let direction = obj.get("direction").and_then(|v| v.as_str()).unwrap_or("");

            let direction = match direction.to_lowercase().as_str() {
                "backwards" | "backward" | "b" => CheckpointDirection::Backwards,
                "forwards" | "forward" | "f" => CheckpointDirection::Forwards,
                "" => CheckpointDirection::Backwards,
                d => anyhow::bail!("Unknown checkpoint direction: {}", d),
            };

            Ok(Some(CrawlerCheckpoint {
                room_id,
                token,
                full_crawl,
                direction,
            }))
        }
        Some(Value::Null) | None => Ok(None),
        _ => anyhow::bail!("Invalid checkpoint type"),
    }
}

pub(crate) fn parse_event(event: &Value) -> Result<Event> {
    println!("---- parse_event event {:?}", event);
    let event_obj = event.as_object().context("Event must be an object")?;

    let sender = event_obj
        .get("sender")
        .and_then(|v| v.as_str())
        .context("Missing sender")?
        .to_string();

    let event_id = event_obj
        .get("event_id")
        .and_then(|v| v.as_str())
        .context("Missing event_id")?
        .to_string();

    let server_timestamp = event_obj
        .get("origin_server_ts")
        .and_then(|v| v.as_f64())
        .context("Missing or invalid timestamp")? as i64;

    let room_id = event_obj
        .get("room_id")
        .and_then(|v| v.as_str())
        .context("Missing room_id")?
        .to_string();

    let content = event_obj
        .get("content")
        .context("Missing content")?
        .as_object()
        .context("Content must be an object")?;

    let event_type_str = event_obj
        .get("type")
        .and_then(|v| v.as_str())
        .context("Missing event type")?;

    let event_type = match event_type_str {
        "m.room.message" => EventType::Message,
        "m.room.name" => EventType::Name,
        "m.room.topic" => EventType::Topic,
        _ => anyhow::bail!("Unsupported event type: {}", event_type_str),
    };

    let (key, content_value) = match event_type {
        EventType::Message => ("body", content.get("body")),
        EventType::Topic => ("topic", content.get("topic")),
        EventType::Name => ("name", content.get("name")),
    };

    let content_value = content_value
        .and_then(|v| v.as_str())
        .context(format!("Missing {} in content", key))?
        .to_string();

    let msgtype = match event_type {
        EventType::Message => content
            .get("msgtype")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        _ => None,
    };

    let event_source = serde_json::to_string(event).context("Cannot serialize event")?;

    Ok(Event {
        event_type,
        content_value,
        msgtype,
        event_id,
        sender,
        server_ts: server_timestamp,
        room_id,
        source: event_source,
    })
}

pub(crate) fn parse_profile(profile: &Value) -> Result<Profile> {
    let profile_obj = profile.as_object().context("Profile must be an object")?;

    let displayname = profile_obj.get("displayname").and_then(|v| match v {
        Value::String(s) => Some(s.to_string()),
        Value::Null => None,
        _ => None,
    });

    let avatar_url = profile_obj.get("avatar_url").and_then(|v| match v {
        Value::String(s) => Some(s.to_string()),
        Value::Null => None,
        _ => None,
    });

    Ok(Profile {
        displayname,
        avatar_url,
    })
}

pub(crate) fn add_historic_events_helper(
    events: &Vec<Value>,
    new_checkpoint: Option<&Value>,
    old_checkpoint: Option<&Value>,
) -> Result<(
    Vec<(Event, Profile)>,
    Option<CrawlerCheckpoint>,
    Option<CrawlerCheckpoint>,
)> {
    let mut parsed_events: Vec<(Event, Profile)> = Vec::new();

    for event_obj in events {
        let event_obj = event_obj.as_object().context("Event must be an object")?;

        let event = event_obj
            .get("event")
            .map(|e| parse_event(e))
            .transpose()?
            .unwrap_or(Event {
                event_type: EventType::Message,
                content_value: "".to_string(),
                msgtype: None,
                event_id: "".to_string(),
                sender: "".to_string(),
                server_ts: 0,
                room_id: "".to_string(),
                source: "".to_string(),
            });

        let profile = event_obj
            .get("profile")
            .map(|p| parse_profile(p))
            .transpose()?
            .unwrap_or(Profile {
                displayname: None,
                avatar_url: None,
            });

        parsed_events.push((event, profile));
    }

    let new_checkpoint = parse_checkpoint(new_checkpoint)?;
    let old_checkpoint = parse_checkpoint(old_checkpoint)?;

    Ok((parsed_events, new_checkpoint, old_checkpoint))
}

pub(crate) fn search_result_to_json(
    mut result: SearchResult,
) -> Result<serde_json::Value> {
    let rank = f64::from(result.score);
    let event = serde_json::from_str(&result.event_source)?;

    let mut context = serde_json::Map::new();
    let mut before = Vec::new();
    let mut after = Vec::new();
    let mut profile_info = serde_json::Map::new();

    for event in result.events_before.iter() {
        match serde_json::from_str(event) {
            Ok(js_event) => before.push(js_event),
            Err(_) => continue,
        }
    }

    for event in result.events_after.iter() {
        match serde_json::from_str(event) {
            Ok(js_event) => after.push(js_event),
            Err(_) => continue,
        }
    }

    for (sender, profile) in result.profile_info.drain() {
        let (js_sender, js_profile) = sender_and_profile_to_json(sender, profile)?;
        profile_info.insert(js_sender.to_string(), js_profile);
    }

    context.insert(
        "events_before".to_string(),
        serde_json::Value::Array(before),
    );
    context.insert("events_after".to_string(), serde_json::Value::Array(after));
    context.insert(
        "profile_info".to_string(),
        serde_json::Value::Object(profile_info),
    );

    let mut object = serde_json::Map::new();
    object.insert("rank".to_string(), serde_json::Value::from(rank));
    object.insert("result".to_string(), event);
    object.insert("context".to_string(), serde_json::Value::Object(context));

    Ok(serde_json::Value::Object(object))
}

pub fn profile_to_json(profile: Profile) -> Result<serde_json::Value> {
    let mut js_profile = serde_json::Map::new();

    match profile.displayname {
        Some(name) => {
            js_profile.insert("displayname".to_string(), serde_json::Value::String(name));
        }
        None => {
            js_profile.insert("displayname".to_string(), serde_json::Value::Null);
        }
    }

    match profile.avatar_url {
        Some(avatar) => {
            js_profile.insert("avatar_url".to_string(), serde_json::Value::String(avatar));
        }
        None => {
            js_profile.insert("avatar_url".to_string(), serde_json::Value::Null);
        }
    }

    Ok(serde_json::Value::Object(js_profile))
}
pub(crate) fn sender_and_profile_to_json(
    sender: String,
    profile: Profile,
) -> Result<(String, serde_json::Value)> {
    let profile_json = profile_to_json(profile)?;
    Ok((sender, profile_json))
}

pub(crate) fn deserialize_event(source: &str) -> Result<serde_json::Value, String> {
    let source = serde_json::from_str(source)
        .map_err(|e| format!("Couldn't load the event from the store: {}", e))?;

    Ok(source)
}
