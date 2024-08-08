#[macro_use]
extern crate log;
extern crate env_logger;

use chrono::Local;
use colored::{ColoredString, Colorize};
use log::Level;
use std::{env, io::Write, str::FromStr};mod application;

use application::service::utils::*;
use application::service::parse::*;
use application::service::serialize::*;
use serde::{Deserialize, Serialize};
use env_logger::*;
use log::*;

#[tokio::main]
async fn main() {
    init_logging();
    let html_habr = scrape("https://freelance.habr.com/tasks").await;
    let tasks = parse_html_habr(html_habr);
}

fn init_logging() {
    if env::var_os("RUST_LOG").is_none() {
        log::set_max_level(log::LevelFilter::Info);
    } else {
        log::set_max_level(
            log::LevelFilter::from_str(env::var_os("RUST_LOG").unwrap().to_str().unwrap()).unwrap(),
        );
    }

    env_logger::Builder::from_default_env()
        .filter_level(log::max_level())
        .format(|buf, record| {
            writeln!(
                buf,
                "{}{}:\t{}",
                Local::now().format("[ %d/%m/%Y - %H:%M:%S ] "),
                colourful_loglevel(record.level()),
                record.args()
            )
        })
        .init();

    println!(
        "----------------------------------------------------------------------------------------|"
    );
    println!(" --> Log level: {}", log::max_level());
    println!(r"  _                      _       _   ");
    println!(r" | |                    | |     | |  ");
    println!(r" | | __ _ _ __   ___ ___| | ___ | |_ ");
    println!(r" | |/ _` | '_ \ / __/ _ \ |/ _ \| __|");
    println!(r" | | (_| | | | | (_|  __/ | (_) | |_ ");
    println!(r" |_|\__,_|_| |_|\___\___|_|\___/ \__|");
    println!("\n");
}

fn colourful_loglevel(level: Level) -> ColoredString {
    match level {
        Level::Error => level.to_string().red(),
        Level::Warn => level.to_string().yellow(),
        Level::Info => level.to_string().blue(),
        Level::Debug => level.to_string().cyan(),
        Level::Trace => level.to_string().magenta(),
    }
}
