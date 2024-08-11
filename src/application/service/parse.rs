use crate::application::entity::task::*;
use crate::application::service::html::{get_attr, get_inner_html};
use scraper::{ElementRef, Html, Selector};

pub fn parse_html_habr(html: Html) -> Vec<Task> {
    let task_selector = Selector::parse(".task").unwrap();
    let task_articles: Vec<ElementRef<'_>> = html.select(&task_selector).collect();
    let mut tasks: Vec<Task> = vec![];

    // dbg!(&task_articles);
    for task in &task_articles {
        // dbg!(&task);
        let title = get_inner_html(task, ".task__title > a");
        let views = get_inner_html(task, ".params__views > i")
            .parse::<u32>()
            .unwrap_or_default();
        let responses = get_inner_html(task, ".params__responses > i")
            .parse::<u32>()
            .unwrap_or_default();

        // TODO: make this a unix timestamp instead of "7 минут назад"
        let published_at = get_inner_html(task, ".params__published-at > span");
        let url = get_attr(task, "a", "href");

        let price_value = get_inner_html(task, ".task__price > .count")
            .chars()
            .filter(|char| char.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or_default();

        let price_value = PriceValue::Exact(price_value);

        let price_kind = get_inner_html(task, ".suffix");

        let price_kind: PriceKind = match price_kind.as_str() {
            "за час" => PriceKind::PerHour,
            "за проект" => PriceKind::PerProject,
            _ => PriceKind::Negotiated,
        };

        let mut tags = vec![];
        for li in task.select(&Selector::parse("ul.tags > *").unwrap()) {
            let tag = get_inner_html(&li, "a");
            tags.push(tag);
        }

        let price = Price {
            kind: price_kind,
            value: price_value,
        };

        let task_specific = HabrTask {
            views,
            published_at,
            tags,
        };

        let task = Task {
            title,
            url,
            responses,
            price,
            platform: Platform::from(task_specific),
        };

        tasks.push(task);
    }

    tasks
}

fn filter_digits(string: String) -> u32 {
    string
        .chars()
        .filter(|el| el.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap_or_default()
}

pub fn parse_html_kwork(html: Html) -> Vec<Task> {
    let task_selector = Selector::parse(".want-card").unwrap();
    let task_articles: Vec<ElementRef<'_>> = html.select(&task_selector).collect();
    let mut tasks: Vec<Task> = vec![];

    // dbg!(&task_articles);
    for task in &task_articles {
        // dbg!(&task);
        let title = get_inner_html(task, ".wants-card__header-title > a");
        let responses = filter_digits(get_inner_html(task, "want-card__informers-row:nth-child(2)"));

        let url = get_attr(task, ".wants-card__header-title > a", "href");
        let expires_at = get_inner_html(task, "want-card__informers-row:first-child");

        let mut tags = vec![];
        for li in task.select(&Selector::parse("ul.tags > *").unwrap()) {
            let tag = get_inner_html(&li, "a");
            tags.push(tag);
        }

        let price_lower_bound = filter_digits(get_inner_html(task, ".wants-card__price .d-inline"));
        let price_upper_bound =
            filter_digits(get_inner_html(task, ".wants-card__description-higher-price .d-inline"));
    
        let price_value: PriceValue;
        match price_upper_bound{
            0 => price_value = PriceValue::Exact(price_lower_bound),
            _ => price_value = PriceValue::Range(price_lower_bound, price_upper_bound),
        }

        let price = Price {
            kind: PriceKind::PerProject,
            value: price_value,
        };

        let task_specific = KworkTask { expires_at };

        let task = Task {
            title,
            url,
            responses,
            price,
            platform: Platform::from(task_specific),
        };

        tasks.push(task);
    }

    tasks
}

// pub fn parse_html_fl(html: Html) -> Vec<Task> {
//     // dbg!(&html);
//     let task_selector = Selector::parse("[qa-project-name]").unwrap();
//     let task_articles: Vec<ElementRef<'_>> = html.select(&task_selector).collect();
//     let mut tasks: Vec<Task> = vec![];
//
//     dbg!(&task_articles);
//     for task in &task_articles {
//         dbg!(task);
//         let title = get_inner_html(task, "a");
//         let views = get_inner_html(task, ".params__views > i")
//             .parse::<u32>()
//             .unwrap_or_default();
//         let responses = get_inner_html(task, ".params__responses > i")
//             .parse::<u32>()
//             .unwrap_or_default();
//
//         // TODO: make this a unix timestamp instead of "7 минут назад"
//         let timestamp = get_inner_html(task, ".text-gray-opacity-4.text-7.mr-16");
//         let link = get_attr(task, "a", "href");
//
//         let price_count: String = get_inner_html(task, ".task__price > .count")
//             .chars()
//             .filter(|char| char.is_ascii_digit())
//             .collect();
//         let price_type = get_inner_html(task, ".suffix");
//
//         let price: Price = match price_type.as_str() {
//             "за час" => Price::PerHour(price_count.parse::<u32>().unwrap()),
//             "за проект" => Price::PerProject(price_count.parse::<u32>().unwrap()),
//             _ => Price::Negotiated,
//         };
//
//         let mut tags = vec![];
//         for li in task.select(&Selector::parse("ul.tags > *").unwrap()) {
//             let tag = get_inner_html(&li, "a");
//             tags.push(tag);
//         }
//
//         let task_element = Task {
//             title,
//             views,
//             responses,
//             link,
//             timestamp,
//             tags,
//             platform: Platform::FL,
//             price,
//         };
//         tasks.push(task_element);
//     }
//
//     tasks
// }

// pub fn parse_html_kwork(html: Html) -> Vec<Task>{
//
// }
//
