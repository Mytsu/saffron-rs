use scraper::{ElementRef, Html, Selector};

pub fn get_selector(tag: &str) -> Selector {
    // println!("Selector: {}", tag);
    Selector::parse(tag).expect("Failed to parse selector")
}

pub fn get_element_ref<'a>(selector: &Selector, fragment: &'a Html) -> ElementRef<'a> {
    fragment
        .select(selector)
        .next()
        .expect("Failed to find element")
}

pub fn get_content(tag: &str, html: &str) -> Vec<String> {
    let selector = Selector::parse(tag).unwrap();
    let fragment = Html::parse_document(html);
    let element_contents = fragment.select(&selector)
        .map(|element| element.text().collect::<Vec<_>>())
        .flatten()
        .map(|text| text.to_string())
        .collect::<Vec<_>>();

    element_contents
}
