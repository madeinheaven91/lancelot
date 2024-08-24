use crate::application::service::html::{exists, get_attr, get_inner_html};
use crate::application::{
    entity::task::*,
    service::html::{filter_digits, get_text, get_text_filtered},
};
use scraper::{ElementRef, Html, Selector};

pub fn parse_html_habr(html: Html) -> Vec<Task> {
    let task_selector = Selector::parse(".task").unwrap();
    let task_articles: Vec<ElementRef> = html.select(&task_selector).collect();
    let mut tasks: Vec<Task> = vec![];

    for task in &task_articles {
        let title = get_inner_html(task, ".task__title > a");
        let views = get_inner_html(task, ".params__views > i")
            .parse::<u32>()
            .ok();
        let responses = get_inner_html(task, ".params__responses > i")
            .parse::<u32>()
            .ok();

        let published_at = Some(get_inner_html(task, ".params__published-at > span"));
        let url = get_attr(task, "a", "href");

        let price_value = get_inner_html(task, ".task__price > .count")
            .chars()
            .filter(|char| char.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .ok();

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
        let tags = Some(tags);

        tasks.push(Task {
            title,
            url,
            responses,
            price_kind,
            price_value,
            platform: Platform::Habr,
            published_at,
            views,
            tags,
            is_pinned: None,
            is_urgent: None,
            is_vacancy: None,
            expires_at: None,
            price_bounds: None,
        });
    }

    tasks
}

// TODO: отрефачить, а то код просто пиздец
pub fn parse_html_fl(html: Html) -> Vec<Task> {
    // Fucking tailwind
    let task_selector = Selector::parse("[qa-project-name]").unwrap();
    let task_articles: Vec<ElementRef> = html.select(&task_selector).collect();
    let mut tasks: Vec<Task> = vec![];

    for task in &task_articles {
        let title = get_inner_html(task, ".b-post__title > a");

        let views = Some(filter_digits(get_text(
            task.select(&Selector::parse("span[title='Количество просмотров'] span").unwrap())
                .next()
        )));

        let response_element = task
            .select(&Selector::parse("div.b-post__txt.b-post__txt_fontsize_11 + a").unwrap())
            .next();
        let responses = match response_element {
            None => None,
            _ => Some(filter_digits(get_text(response_element))),
        };

        let published_at = Some(get_inner_html(task, ".text-gray-opacity-4.text-7.mr-16"));
        let url = get_attr(task, ".b-post__title > a", "href");

        let binding = task
            .select(&Selector::parse("div.b-post__price > span").unwrap())
            .next();
        let price_text = get_text(binding);

        let price_kind = if price_text.contains("час") {
            PriceKind::PerHour
        } else if price_text.contains("заказ") {
            PriceKind::PerProject
        } else if price_text.contains("По результатам собеседования") {
            PriceKind::Negotiated
        } else {
            PriceKind::PerMonth
        };

        let price_value = match filter_digits(price_text.clone()) {
            0 => None,
            _ => Some(filter_digits(price_text.clone())),
        };

        let price_bounds: Option<(u32, u32)> = if price_text.contains('—') {
            let bound = price_text
                .split('—')
                .map(|el| filter_digits(el.to_string()))
                .collect::<Vec<_>>();
            let lower_bound = bound[0];
            let upper_bound = bound[1];
            Some((lower_bound, upper_bound))
        } else {
            None
        };

        let binding = get_inner_html(
            task,
            "span.b-post__bold.b-layout__txt_inline-block.text-7.text-dark.mr-4",
        );
        let is_vacancy = Some(!binding.contains("Заказ"));

        let is_urgent = Some(exists(
            task,
            "[src='https://st.fl.ru/images/urgently-1.png']",
        ));
        let is_pinned = Some(exists(task, ".b-post__pin"));

        tasks.push(Task {
            title,
            url,
            responses,
            price_kind,
            price_value,
            price_bounds,
            platform: Platform::FL,
            published_at,
            views,
            tags: None,
            is_pinned,
            is_urgent,
            is_vacancy,
            expires_at: None,
        });
    }

    tasks
}

pub fn parse_html_kwork(html: Html) -> Vec<Task> {
    let task_selector = Selector::parse(".want-card").unwrap();
    let task_articles: Vec<ElementRef> = html.select(&task_selector).collect();
    let mut tasks: Vec<Task> = vec![];

    for task in &task_articles {
        let title = get_inner_html(task, ".wants-card__header-title > a");
        let responses = Some(filter_digits(get_inner_html(
            task,
            ".want-card__informers-row > span + span",
        )));

        let url = get_attr(task, ".wants-card__header-title > a", "href");

        let expires_at = Some(get_text_filtered(
            task.select(&Selector::parse(".want-card__informers-row > span").unwrap())
                .next()
                .unwrap(),
        ));
        let mut tags = vec![];
        for li in task.select(&Selector::parse("ul.tags > *").unwrap()) {
            let tag = get_inner_html(&li, "a");
            tags.push(tag);
        }

        let price_lower_bound = filter_digits(get_inner_html(task, ".wants-card__price .d-inline"));
        let price_upper_bound = filter_digits(get_inner_html(
            task,
            ".wants-card__description-higher-price .d-inline",
        ));

        let (price_value, price_bounds) = match price_upper_bound {
            0 => (Some(price_lower_bound), None),
            _ => (None, Some((price_lower_bound, price_upper_bound))),
        };

        tasks.push(Task {
            title,
            url,
            responses,
            price_kind: PriceKind::None,
            price_value,
            price_bounds,
            platform: Platform::Kwork,
            published_at: None,
            views: None,
            tags: None,
            is_pinned: None,
            is_urgent: None,
            is_vacancy: None,
            expires_at,
        });
    }

    tasks
}

// TODO: make timestamps for all kinds of tasks
