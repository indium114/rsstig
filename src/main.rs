use color_eyre::eyre::Result;

mod opml;

fn main() -> Result<()> {
    color_eyre::install()?;

    for feed in opml::load_feeds() {
        println!("Feed: {}", feed.text);
        println!("  > Description : {}", feed.description.unwrap_or("<no description>".to_string()).trim());
    }

    Ok(())
}
