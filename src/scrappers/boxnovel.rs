use crate::utils::errors::ErrorMessages;
use crate::utils::novel::{Chapter, Novel};
use crate::utils::scraper::{get_content, get_element_ref, get_selector};
use reqwest::blocking::{get, Client};
use scraper::{Html, Selector};
use std::{thread, time};
use url::Url;

pub(crate) const HOSTNAME: &str = "boxnovel.com";
pub(crate) const COVER_URL: &str = ".summary_image > a > .img-responsive";
pub(crate) const NOVEL_TITLE: &str = ".post-title > h1";
pub(crate) const NOVEL_SYNOPSIS: &str = ".summary__content p";
pub(crate) const AUTHOR: &str = ".author-content > a";
pub(crate) const CHAPTER_URL: &str = "li.wp-manga-chapter > a";
pub(crate) const CHAPTER_TITLE: &str = ".breadcrumb li.active";
pub(crate) const CHAPTER_CONTENT: &str = ".text-left p";
pub(crate) const REQUEST_DELAY: u64 = 500;

pub(crate) fn get_novel(input: &str) -> Novel {
    let url = Url::parse(input).expect(ErrorMessages::ParseUrl.as_str());
    assert!(
        url.host_str().expect(ErrorMessages::ParseHostname.as_str()) == HOSTNAME,
        "Hostname doesn't match {}",
        HOSTNAME
    );
    let body = get_response(&url);
    let mut chapters = Vec::<Chapter>::new();
    let mut chapter_links = fetch_chapter_links(&url);
    chapter_links.reverse();
    for link in chapter_links {
        chapters.push(get_chapter(
            &Url::parse(&link).expect(ErrorMessages::ParseUrl.as_str()),
        ));
    }
    Novel::new(
        get_novel_title(&body),
        get_novel_synopsis(&body),
        get_novel_author(&body),
        get_cover_url(&body),
        input.to_string(),
        chapters,
    )
}

pub(crate) fn get_cover_url(body: &str) -> String {
    let selector = get_selector(COVER_URL);
    Html::parse_fragment(body)
        .select(&selector)
        .flat_map(|el| el.value().attr("src"))
        .collect()
}

pub(crate) fn get_novel_title(body: &str) -> String {
    let title = get_element_content(NOVEL_TITLE, body);
    println!("Fetching: {}", title);
    title.replace("\n", "").replace("\t", "")
}

pub(crate) fn get_novel_synopsis(body: &str) -> Vec<String> {
    get_content(NOVEL_SYNOPSIS, body)
}

pub(crate) fn get_novel_author(body: &str) -> Vec<String> {
    let author = get_content(AUTHOR, body);
    println!("{}", author.join(", "));
    author
}

pub(crate) fn get_chapter_links(body: &str) -> Vec<String> {
    let selector = Selector::parse(CHAPTER_URL).expect(ErrorMessages::ParseSelector.as_str());
    let fragment = Html::parse_document(body);
    let mut links = Vec::<String>::new();
    for el in fragment.select(&selector) {
        links.push(
            el.value()
                .attr("href")
                .expect(ErrorMessages::ParseElementAttribute.as_str())
                .to_owned(),
        );
    }
    links
}

pub(crate) fn get_chapter(url: &Url) -> Chapter {
    let body = get_response(url);
    let title = get_chapter_title(&body);
    println!("{}", title);
    Chapter::new(title, get_chapter_content(&body))
}

pub(crate) fn get_chapter_title(body: &str) -> String {
    get_element_content(CHAPTER_TITLE, body)
        .replace("\n", "")
        .replace("\t", "")
}

pub(crate) fn get_chapter_content(body: &str) -> Vec<String> {
    get_content(CHAPTER_CONTENT, body)
}

pub(crate) fn get_response(url: &Url) -> String {
    thread::sleep(time::Duration::from_millis(REQUEST_DELAY));
    get(url.to_string())
        .expect(ErrorMessages::FetchHtml.as_str())
        .text()
        .expect(ErrorMessages::ParseText.as_str())
}

pub(crate) fn get_element_content(tag: &str, body: &str) -> String {
    let selector = get_selector(tag);
    let fragment = Html::parse_document(body);
    let element_ref = get_element_ref(&selector, &fragment);
    element_ref
        .text()
        .next()
        .expect(ErrorMessages::ParseText.as_str())
        .to_owned()
}

pub(crate) fn fetch_chapter_links(url: &Url) -> Vec<String> {
    let new_url = url
        .join("ajax/chapters")
        .expect(ErrorMessages::JoinUrl.as_str())
        .to_string();
    let client = Client::new();
    let body = client
        .post(&new_url)
        .send()
        .expect(ErrorMessages::SendRequest.as_str())
        .text()
        .expect(ErrorMessages::ParseText.as_str());
    let links = get_chapter_links(&body);
    links
}
