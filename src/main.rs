mod novel;
mod scrappers;
mod utils;
use serde_json;
use std::fs;

fn main() {
    let url = "https://boxnovel.com/novel/reincarnation-of-the-strongest-sword-god-boxnovel/";
    let novel = scrappers::boxnovel::get_novel(url);
    fs::write(
        "./novel.json",
        serde_json::to_string_pretty(&novel).expect("Failed to parse novel to json"),
    )
    .expect("Failed to write file");
}
