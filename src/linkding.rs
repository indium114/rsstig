/// integration for [linkding]
/// [linkding]: https://linkding.link
use serde::{Deserialize, Serialize};
use std::fs;

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

fn load_credentials() -> Credentials {
    let dir = crate::persistence::home() + "/.config/rsstig/credentials.toml";
    let file =
        fs::read_to_string(&dir).expect(&format!("Failed to read {dir}, make sure it exists"));
    toml::from_str(&file).expect(&format!("Failed to read {dir}, make sure it is valid TOML with a 'url' field, a 'username' field, and a 'token' field"))
}

pub fn bookmark(url: String, title: String, description: String) {
    let credentials = load_credentials();
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
        .send()
        .unwrap();
}
