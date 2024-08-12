use std::collections::HashMap;

use crate::application::entity::task::*;
use serde_json::*;

pub fn json_task(task: Task) -> serde_json::Value {
    let mut map: HashMap<&str, Value> = HashMap::new();

    // Insertion of general properties
    map.insert("title", json!(&task.title));
    map.insert("link", json!(&task.get_link()));
    map.insert("platform", json!(&task.get_platform_name()));
    map.insert("responses", json!(&task.responses));
  
    // Insertion of prices
    let price = &task.price;
    let price_kind = match price.kind{
        PriceKind::PerProject => json!("per project"),
        PriceKind::PerHour =>  json!("per hour"),
        PriceKind::Negotiated => json!("negotiated")
    };
    map.insert("price_kind", price_kind);

    let mut price_value: Option<u32> = None;
    let mut price_bounds: Option<(u32, u32)> = None;
    
    match price.value{
        PriceValue::Exact(val) => {price_value = Some(val)},
        PriceValue::Range(lower, upper) => {price_bounds = Some((lower, upper))},
    }

    if price_value.unwrap_or_default() == 0{
        if price_bounds.unwrap_or_default() == (0u32, 0u32){
            map.insert("price_value", json!(null));
        }else{
            map.insert("price_lower_bound", json!(price_bounds.unwrap_or_default().0));
            map.insert("price_upper_bound", json!(price_bounds.unwrap_or_default().1));
        }
    }else{
        map.insert("price_value", json!(price_value.unwrap_or_default()));
    }

    // Insertion of platform specific properties
    match task.platform {
        Platform::Habr(specific_task) => {
            map.insert("views", json!(&specific_task.views));
            map.insert("published_at", json!(&specific_task.published_at));
            map.insert("tags", json!(&specific_task.tags));
        },
        Platform::FL(specific_task) => {
            map.insert("is_urgent", json!(&specific_task.is_urgent));
            map.insert("is_vacancy", json!(&specific_task.is_vacancy));
            map.insert("is_pinned", json!(&specific_task.is_pinned));
            map.insert("views", json!(&specific_task.views));
        },
        Platform::Kwork(specific_task) => {
            map.insert("expires_at", json!(&specific_task.expires_at));
        },
        _ => () ,
    }
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

pub fn json_task_vec(tasks: Vec<Task>) -> serde_json::Value {
    json!(tasks
        .iter()
        .map(|el| json_task(el.clone()))
        .collect::<Vec<_>>())
}
