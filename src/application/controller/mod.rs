pub mod logging;

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{
    body::Bytes, server::conn::http1, service::service_fn, Method, Request, Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use serde::Serialize;
use tower::ServiceBuilder;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::{application::service::{serialize::json_task_vec, utils::scrape}, Logger};

use super::service::parse::*;

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
                eprintln!("server error: {}", err);
            }
        });
    }
}

async fn router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full("Bebra"))),
        (&Method::GET, "/habr") => ok(
            &json_task_vec(
               parse_html_habr(
                    scrape("https://freelance.habr.com/tasks").await
                ) 
            )
        ),
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn ok<T>(result: &T) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>
where
    T: Serialize,
{
    let response = Response::builder()
        .header("Content-Type", "application/json")
        .body(full(serde_json::to_string(result).unwrap()))
        .unwrap();
    Ok(response)
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
