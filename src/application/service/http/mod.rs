use std::env;

use fantoccini::{ClientBuilder, Locator};
use hyper::{header::{self, HeaderName, HeaderValue, HOST}, HeaderMap};
// use http_body_util::Empty;
// use hyper::body::Bytes;
// use hyper::Request;
// use hyper_util::rt::TokioIo;
// use scraper::{ElementRef, Html, Selector};
// use tokio::net::TcpStream;
//
// type HyperError = Box<dyn std::error::Error + Send + Sync>;
// type SendReq<T> = hyper::client::conn::http1::SendRequest<T>;
//
// pub async fn connect(url: &str) -> Result<(), HyperError>{
//     let url = url.parse::<hyper::Uri>()?;
//
//     // Get the host and the port
//     let host = url.host().expect("uri has no host");
//     let port = url.port_u16().unwrap_or(80);
//
//     let address = format!("{}:{}", host, port);
//
//     // Open a TCP connection to the remote host
//     let stream = TcpStream::connect(address).await?;
//
//     // Use an adapter to access something implementing `tokio::io` traits as if they implement
//     // `hyper::rt` IO traits.
//     let io = TokioIo::new(stream);
//
//     // Create the Hyper client
//
//     let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
//
//     // Spawn a task to poll the connection, driving the HTTP state
//     tokio::task::spawn(async move {
//         if let Err(err) = conn.await {
//             println!("Connection failed: {:?}", err);
//         }
//     });
//
//     let authority = url.authority().unwrap().clone();
//
//     // Create an HTTP request with an empty body and a HOST header
//     let req = Request::builder()
//         .uri(url)
//         .header(hyper::header::HOST, authority.as_str())
//         .body(Empty::<Bytes>::new())?;
//
//     // Await the response...
//     let mut res = sender.send_request(req).await?;
//
//     println!("Response status: {}", res.status());
//     Ok(())
//
// }
//
// fn parse_address(url_str: &str) -> Result<String, HyperError> {
//     let url = url_str.parse::<hyper::Uri>()?;
//
//     let host = url.host().expect("uri has no host");
//     let port = url.port_u16().unwrap_or(80);
//
//     let res = format!("{}:{}", host, port);
//     Ok(res)
// }
//
// async fn open_tcp(address: &str) -> Result<TcpStream, HyperError> {
//     let stream = TcpStream::connect(address).await?;
//     Ok(stream)
// }
//
// // async fn ee() {
// //     let url = "http://httpbin.org/ip".parse::<hyper::Uri>()?;
// //
// //     // Get the host and the port
// //     let host = url.host().expect("uri has no host");
// //     let port = url.port_u16().unwrap_or(80);
// //
// //     let address = format!("{}:{}", host, port);
// //
// //     // Open a TCP connection to the remote host
// //     let stream = TcpStream::connect(address).await?;
// //
// //     // Use an adapter to access something implementing `tokio::io` traits as if they implement
// //     // `hyper::rt` IO traits.
// //     let io = TokioIo::new(stream);
// //
// //     // Create the Hyper client
// //
// //     let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
// //
// //     // Spawn a task to poll the connection, driving the HTTP state
// //     tokio::task::spawn(async move {
// //         if let Err(err) = conn.await {
// //             println!("Connection failed: {:?}", err);
// //         }
// //     });
// //
// //     let authority = url.authority().unwrap().clone();
// //
// //     // Create an HTTP request with an empty body and a HOST header
// //     let req = Request::builder()
// //         .uri(url)
// //         .header(hyper::header::HOST, authority.as_str())
// //         .body(Empty::<Bytes>::new())?;
// //
// //     // Await the response...
// //     let mut res = sender.send_request(req).await?;
// //
// //     println!("Response status: {}", res.status());
// // }
// async fn fetch_url(url: hyper::Uri) -> Result<(), HyperError> {
//     let host = url.host().expect("uri has no host");
//     let port = url.port_u16().unwrap_or(80);
//     let addr = format!("{}:{}", host, port);
//     let stream = TcpStream::connect(addr).await?;
//     let io = TokioIo::new(stream);
//
//     let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
//     tokio::task::spawn(async move {
//         if let Err(err) = conn.await {
//             println!("Connection failed: {:?}", err);
//         }
//     });
//
//     let authority = url.authority().unwrap().clone();
//
//     let path = url.path();
//     let req = Request::builder()
//         .uri(path)
//         .header(hyper::header::HOST, authority.as_str())
//         .body(Empty::<Bytes>::new())?;
//
//     let mut res = sender.send_request(req).await?;
//
//     println!("Response: {}", res.status());
//     println!("Headers: {:#?}\n", res.headers());
//
//     // Stream the body, writing each chunk to stdout as we get it
//     // (instead of buffering and printing at the end).
//     while let Some(next) = res.frame().await {
//         let frame = next?;
//         if let Some(chunk) = frame.data_ref() {
//             io::stdout().write_all(&chunk).await?;
//         }
//     }
//
//     println!("\n\nDone!");
//
//     Ok(())
// }
//
//TODO: rewrite http client for hyper instead of reqwest
use scraper::Html;

use crate::application::service::http::utils::{gen_headers, random_user_agent};
mod utils;
    
pub async fn fetch_html_headless(url: &str, await_css: &str) -> Html {
    let client = ClientBuilder::native().connect("http://localhost:4444/").await.expect("Failed to connect to WebDriver");
    client.goto(url).await;
    info!("Waiting for {url} to load");
    client.wait().for_element(Locator::Css(await_css)).await;
    let res = client.source().await.unwrap();
    info!("Successfully loaded {url}");
    scraper::Html::parse_document(res.as_str())
}

pub async fn fetch_html(url: &str, hostname: &str) -> Html {
    let client = reqwest::Client::new();
   
    let headers = gen_headers();
    debug!("Requesting {url}");
    let req = client
        .get(url)
        .headers(headers)
        .header(HOST, hostname);

    if env::var_os("RUST_LOG").unwrap_or_default().to_str().unwrap() == "debug" { dbg!(&req); }; 
    let res = req
        .send()
        .await
        .unwrap();
    match res.status().as_u16() / 100 {
        4 => error!("GET {} : {}", url, res.status()),
        _ => info!("GET {} : {}", url, res.status()),

    };
    Html::parse_document(&res.text().await.unwrap_or_default())
}

