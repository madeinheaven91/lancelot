use std::collections::HashMap;

use serde_json::*;
use super::utils::*;
use super::super::entity::task::*;

pub fn serialize_task(task: Task) -> serde_json::Value{
    let mut map: HashMap<&str, Value> = HashMap::new();

    let price_count = match &task.price.get_price(){
        Some(val) => json!(val),
        _ => json!(null)
    };

    map.insert("title", json!(&task.title));
    map.insert("link", json!(&task.get_link()));
    map.insert("platform", json!(&task.get_platform()));
    map.insert("price_type", json!(&task.price.get_type()));
    map.insert("price", price_count);
    map.insert("views", json!(&task.views));
    map.insert("responses", json!(&task.responses));
    map.insert("timestamp", json!(&task.timestamp));
    map.insert("tags", json!(&task.tags));
    
    json!(map)
}

//Object {
//  title: String,
//  link: String,
//  platform: String,
//  price_type: String,
//  price: Number?,
//  views: Number,
//  responses: Number,
//  timestamp: Number,
//  tags: Array[String]
//}
