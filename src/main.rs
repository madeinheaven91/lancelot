#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

mod application;

use application::controller::*;
use application::controller::logging::*;

#[tokio::main]
async fn main() {
    init_logging();
    _ = init_server().await;
}


