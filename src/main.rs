use clap::Parser;
use bing_wall::cmdline::Args;
use bing_wall::{parse_bing_feed, paths};

#[tokio::main]
async fn main() {
    paths::check_directories().expect("Failed to create application directories");
    let args = Args::parse();
    println!("{:?}", args);
    parse_bing_feed(args).await.unwrap();
}

