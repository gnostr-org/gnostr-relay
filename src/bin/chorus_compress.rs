use gnostr_relay::error::Error;
use std::env;
use std::io::BufRead;
fn main() -> Result<(), Error> {
    // Get args (config path)
    let mut args = env::args();
    if args.len() <= 1 {
        panic!("USAGE: gnostr_relay_compress <config_path>");
    }
    let _ = args.next(); // ignore program name
    let config_path = args.next().unwrap();

    let config = gnostr_relay::load_config(config_path)?;

    gnostr_relay::setup_logging(&config);

    println!("gnostr_relay must NOT be running when you do this.");
    println!("Proceed? (break out with ^C, or press <ENTER> to proceed)");
    let stdin = std::io::stdin();
    let _ = stdin.lock().lines().next().unwrap().unwrap();

    let store = gnostr_relay::setup_store(&config)?;
    let pre_stats = store.stats()?;
    println!("{:?}", pre_stats);

    let store = unsafe { store.rebuild()? };
    let post_stats = store.stats()?;
    println!("{:?}", post_stats);

    Ok(())
}
