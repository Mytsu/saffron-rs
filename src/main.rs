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
}

fn main() {
    let args = Args::parse();
    let parsed_url = Url::parse(&args.url).expect(ErrorMessages::ParseUrl.as_str());
    let hostname = parsed_url
        .host_str()
        .expect(ErrorMessages::ParseHostname.as_str());
    match hostname {
        boxnovel::HOSTNAME => {
            let novel = boxnovel::get_novel(&args.url);
            fs::write(
                "./novel.json",
                serde_json::to_string_pretty(&novel).expect("Failed to parse novel to json"),
            )
            .expect("Failed to write file");
        }
        _ => {
            println!("{}", ErrorMessages::IncompatibleDomain.as_str());
        }
    }
}
