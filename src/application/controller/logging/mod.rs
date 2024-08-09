use chrono::Local;
use colored::{ColoredString, Colorize};
use log::Level;
use std::{env, io::Write, str::FromStr};
use hyper::{body::Incoming, service::Service, Request};

pub fn init_logging() {
    // Set log level depending on provided env variables
    if env::var_os("RUST_LOG").is_none() {
        log::set_max_level(log::LevelFilter::Info);
    } else {
        log::set_max_level(
            log::LevelFilter::from_str(env::var_os("RUST_LOG").unwrap().to_str().unwrap()).unwrap(),
        );
    }

    build_logger();
    greet();
}

fn colourful_loglevel(level: Level) -> ColoredString {
    match level {
        Level::Error => level.to_string().red(),
        Level::Warn => level.to_string().yellow(),
        Level::Info => level.to_string().blue(),
        Level::Debug => level.to_string().green(),
        Level::Trace => level.to_string().magenta(),
    }
}

fn greet() {
    println!(r"                       _                      _       _   ");
    println!(r"                      | |                    | |     | |  ");
    println!(r"                      | | __ _ _ __   ___ ___| | ___ | |_ ");
    println!(r"                      | |/ _` | '_ \ / __/ _ \ |/ _ \| __|");
    println!(r"                      | | (_| | | | | (_|  __/ | (_) | |_ ");
    println!(r"                      |_|\__,_|_| |_|\___\___|_|\___/ \__|");
    println!("\n");
    info!("Log level: {}", log::max_level());
}

fn build_logger() {
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
}


#[derive(Debug, Clone)]
pub struct Logger<S> {
    inner: S,
}
impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}
type Req = Request<Incoming>;

impl<S> Service<Req> for Logger<S>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn call(&self, req: Req) -> Self::Future {
        info!("Processing request: {} {}", req.method(), req.uri().path());
        self.inner.call(req)
    }
}
