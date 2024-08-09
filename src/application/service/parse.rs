use crate::application::service::html::{get_attr, get_inner_html};
use crate::application::entity::task::*;
use scraper::{ElementRef, Html, Selector};

pub fn parse_html_habr(html: Html) -> Vec<Task> {
    let task_selector = Selector::parse(".task").unwrap();
    let task_articles: Vec<ElementRef<'_>> = html.select(&task_selector).collect();
    let mut tasks: Vec<Task> = vec![];

    for task in &task_articles {
        let title = get_inner_html(task, ".task__title > a");
        let views = get_inner_html(task, ".params__views > i")
            .parse::<u32>()
            .unwrap();
        let responses = get_inner_html(task, ".params__responses > i")
            .parse::<u32>()
            .unwrap_or_default();

        // TODO: make this a unix timestamp instead of "7 минут назад"
        let timestamp = get_inner_html(task, ".params__published-at > span");
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
            timestamp,
            tags,
            platform: Platform::Habr,
            price,
        };
        tasks.push(task_element);
    }

    tasks
}

pub fn parse_html_fl(html: Html) -> Vec<Task>{
    vec![Default::default()]
}

pub fn parse_html_kwork(html: Html) -> Vec<Task>{
    vec![Default::default()]
}


