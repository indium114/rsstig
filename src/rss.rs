use feed_rs::parser;
use markdowndown::{HtmlConverter, convert_url};

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub content: String,
    pub url: String,
}

pub async fn get(url: &str) -> String {
    convert_url(url).await.unwrap().to_string()
}

pub fn get_rss(url: String) -> Result<Vec<Entry>, anyhow::Error> {
    let feed = reqwest::blocking::get(&url)?.text()?;
    let result = parser::parse(feed.as_bytes())?
        .entries
        .into_iter()
        .map(|e| Entry {
            id: e.id,
            title: match e.title {
                Some(t) => t.content.to_string(),
                None => "<No title>".to_string(),
            },
            content: match e.summary {
                Some(s) => HtmlConverter::default().convert_html(&s.content).unwrap(),
                None => "<No summary>".to_string(),
            },
            url: e.links[0].href.clone(),
        })
        .collect();

    Ok(result)
}
