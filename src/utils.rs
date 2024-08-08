use crate::traits::*;
use scraper::{ElementRef, Html, Selector};

pub async fn scrape_habr(url: &str) -> Html {
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
    let mut name: String;
    let mut views: u32;
    let mut responses: u32;
    let mut published_at: String;
    let mut link: String;
    let mut tags: Vec<String>;
    let mut price_count: Option<u32>;
    let mut price_is_negotiable: bool;
    let mut price_type: String;
    let mut task_element: Task;

    for task in &task_articles {
        // let name = task.select(&Selector::parse(".task__title").unwrap()).next().unwrap().attr("title").unwrap();
        // let name = get_attr(task, ".task__title", "title").unwrap();
        // let name = task.select(&Selector::parse(".task__title > a").unwrap()).next().unwrap().inner_html();
        // let views = task.select(&Selector::parse(".params__views > i.params__count").unwrap()).next().unwrap().inner_html();
        // let published_at = task.select(&Selector::parse(".params__published-at > span").unwrap()).next().unwrap().inner_html();

        name = get_inner_html(task, ".task__title > a");
        views = get_inner_html(task, ".params__views > i")
            .parse::<u32>()
            .unwrap();
        responses = get_inner_html(task, ".params__responses > i")
            .parse::<u32>()
            .unwrap_or_default();
        published_at = get_inner_html(task, ".params__published-at > span");
        link = get_attr(task, "a", "href");

        price_count = get_inner_html(task, ".task__price > .count")
            .strip_suffix(" руб. ")
            .unwrap()
            .to_string()
            .replace(" ", "")
            .parse::<u32>();
        tags = vec![];
        for li in task.select(&Selector::parse("ul.tags > *").unwrap()) {
            // dbg!(&li);
            let tag = get_inner_html(&li, "a");
            tags.push(tag);
        }

        task_element = Task::Habr {
            name,
            views,
            responses,
            link,
            published_at,
            tags,
            price_count,
        };

        tasks.push(task_element);

        // let views = get_attr(task, ".params__views > i", )
        dbg!(name, responses, views, published_at, link, tags);
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
