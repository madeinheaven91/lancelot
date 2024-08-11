use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{body::Bytes, Method, Request, Response, StatusCode};
use serde::Serialize;

use crate::{
    application::service::{
        http::fetch_html,
        parse,
        serialize::{json_task, json_task_vec},
    },
    Logger,
};

use crate::application::service::parse::*;

pub async fn router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/habr") => ok(&json_task_vec(parse_html_habr(
            fetch_html("https://freelance.habr.com/tasks").await,
        ))),
        // (&Method::GET, "/fl") => ok(&json_task_vec(parse_html_fl(
        //     fetch_html("https://fl.ru/projects").await,
        // ))),
        // (&Method::GET, "/kwork") => ok(&json_task_vec(parse_html_kwork(
        //     fetch_html("https://kwork.ru/projects").await,
        // ))),

        (&Method::GET, "/test/client") => ok(&json_task_vec(parse_html_habr(
            fetch_html("http://httpbin.org/ip").await
        ))),
        (&Method::GET, "/test/server") => ok(&"test".to_string()),
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
    info!("Processed request");
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
