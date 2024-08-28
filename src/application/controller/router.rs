use std::{collections::HashMap, fs, path::Path};

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{body::Bytes, Method, Request, Response, StatusCode};
use serde::Serialize;

use crate::application::service::{
    http::{fetch_html, fetch_html_headless},
    json::{
        filter::filter,
        parse::{parse_html_fl, parse_html_habr, parse_html_kwork},
    },
};

pub async fn router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let params: HashMap<String, String> = req
        .uri()
        .query()
        .map(|v| form_urlencoded::parse(v.as_bytes()).into_owned().collect())
        .unwrap_or_default();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/parse/habr") => ok(&filter(
            &mut parse_html_habr(
                fetch_html("https://freelance.habr.com/tasks", "freelance.habr.com").await,
            ),
            params,
        )),
        (&Method::GET, "/parse/fl") => ok(&filter( &mut parse_html_fl(
            fetch_html_headless("https://www.fl.ru/projects", "div[qa-project-name]").await,
        ), params)),
        (&Method::GET, "/parse/kwork") => ok(&filter(&mut parse_html_kwork(
            fetch_html_headless("https://kwork.ru/projects", "div.want-card").await,
        ), params)),
        (&Method::GET, "/parse/all") => {
            let mut habr_tasks = parse_html_habr(
                fetch_html("https://freelance.habr.com/tasks", "freelance.habr.com").await,
            );
            let mut kwork_tasks = parse_html_kwork(
                fetch_html_headless("https://kwork.ru/projects", "div.want-card").await,
            );
            let mut fl_tasks = parse_html_fl(
                fetch_html_headless("https://www.fl.ru/projects", "div[qa-project-name]").await,
            );

            let mut res = vec![];
            res.append(&mut habr_tasks);
            res.append(&mut kwork_tasks);
            res.append(&mut fl_tasks);
            ok(&filter(&mut res, params))
        }
        (&Method::GET, "/info") => {
            ok_txt("./src/assets/html/info.html", "text/html; charset=utf-8")
        }
        (&Method::GET, "/error/not_found") => {
            let html = fs::read_to_string(Path::new("./src/assets/html/not_found.html")).unwrap();
            let res = res_html(html, StatusCode::NOT_FOUND);
            Ok(res)
        }
        (&Method::GET, "/assets/images/not_found.jpg") => {
            ok_img(src_plus_path(req.uri().path()).as_str())
        }
        (&Method::GET, "/assets/css/main.css") => {
            ok_txt(src_plus_path(req.uri().path()).as_str(), "text/css")
        }
        _ => {
            warn!("Requested page does not exist! Redirecting to 404 page...");
            let res = Response::builder()
                .header("Location", "/error/not_found")
                .status(308)
                .body(empty())
                .unwrap();
            Ok(res)
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

fn res_html(html: String, status_code: StatusCode) -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .status(status_code)
        .body(full(html))
        .unwrap()
}

fn ok_img(path: &str) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let img = fs::read(Path::new(&path)).unwrap();
    let res = Response::builder()
        .header("Content-Type", "image/jpeg")
        .body(full(img))
        .unwrap();
    Ok(res)
}

fn ok_txt(
    path: &str,
    content_type: &str,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let img = fs::read(Path::new(&path)).unwrap();
    let res = Response::builder()
        .header("Content-Type", content_type)
        .body(full(img))
        .unwrap();
    Ok(res)
}

fn src_plus_path(path: &str) -> String {
    "./src".to_string() + path
}
