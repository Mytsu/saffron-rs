mod scrappers;
mod utils;
use clap::Parser;
use scrappers::boxnovel;
use serde_json;
use std::fs;
use url::Url;
use utils::errors::ErrorMessages;

#[derive(Parser, Debug)]
#[clap(
    author = "mytsu",
    version = "0.2.0",
    about = "Saffron is a web scraper that downloads novels from supported domains."
)]
struct Args {
    #[arg(short, long)]
    url: String,
    #[arg(short, long, default_value_t = false)]
    chapter: bool,
}

fn scrap(url: &Url, args: &Args) {
    if args.chapter {
        let chapter = boxnovel::get_chapter(&url);
        fs::write(
            format!("./{}.json", chapter.title),
            serde_json::to_string_pretty(&chapter).expect("Failed to parse novel to json"),
        )
        .expect("Failed to write file");
        return ();
    }
    let novel = boxnovel::get_novel(&url);
    fs::write(
        format!("./{}.json", novel.title),
        serde_json::to_string_pretty(&novel).expect("Failed to parse novel to json"),
    )
    .expect("Failed to write file");
}

fn main() {
    let args = Args::parse();
    let parsed_url = Url::parse(&args.url).expect(ErrorMessages::ParseUrl.as_str());
    let hostname = parsed_url
        .host_str()
        .expect(ErrorMessages::ParseHostname.as_str());
    match hostname {
        boxnovel::HOSTNAME => scrap(&parsed_url, &args),
        _ => {
            println!("{}", ErrorMessages::IncompatibleDomain.as_str());
        }
    }
}
