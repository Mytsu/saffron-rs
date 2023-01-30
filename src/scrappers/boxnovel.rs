use crate::novel::{Chapter, Novel};
use crate::utils::scraper::{get_content, get_element_ref, get_selector};
use reqwest::blocking::{get, Client};
use scraper::{Html, Selector};
use std::{thread, time};

pub fn get_novel(url: &str) -> Novel {
    let body = get_response(url);
    let mut chapters = Vec::<Chapter>::new();
    let mut chapter_links = fetch_chapter_links(url);
    chapter_links.reverse();
    for link in chapter_links {
        chapters.push(get_chapter(&link));
        thread::sleep(time::Duration::from_millis(500));
    }
    Novel::new(
        get_novel_title(&body),
        get_novel_synopsis(&body),
        get_novel_author(&body),
        get_cover_url(&body),
        url.to_string(),
        chapters,
    )
}

pub fn get_cover_url(body: &str) -> String {
    let selector = get_selector(".summary_image > a > .img-responsive");
    Html::parse_fragment(body)
        .select(&selector)
        .flat_map(|el| el.value().attr("src"))
        .collect()
}

pub fn get_novel_title(body: &str) -> String {
    let title = get_element_content(".post-title > h1", body);
    println!("Fetching: {}", title);
    title.replace( "\n", "").replace("\t", "")
}

pub fn get_novel_synopsis(body: &str) -> Vec<String> {
    get_content(".summary__content p", body)
}

pub fn get_novel_author(body: &str) -> Vec<String> {
    let author = get_content(".author-content > a", body);
    println!("{}", author.join(", "));
    author
}

pub fn get_chapter_links(body: &str) -> Vec<String> {
    let selector = Selector::parse("li.wp-manga-chapter > a")
        .unwrap();
    let fragment = Html::parse_document(body);
    let mut links = Vec::<String>::new();
    for el in fragment.select(&selector) {
        links.push(el.value().attr("href").unwrap().to_owned());
    }

    links
}

pub fn get_chapter(url: &str) -> Chapter {
    let body = get_response(url);
    let title = get_chapter_title(&body);
    println!("{}", title);
    Chapter::new(title, get_chapter_content(&body))
}

pub fn get_chapter_title(body: &str) -> String {
    get_element_content(".breadcrumb li.active", body)
        .replace("\n", "")
        .replace("\t", "")
}

pub fn get_chapter_content(body: &str) -> Vec<String> {
    get_content(".text-left > p", body)
}

pub fn get_response(url: &str) -> String {
    println!("Url: {}", url);
    get(url).unwrap().text().unwrap()
}

pub fn get_element_content(tag: &str, body: &str) -> String {
    let selector = get_selector(tag);
    let fragment = Html::parse_document(body);
    let element_ref = get_element_ref(&selector, &fragment);
    element_ref
        .text()
        .next()
        .expect("Failed to extract cover url")
        .to_owned()
}

pub fn fetch_chapter_links(url: &str) -> Vec<String> {
    // TODO: Handle if url has no / at the end
    let new_url = vec![url, "ajax/chapters"].join("");
    let client = Client::new();
    let body = client.post(&new_url).send().unwrap().text().unwrap();
    let links = get_chapter_links(&body);
    links
}
