use gnostr_relay::error::Error;
use pocket_types::{Event, Filter};
use std::env;

fn main() -> Result<(), Error> {
    // Get args (config path)
    let mut args = env::args();
    if args.len() <= 1 {
        panic!("USAGE: gnostr_relay_dump <gnostr_relay_config_path>");
    }
    let _ = args.next(); // ignore program name
    let config_path = args.next().unwrap();

    let mut config = gnostr_relay::load_config(config_path)?;

    // Force allow of scraping (this program is a scraper)
    config.allow_scraping = true;

    gnostr_relay::setup_logging(&config);

    // Setup store
    let store = gnostr_relay::setup_store(&config)?;

    let mut buffer: [u8; 128] = [0; 128];
    let (_incount, _outcount, filter) = Filter::from_json(b"{}", &mut buffer)?;
    let screen = |_: &Event| -> bool { true };

    let mut events = store.find_events(
        filter,
        config.allow_scraping,
        config.allow_scrape_if_limited_to,
        config.allow_scrape_if_max_seconds,
        screen,
    )?;

    for event in events.drain(..) {
        let bytes = event.as_json()?;
        let s = unsafe { std::str::from_utf8_unchecked(&bytes) };
        println!("{s}");
    }

    Ok(())
}
