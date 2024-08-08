mod application;
use application::service::utils::*;
use application::service::parse::*;
use application::service::serialize::*;
use serde::Serialize;

#[tokio::main]
async fn main() {
    println!("[LOG] Lancelot working");
    let html_habr = scrape("https://freelance.habr.com/tasks").await;
    let tasks = parse_html_habr(html_habr);

    for task in tasks{
        let serialized = serialize_task(task);
        println!("{}", serialized);
    }
   // dbg!(tasks); 
}
