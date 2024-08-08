use scraper::{ElementRef, Html, Selector};

pub async fn scrape(url: &str) -> Html {
    debug!("Requesting {url}");
    let req = reqwest::get(url).await.unwrap();
    match req.status().as_u16() {
        200 => info!("Successfully fetched {url}"),
        _ => {
            error!(" Couldn't fetch {url}")
        }
    };
    Html::parse_document(&req.text().await.unwrap_or_default())
}

pub fn get_attr<'a>(html: &'a ElementRef<'a>, selector: &'a str, attr: &'a str) -> String {
    let option = html
        .select(&Selector::parse(selector).unwrap())
        .next()
        .unwrap()
        .attr(attr);
    match option {
        Some(attr) => attr.to_string(),
        _ => "N/A".to_string(),
    }
}

pub fn get_inner_html(html: &ElementRef, selector: &str) -> String {
    let option = html.select(&Selector::parse(selector).unwrap()).next();
    match option {
        Some(element) => element.inner_html(),
        _ => "N/A".to_string(),
    }
}
