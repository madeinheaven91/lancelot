use std::fmt::Debug;

use serde::{Deserialize, Serialize};

// pub struct Task{
//     pub title: String,
//     pub url: String,
//     pub responses: Option<u32>,
//     pub platform: Platform,
//     pub price: Price,
// }

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Task{
    pub title: String,
    pub url: String,
    pub responses: Option<u32>,
    pub price_kind: PriceKind,
    pub price_value: Option<u32>,
    pub price_bounds: Option<(u32, u32)>,
    pub platform: Platform,
    // Habr
    pub views: Option<u32>,
    pub published_at: Option<String>,
    pub tags: Option<Vec<String>>,
    // FL
    pub is_urgent: Option<bool>,
    pub is_pinned: Option<bool>,
    pub is_vacancy: Option<bool>,
    // Kwork
    pub expires_at: Option<String>
}


impl Task{
    pub fn link(&self) -> String {
        let mut link = match &self.platform {
            Platform::Habr => "https://freelance.habr.com".to_string(),
            Platform::FL => "https://www.fl.ru".to_string(),
            Platform::Kwork => "https://kwork.ru".to_string(),
            _ => "".to_string()
        };
        link.push_str(&self.url);
        link
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum Platform{
    Habr,
    FL,
    Kwork,
    #[default]
    None
}

impl Platform{
    pub fn to_string(&self) -> String{
        match self{
            Platform::Habr => "Habr".to_string(),
            Platform::FL => "FL".to_string(),
            Platform::Kwork => "Kwork".to_string(),
            _ => "".to_string()
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum PriceKind{
    PerProject,
    PerHour,
    Negotiated,
    PerMonth,
    #[default]
    None
}

impl PriceKind{
    pub fn to_string(&self) -> String{
        match self{
            PriceKind::PerProject => "per project".to_string(),
            PriceKind::PerHour => "per hour".to_string(),
            PriceKind::Negotiated => "negotiated".to_string(),
            PriceKind::PerMonth => "per month".to_string(),
            _ => "".to_string()
        } 
    }
}
