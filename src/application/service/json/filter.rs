use crate::application::entity::task::{PriceKind, Task};
use std::collections::HashMap;

pub fn filter(tasks: &mut [Task], query: HashMap<String, String>) -> Vec<Task> {
    let tasks = tasks.to_owned();
    if query.is_empty() {
        return tasks.clone();
    };

    let responses_lte = query
        .get("responses[lte]")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(u32::MAX);
    let responses_gte = query
        .get("responses[gte]")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    let views_lte = query
        .get("views[lte]")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(u32::MAX);
    let views_gte = query
        .get("views[gte]")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    let price_lte = query
        .get("price_value[lte]")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(u32::MAX);
    let price_gte = query
        .get("price_value[gte]")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    let resp_bounds = (responses_gte, responses_lte);
    let views_bounds = (views_gte, views_lte);
    let price_bounds = (price_gte, price_lte);

    tasks
        .iter()
        .filter(|el| {
            query.iter().all(|(key, value)| match key.as_str() {
                "price_kind" => match value.as_str() {
                    "negotiated" => el.price_kind == Some(PriceKind::Negotiated),
                    "per_project" => el.price_kind == Some(PriceKind::PerProject),
                    "per_hour" => el.price_kind == Some(PriceKind::PerHour),
                    _ => false,
                },

                "responses[eq]" => el.responses.map_or(false, |r| r.to_string() == *value),
                "responses[lte]" | "responses[gte]" => {
                    el.responses.map_or(false, |r| is_in_bounds(r, resp_bounds))
                },
                "views[eq]" => el.views.map_or(false, |r| r.to_string() == *value) || el.views.is_none(),
                "views[lte]" | "views[gte]" => {
                    el.views.map_or(false, |r| is_in_bounds(r, views_bounds)) || el.views.is_none()
                },

                "price_value[lte]" | "price_value[gte]" => {
                    let el_bounds= if el.price_bounds.is_none(){
                        if el.price_value.is_none(){
                            return false;
                        }else{
                            (el.price_value.unwrap(), el.price_value.unwrap())
                        }
                    }else{
                        el.price_bounds.unwrap()
                    };
                    is_subset( el_bounds, price_bounds)
                },

                "pinned" => el.is_pinned.map_or(false, |p| p.to_string() == *value),
                "urgent" => el.is_urgent.map_or(false, |u| u.to_string() == *value),
                "vacancy" => el.is_vacancy.map_or(false, |v| v.to_string() == *value),
                _ => true,
            })
        })
        .cloned()
        .collect()
}

// NOTE: фильтры:
// по просмотрам (больше меньше равно в промежутке)
// по ответам (больше меньше равно в промежутке)
// по типу цены (договорная, за проект, за заказ)
// по цене (больше меньше в промежутке)
// закреп, срочно, вакансия

fn is_in_bounds(num: u32, bounds: (u32, u32)) -> bool {
    num <= bounds.1 && num >= bounds.0
}

fn is_subset(set: (u32, u32), superset: (u32, u32)) -> bool {
    set.0 > superset.0 && set.1 < superset.1
}
