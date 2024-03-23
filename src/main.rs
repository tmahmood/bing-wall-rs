use clap::Parser;
use tracing::debug;
use bing_wall::cmdline::Args;
use bing_wall::{parse_bing_feed, paths, set_tracing};

#[tokio::main]
async fn main() {
    set_tracing().expect("Failed to setup tracing");
    paths::check_directories().expect("Failed to create application directories");
    let args = Args::parse();
    debug!("{:?}", args);
    parse_bing_feed(args).await.unwrap();
}

