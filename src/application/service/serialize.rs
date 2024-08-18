use std::collections::HashMap;

use crate::application::entity::task::*;
use serde_json::*;

pub fn json_task(task: Task) -> serde_json::Value {
    let mut map: HashMap<&str, Value> = HashMap::new();

    // Insertion of general properties
    map.insert("title", json!(&task.title));
    map.insert("url", json!(&task.url));
    map.insert("platform", json!(&task.platform));
    map.insert("responses", json!(&task.responses));
    // Insertion of prices
    map.insert("price_kind", json!(&task.price_kind));
    map.insert("price_value", json!(&task.price_value));
    map.insert("price_bounds", json!(&task.price_bounds));
    // Insertion of platform specific properties
    map.insert("views", json!(&task.views));
    map.insert("published_at", json!(&task.published_at));
    map.insert("tags", json!(&task.tags));
    map.insert("is_urgent", json!(&task.is_urgent));
    map.insert("is_vacancy", json!(&task.is_vacancy));
    map.insert("is_pinned", json!(&task.is_pinned));
    map.insert("expires_at", json!(&task.expires_at));

    json!(map)
}

pub fn json_task_vec(tasks: Vec<Task>) -> serde_json::Value {
    json!(tasks
        .iter()
        .map(|el| json_task(el.clone()))
        .collect::<Vec<_>>())
}
