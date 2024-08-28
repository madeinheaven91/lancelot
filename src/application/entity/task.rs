#![allow(dead_code)]
#![allow(unreachable_patterns)]

use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Task {
    pub title: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub responses: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub price_kind: Option<PriceKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub price_value: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub price_bounds: Option<(u32, u32)>,
    pub platform: Platform,
    // Habr
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub views: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    // FL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_urgent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_vacancy: Option<bool>,
    // Kwork
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expires_at: Option<String>,
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
    #[default]
    Habr,
    FL,
    Kwork,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PriceKind{
    PerProject,
    PerHour,
    Negotiated,
    PerMonth,
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
