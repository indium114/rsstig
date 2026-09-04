use opml::{OPML, Outline};
use std::fs;

fn home() -> String {
    let dir = dirs::home_dir();
    dir.map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default()
}

pub fn load_feeds() -> Vec<Outline> {
    let dir = home() + "/.config/rsstig/feeds.opml";
    let file = fs::read_to_string(&dir)
        .unwrap_or_else(|_| panic!("Failed to read {dir}; make sure it exists"));
    OPML::from_str(&file)
        .unwrap_or_else(|_| panic!("Failed to read {dir}; make sure it is a valid opml file"))
        .body
        .outlines
}
