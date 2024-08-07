use clap::Parser;

#[derive(Parser)]
#[command(about = "gnostr: git+nostr rust relay", author = env!("CARGO_PKG_AUTHORS"), version = env!("CARGO_PKG_VERSION"))]
pub struct CLIArgs {
    #[arg(
        short,
        long,
        help = "Use the <port> as the listening port <u16>",
        required = false
    )]
    pub port: Option<u16>,
    #[arg(
        short,
        long,
        help = "Use the <directory> as the location of the database",
        required = false
    )]
    pub db: Option<String>,
    #[arg(
        short,
        long,
        help = "Use the <file name> as the location of the config file",
        required = false
    )]
    pub config: Option<String>,
    #[arg(
        long,
        help = "Display the gnostr-relay-dashboard",
        required = false
    )]
    pub dashboard: bool,
}
