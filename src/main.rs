extern crate reqwest;
extern crate scraper;
extern crate serde;

mod traits;
mod utils;
use crate::utils::*;
use scraper::{ElementRef, Html, Selector};

#[tokio::main]
async fn main() {
    println!("[LOG] Lancelot working");
    let html_habr = scrape("https://freelance.habr.com/tasks").await;
    let tasks = parse_html_habr(html_habr);
    dbg!(tasks);
}
