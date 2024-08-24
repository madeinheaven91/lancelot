use scraper::{ElementRef, Selector};

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


pub fn filter_digits(string: String) -> u32 {
    string
        .chars()
        .filter(|el| el.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap_or_default()
}

pub fn get_text(element: Option<ElementRef>) -> String{
    if element.is_none() { return String::new() };
    let text = element.unwrap().text().collect::<String>();
    text
}

pub fn get_text_filtered(element: ElementRef) -> String{
    let text = element.text().collect::<String>();
    text.chars().filter(|char| !char.is_control()).collect::<String>()
}

pub fn exists(element: &ElementRef, selector: &str) -> bool{
    let selection = element.select(&Selector::parse(selector).unwrap()).next();
    selection.is_some() 
}
