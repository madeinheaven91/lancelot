use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task{
    pub title: String,
    pub url: String,
    pub responses: Option<u32>,
    pub platform: Platform,
    pub price: Price,
}

impl Task{
    pub fn get_link(&self) -> String {
        let mut link = match &self.platform {
            Platform::Habr(_) => "https://freelance.habr.com".to_string(),
            Platform::FL(_) => "https://www.fl.ru".to_string(),
            Platform::Kwork(_) => "https://kwork.ru".to_string(),
            // _ => "".to_string()
        };
        link.push_str(&self.url);
        link
    }

    pub fn get_platform_name(&self) -> String{
        match self.platform{
            Platform::Habr(_) => "Habr".to_string(),
            Platform::FL(_) => "FL".to_string(),
            Platform::Kwork(_) => "Kwork".to_string(),
        } 
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Platform{
    Habr(HabrTask),
    FL(FLTask),
    Kwork(KworkTask)
}

impl From<KworkTask> for Platform {
    fn from(v: KworkTask) -> Self {
        Self::Kwork(v)
    }
}

impl From<FLTask> for Platform {
    fn from(v: FLTask) -> Self {
        Self::FL(v)
    }
}

impl From<HabrTask> for Platform {
    fn from(v: HabrTask) -> Self {
        Self::Habr(v)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HabrTask{
    pub views: u32,
    pub published_at: String,
    pub tags: Vec<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FLTask{
    pub published_at: String,
    pub views: u32,
    pub is_urgent: bool,
    pub is_pinned: bool,
    pub is_vacancy: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KworkTask{
    pub expires_at: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Price{
    pub kind: PriceKind,
    pub value: PriceValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PriceKind{
    PerProject,
    PerHour,
    Negotiated,
    Monthly
}

impl Price{
    pub fn get_kind(&self) -> String{
        match self.kind{
            PriceKind::PerProject => "per project".to_string(),
            PriceKind::PerHour => "per hour".to_string(),
            PriceKind::Negotiated => "negotiated".to_string(),
            PriceKind::Monthly => "per month".to_string()
        } 
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PriceValue{
    Exact(u32),
    Range(u32, u32),
}
