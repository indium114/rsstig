use opml::{OPML, Outline};
use std::fs;

fn home() -> String {
    let dir = dirs::home_dir();
    return dir
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
}

pub fn load_feeds() -> Vec<Outline> {
    let dir = home() + "/.config/rsstig/feeds.opml";
    let file = fs::read_to_string(&dir).expect(&format!("Failed to read {dir}; make sure it exists"));
    OPML::from_str(&file).expect(&format!("Failed to read {dir}; make sure it is a valid opml file")).body.outlines
}
