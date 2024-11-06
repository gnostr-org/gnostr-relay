use gnostr_relay::error::Error;
use std::env;

fn main() -> Result<(), Error> {
    // Get args (config path)
    let mut args = env::args();
    if args.len() <= 1 {
        panic!("USAGE: gnostr_relay_moderate <config_path>");
    }
    let _ = args.next(); // ignore program name
    let config_path = args.next().unwrap();

    let mut config = gnostr_relay::load_config(config_path)?;

    // Force allow of scraping (this program is a scraper)
    config.allow_scraping = true;

    gnostr_relay::setup_logging(&config);

    // Setup store
    let store = gnostr_relay::setup_store(&config)?;

    for (id, approved) in gnostr_relay::dump_event_approvals(&store)? {
        println!("ID {} = {}", id, approved);
    }

    for (pubkey, approved) in gnostr_relay::dump_pubkey_approvals(&store)? {
        println!("PUBKEY {} = {}", pubkey, approved);
    }

    Ok(())
}
