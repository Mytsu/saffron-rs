mod novel;
mod scrappers;
mod utils;
use serde_json;
use std::fs;
// use reqwest;
// use scraper;

fn main() {
    let url = "https://boxnovel.com/novel/reincarnation-of-the-strongest-sword-god-boxnovel/";
    let novel = scrappers::boxnovel::get_novel(url);
    fs::write("./novel.json", serde_json::to_string_pretty(&novel).unwrap()).unwrap();
    // test();
}

/* 
fn test() {
    let url = "https://boxnovel.com/novel/reincarnation-of-the-strongest-sword-god-boxnovel/ajax/chapters";
    let body = reqwest::blocking::Client::new().post(url).send().unwrap().text().unwrap();
    let fragment = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("li.wp-manga-chapter > a").unwrap();
    let mut links = Vec::<String>::new();
    for el in fragment.select(&selector) {
        links.push(el.value().attr("href").unwrap().to_owned());
    }
    println!("{:?}", links);
}
*/