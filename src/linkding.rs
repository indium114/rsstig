/// integration for [linkding]
/// [linkding]: https://linkding.link
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::{Context, Result};

#[derive(Deserialize)]
struct Credentials {
    url: String,
    token: String,
}

#[derive(Serialize)]
struct Bookmark {
    url: String,
    title: String,
    description: String,
    notes: String,
    is_archived: bool,
    unread: bool,
    shared: bool,
    tag_names: Vec<String>,
}

fn load_credentials() -> Result<Credentials, anyhow::Error> {
    let dir = crate::persistence::home() + "/.config/rsstig/credentials.toml";
    let file =
        fs::read_to_string(&dir).with_context(|| "credentials.toml does not exist or is inaccessible".to_string())?;
    toml::from_str(&file).with_context(|| "failed to parse credentials.toml".to_string())
}

pub fn bookmark(url: String, title: String, description: String) -> Result<(), anyhow::Error> {
    let credentials = load_credentials()?;
    let bookmark = Bookmark {
        url,
        title,
        description,
        notes: "Bookmarked by rsstig".to_string(),
        is_archived: false,
        unread: true,
        shared: false,
        tag_names: vec!["rsstig".to_string()],
    };

    let client = reqwest::blocking::Client::new();
    let _ = client
        .post(format!("{}/api/bookmarks/", credentials.url))
        .header("Authorization", format!("Token {}", credentials.token))
        .json(&bookmark)
        .send()?;

    Ok(())
}
