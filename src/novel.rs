use chrono::Local;
use serde::Serialize;
use url::Url;

use crate::utils::errors::ErrorMessages;

#[derive(Debug, Serialize)]
pub struct Novel {
    title: String,
    synopsis: String,
    date: String,
    authors: String,
    cover_url: String,
    url: String,
    chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize)]
pub struct Chapter {
    title: String,
    content: String,
}

impl Novel {
    pub fn new(
        title: String,
        synopsis: Vec<String>,
        authors: Vec<String>,
        cover_url: String,
        url: String,
        chapters: Vec<Chapter>,
    ) -> Novel {
        Novel {
            title,
            synopsis: synopsis.join("\n\n"),
            date: Local::now().to_rfc3339(),
            authors: authors.join(", "),
            cover_url: Url::parse(&cover_url)
                .expect(ErrorMessages::ParseUrl.as_str())
                .as_str()
                .to_owned(),
            url: Url::parse(&url)
                .expect(ErrorMessages::ParseUrl.as_str())
                .as_str()
                .to_owned(),
            chapters,
        }
    }
}

impl Chapter {
    pub fn new(title: String, content: Vec<String>) -> Chapter {
        Chapter {
            title,
            content: content.join("\n\n"),
        }
    }
}
