use color_eyre::eyre::Result;

mod opml;

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("{:#?}", opml::load_feeds());

    Ok(())
}
