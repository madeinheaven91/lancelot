pub mod logging;
mod router;

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{
    body::Bytes, server::conn::http1, Method, Request, Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use serde::Serialize;
use tower::ServiceBuilder;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::{application::service::{http::fetch_html, serialize::json_task_vec}, Logger};
use crate::application::controller::router::*;


pub async fn init_server() -> Result<(), Box<dyn std::error::Error>> {
    // let port = dotenv!("PORT").parse::<u32>().unwrap_or(3000);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening at {}", addr.to_string());

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::spawn(async move {
            let svc = hyper::service::service_fn(router);
            let svc = ServiceBuilder::new().layer_fn(Logger::new).service(svc);
            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                error!("Server error: {}", err);
            }
        });
    }
}
