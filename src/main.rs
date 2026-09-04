use color_eyre::eyre::Result;

mod opml;
mod rss;

fn main() -> Result<()> {
    color_eyre::install()?;

    for feed in opml::load_feeds() {
        println!("== Feed: {}", feed.text);
        println!(
            "{:#?}",
            rss::get_rss(
                feed.xml_url
                    .unwrap_or_else(|| panic!("{}", format!("Feed {} does not have an xmlUrl", feed.text).to_string()))
            )
        );
    }

    Ok(())
}
