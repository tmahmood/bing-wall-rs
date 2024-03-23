use serde::{Deserialize, Serialize};
use crate::cmdline::Args;

#[derive(Deserialize, Serialize, Debug)]
pub struct Tooltips {
    pub loading: String,
    pub previous: String,
    pub next: String,
    pub walle: String,
    pub walls: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BingImage {
    pub startdate: String,
    pub fullstartdate: String,
    pub enddate: String,
    pub url: String,
    pub urlbase: String,
    pub copyright: String,
    pub copyrightlink: String,
    pub title: String,
    pub quiz: String,
    pub wp: bool,
    pub hsh: String,
    pub drk: i64,
    pub top: i64,
    pub bot: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BingWallpaperFeed {
    pub images: Vec<BingImage>,
    pub tooltips: Tooltips,
}
pub const WALLPAPER_URL_BASE:&str = "https://www.bing.com";
pub const FEED_URL: &str = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=__IDX__&n=__HOW_MANY__&mkt=__MKT__";
pub const RESOLUTIONS: [&str; 7] = ["UHD", "1920x1200", "1920x1080", "1366x768", "1280x720", "1024x768", "800x600"];

pub fn build_url(args: &Args) -> String {
    FEED_URL
        .replace("__IDX__", &args.idx().to_string())
        .replace("__HOW_MANY__", &args.how_many().to_string())
        .replace("__MKT__", &args.mkt())
}
