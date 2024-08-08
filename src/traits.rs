pub enum Task {
    Habr {
        name: String,
        link: String,
        responses: u32,
        views: u32,
        published_at: String,
        tags: Vec<String>,
        price_is_negotiable: bool,
        price_type: Option<String>,
        price_count: Option<u32>,
    },
    FL,
    Kwork,
}
