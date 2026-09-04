use color_eyre::eyre::Result;
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::fmt::Write;

mod opml;
mod rss;

struct Feed {
    name: String,
    entries: Vec<rss::Entry>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut feeds: Vec<Feed> = Vec::new();
    let feed_files = opml::load_feeds();

    let bar = ProgressBar::new(feed_files.len() as u64);
    bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.white}] ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    for feed in feed_files {
        feeds.push(Feed {
            name: feed.text.clone(),
            entries: rss::get_rss(feed.xml_url.unwrap_or_else(|| {
                panic!(
                    "{}",
                    format!("Feed {} does not have an xmlUrl", feed.text).to_string()
                )
            })),
        });

        bar.inc(1);
    }

    bar.finish();

    Ok(())
}
