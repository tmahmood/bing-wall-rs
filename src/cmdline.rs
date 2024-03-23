use std::path::PathBuf;
use clap::Parser;
use crate::paths::pictures;

/// Simple wallpaper to download daily bing wallpaper
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    resolution: Option<String>,

    #[arg(short, long, default_value_t=1)]
    n: i32,

    #[arg(short, long, default_value_t=0)]
    idx: i32,

    #[arg(short, long)]
    mkt: Option<String>,

    #[arg(short, long)]
    save_to: Option<PathBuf>,
}

impl Args {
    pub(crate) fn resolution(&self) -> String {
        self.resolution.clone().unwrap_or("UHD".to_string())
    }
    pub fn how_many(&self) -> i32 {
        self.n
    }
    pub fn idx(&self) -> i32 {
        self.idx
    }
    pub fn mkt(&self) -> String {
        self.mkt.clone().unwrap_or("en-US".to_string())
    }
    pub fn save_to(&self) -> PathBuf {
        self.save_to.clone().unwrap_or_else(|| {
            let mut p = pictures();
            p.push("BingWallpaper");
            p
        })
    }
}
