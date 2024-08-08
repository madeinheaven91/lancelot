extern crate reqwest;
extern crate scraper;
extern crate serde;

mod traits;
mod utils;
use crate::utils::*;
use crate::traits::*;
use scraper::{ElementRef, Html, Selector};

#[tokio::main]
async fn main() {
    println!("[LOG] Lancelot working");
    let habr_html = scrape_habr("https://freelance.habr.com/tasks").await;
    parse_html_habr(habr_html);

}



