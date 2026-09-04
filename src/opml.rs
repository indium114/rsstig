use opml::{OPML, Outline};
use std::fs;

pub fn load_feeds() -> Vec<Outline> {
    let dir = crate::persistence::home() + "/.config/rsstig/feeds.opml";
    let file = fs::read_to_string(&dir)
        .unwrap_or_else(|_| panic!("Failed to read {dir}; make sure it exists"));
    OPML::from_str(&file)
        .unwrap_or_else(|_| panic!("Failed to read {dir}; make sure it is a valid opml file"))
        .body
        .outlines
}
