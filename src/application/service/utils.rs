use scraper::{ElementRef, Html, Selector};

pub async fn scrape(url: &str) -> Html {
    let req = reqwest::get(url).await.unwrap();
    match req.status().as_u16() {
        200 => println!("[LOG] Successfully fetched {}", url),
        _ => {
            panic!("[ERROR] Couldn't fetched {}, quitting", url)
        }
    };
    Html::parse_document(&req.text().await.unwrap())
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
