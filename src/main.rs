use color_eyre::eyre::Result;

mod opml;
mod rss;

struct Feed {
    name: String,
    entries: Vec<rss::Entry>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut feeds: Vec<Feed> = Vec::new();
    for feed in opml::load_feeds() {
        feeds.push(Feed {
            name: feed.text.clone(),
            entries: rss::get_rss(feed.xml_url.unwrap_or_else(|| {
                panic!(
                    "{}",
                    format!("Feed {} does not have an xmlUrl", feed.text).to_string()
                )
            })),
        })
    }

    Ok(())
}
