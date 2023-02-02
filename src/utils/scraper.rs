use scraper::{ElementRef, Html, Selector};

use super::errors::ErrorMessages;

pub(crate) fn get_selector(tag: &str) -> Selector {
    Selector::parse(tag).expect(ErrorMessages::ParseSelector.as_str())
}

pub(crate) fn get_element_ref<'a>(selector: &Selector, fragment: &'a Html) -> ElementRef<'a> {
    fragment
        .select(selector)
        .next()
        .expect(ErrorMessages::FindElement.as_str())
}

pub(crate) fn get_content(tag: &str, html: &str) -> Vec<String> {
    let selector = Selector::parse(tag).unwrap();
    Html::parse_document(html)
        .select(&selector)
        .map(|element| element.text().collect::<Vec<_>>())
        .flatten()
        .map(|text| text.to_string())
        .collect::<Vec<_>>()
}
