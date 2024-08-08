use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub link: String,
    pub platform: Platform,
    pub views: u32,
    pub responses: u32,
    pub timestamp: String,
    pub tags: Vec<String>,
    pub price: Price
}

impl Task {
    pub fn get_link(&self) -> String {
        let mut link = match &self.platform {
            Platform::Habr => "https://freelance.habr.com".to_string(),
            Platform::FL => "https://www.fl.ru/projects".to_string(),
            Platform::Kwork => "https://kwork.ru/projects".to_string(),
            _ => "".to_string()
        };
        link.push_str(&self.link);
        link
    }

    pub fn get_platform(&self) -> String{
        match &self.platform{
            Platform::Habr => "Habr".to_string(),
            Platform::Kwork => "Kwork".to_string(),
            Platform::FL => "FL".to_string(),
            _ => "".to_string()
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Price {
    #[default]
    Negotiated,
    PerHour(u32),
    PerProject(u32),
}

impl Price {
    pub fn get_price(&self) -> Option<u32> {
        match self {
            Price::Negotiated => None,
            Price::PerHour(val) => Some(*val),
            Price::PerProject(val) => Some(*val),
        }
    }
    pub fn get_type(&self) -> String{
        match self {
            Price::Negotiated => "negotiated".to_string(),
            Price::PerHour(_) => "per hour".to_string(),
            Price::PerProject(_) => "per project".to_string()
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub enum Platform {
    Habr,
    FL,
    Kwork,
    #[default]
    None
}
