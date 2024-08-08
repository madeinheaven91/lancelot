use serde::{Serialize, Deserialize};

// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
// pub enum Task {
//     Habr {
//         name: String,
//         link: String,
//         responses: u32,
//         views: u32,
//         published_at: String,
//         tags: Vec<String>,
//         price_is_negotiable: bool,
//         price_type: Option<String>,
//         price_count: Option<u32>,
//     },
//     FL{
//         name: String
//     },
//     Kwork{
//         name: String
//     },
//     #[default]
//     None
// }


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task{
    pub title: String,
    pub link: String,
    pub platform: Platform,
    pub views: u32,
    pub responses: u32,
    pub published_at: String,
    pub tags: Vec<String>,
    pub price: Price,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform{
    Habr,
    FL,
    Kwork
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Price{
    #[default]
    Negotiated,
    PerHour(u32),
    PerProject(u32)
}
