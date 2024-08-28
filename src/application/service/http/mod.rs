use fantoccini::{ClientBuilder, Locator};
use hyper::header::HOST;
use scraper::Html;
use crate::{application::service::http::utils::gen_headers, log_res_status};
mod utils;

pub async fn fetch_html_headless(url: &str, await_css: &str) -> Html {
    let port = match std::env::var("WEBDRIVER_PORT") {
        Ok(v) => v,
        Err(_) => String::from("4444"),
    };

    let not_casted_url = &format!("http://localhost:{}/", port);
    let webdriver_url = not_casted_url.as_str();

    let mut cap = serde_json::map::Map::new();
    let opts = serde_json::json!({
        "args":[
            "--headless"
        ]
    });
    cap.insert("moz:firefoxOptions".to_string(), opts);

    let client = ClientBuilder::native()
        .capabilities(cap)
        .connect(webdriver_url)
        .await
        .expect("Failed to connect to WebDriver");

    info!("Waiting for {url} to load");
    let goto = client.goto(url).await;
    if goto.is_err() {
        error!("Webdriver couldn't go to {url}")
    }

    let element = client.wait().for_element(Locator::Css(await_css)).await;
    if element.is_err() {
        error!("Webdriver couldn't await element, which selector is {await_css}")
    }

    let res = client.source().await.unwrap();
    info!("Successfully loaded {url}");
    scraper::Html::parse_document(res.as_str())
}

pub async fn fetch_html(url: &str, hostname: &str) -> Html {
    let client = reqwest::Client::new();
    debug!("Requesting {url}");
    let headers = gen_headers();
    debug!("Headers: {headers:?}");
    let req = client.get(url).headers(headers).header(HOST, hostname);
    debug!("Request: {req:?}");
    let res = req.send().await.unwrap();
    log_res_status(&res.status(), url);

    Html::parse_document(&res.text().await.unwrap_or_default())
}
