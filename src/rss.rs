use markdowndown::{convert_url, HtmlConverter};
use feed_rs::parser;

#[derive(Debug)]
pub struct Entry {
    id: String,
    title: String,
    content: String,
}

async fn get(url: &str) -> String {
    convert_url(url).await.unwrap().to_string()
}

pub fn get_rss(url: String) -> Vec<Entry> {
    let feed = reqwest::blocking::get(&url).unwrap().text().unwrap();
    parser::parse(feed.as_bytes()).unwrap().entries.into_iter().map(|e| Entry {
        id: e.id,
        title: match e.title {
            Some(t) => t.content.to_string(),
            None => "No title".to_string(),
        },
        content: match e.summary {
            Some(s) => HtmlConverter::default().convert_html(&s.content).unwrap(),
            None => "No summary".to_string(),
        }
    }).collect()
}
