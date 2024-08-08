use crate::traits::*;
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

// TODO: make this mothefucker return vec of tasks
pub fn parse_html_habr(html: Html) -> Vec<Task> {
    let task_selector = Selector::parse(".task").unwrap();
    let mut task_articles = vec![];
    for element in html.select(&task_selector) {
        task_articles.push(element);
    }

    let mut tasks: Vec<Task> = vec![];

    for task in &task_articles {
        let title = get_inner_html(task, ".task__title > a");
        let views = get_inner_html(task, ".params__views > i")
            .parse::<u32>()
            .unwrap();
        let responses = get_inner_html(task, ".params__responses > i")
            .parse::<u32>()
            .unwrap_or_default();
        let published_at = get_inner_html(task, ".params__published-at > span");
        let link = get_attr(task, "a", "href");

        let price_count: String = get_inner_html(task, ".task__price > .count")
            .chars()
            .filter(|char| char.is_ascii_digit())
            .collect();
        let price_type = get_inner_html(task, ".suffix");

        let price: Price = match price_type.as_str() {
            "за час" => Price::PerHour(price_count.parse::<u32>().unwrap()),
            "за проект" => Price::PerProject(price_count.parse::<u32>().unwrap()),
            _ => Price::Negotiated,
        };

        let mut tags = vec![];
        for li in task.select(&Selector::parse("ul.tags > *").unwrap()) {
            let tag = get_inner_html(&li, "a");
            tags.push(tag);
        }

        let task_element = Task {
            title,
            views,
            responses,
            link,
            published_at,
            tags,
            platform: Platform::Habr,
            price,
        };
        // dbg!(&tasks);
        tasks.push(task_element);
    }

    tasks
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
