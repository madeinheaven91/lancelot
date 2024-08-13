use crate::application::service::html::{exists, get_attr, get_inner_html};
use crate::application::{
    entity::task::*,
    service::html::{filter_digits, get_text},
};
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
        let responses = Some(
            get_inner_html(task, ".params__responses > i")
                .parse::<u32>()
                .unwrap_or_default(),
        );

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

// TODO: отрефачить, а то код просто пиздец
pub fn parse_html_fl(html: Html) -> Vec<Task> {
    let task_selector = Selector::parse("[qa-project-name]").unwrap();
    let task_articles: Vec<ElementRef<'_>> = html.select(&task_selector).collect();
    let mut tasks: Vec<Task> = vec![];

    // dbg!(&task_articles);
    for task in &task_articles {
        // dbg!(&task);
        let title = get_inner_html(task, ".b-post__title > a");
        let views = filter_digits(get_text(
            task.select(&Selector::parse("span[title='Количество просмотров'] span").unwrap())
                .next()
                .unwrap(),
        ));

        let response_element = task
            .select(&Selector::parse("div.b-post__txt.b-post__txt_fontsize_11 + a").unwrap())
            .next();
        let responses = match response_element {
            None => None,
            _ => Some(filter_digits(get_text(response_element.unwrap()))),
        };
        // TODO: make this a unix timestamp instead of "7 минут назад"
        let published_at = get_inner_html(task, ".text-gray-opacity-4.text-7.mr-16");
        let url = get_attr(task, ".b-post__title > a", "href");

        let binding = task
            .select(&Selector::parse("div.b-post__price > span").unwrap())
            .next();

        let price_text = get_text(binding.unwrap());

        let mut price_kind = PriceKind::Monthly;
        if price_text.contains("час") {
            price_kind = PriceKind::PerHour
        } else if price_text.contains("заказ") {
            price_kind = PriceKind::PerProject
        } else if price_text.contains("По результатам собеседования") {
            price_kind = PriceKind::Negotiated
        }

        let price_value: PriceValue = if price_text.contains('—') {
            let bound = price_text
                .split('—')
                .map(|el| filter_digits(el.to_string()))
                .collect::<Vec<_>>();
            let lower_bound = bound[0];
            let upper_bound = bound[1];
            PriceValue::Range(lower_bound, upper_bound)
        } else {
            let value = filter_digits(price_text);
            PriceValue::Exact(value)
        };

        let binding = get_inner_html(
            task,
            "span.b-post__bold.b-layout__txt_inline-block.text-7.text-dark.mr-4",
        );
        let is_vacancy: bool = !binding.contains("Заказ");

        let is_urgent = exists(task, "[src='https://st.fl.ru/images/urgently-1.png']");
        let is_pinned = exists(task, ".b-post__pin");

        let price = Price {
            kind: price_kind,
            value: price_value,
        };

        let task_specific = FLTask {
            published_at,
            views,
            is_urgent,
            is_vacancy,
            is_pinned,
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

pub fn parse_html_kwork(html: Html) -> Vec<Task> {
    let task_selector = Selector::parse(".want-card").unwrap();
    let task_articles: Vec<ElementRef<'_>> = html.select(&task_selector).collect();
    let mut tasks: Vec<Task> = vec![];

    for task in &task_articles {
        let title = get_inner_html(task, ".wants-card__header-title > a");
        let responses = Some(filter_digits(get_inner_html(
            task,
            ".want-card__informers-row > span + span",
        )));

        let url = get_attr(task, ".wants-card__header-title > a", "href");
        let expires_at = get_inner_html(task, ".want-card__informers-row > span")
            .into_bytes()
            .into_iter()
            .filter(|el| el != &9u8 && el != &10u8)
            .collect::<Vec<_>>();
        let expires_at = String::from_utf8(expires_at).unwrap_or_default();
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

        let price_value = match price_upper_bound {
            0 => PriceValue::Exact(price_lower_bound),
            _ => PriceValue::Range(price_lower_bound, price_upper_bound),
        };

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
