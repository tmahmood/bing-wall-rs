use std::fs;
use std::path::PathBuf;
use clap::Parser;
use serde::de::DeserializeOwned;
use crate::bing_wall_feed::{BingWallpaperFeed, WALLPAPER_URL_BASE};
use crate::cmdline::Args;
use crate::utils::is_cached_file_outdated;
pub const ONE_DAY_IN_SECS: u64 = 60 * 60 * 24;
pub mod bing_wall_feed;
pub mod cmdline;
pub mod utils;
pub mod paths;
pub const APP_GROUP: &str = "org.tm";
pub const APP_DIR: &str = "bing-wall-rs";

pub fn app_path() -> PathBuf {
    PathBuf::from("org.tm/bing-wall-rs")
}

pub fn file_in_cache(file_name: &str) -> PathBuf {
    let mut f = app_path();
    f.push(file_name);
    f
}

pub async fn download_file(url: &str, file_path: &PathBuf) -> Result<Vec<u8>, anyhow::Error> {
    if file_path.exists() {
        if !is_cached_file_outdated(ONE_DAY_IN_SECS, file_path) {
            return Ok(fs::read(file_path)?)
        }
        // old file, remove
        fs::remove_file(file_path)?;
    }
    let resp = reqwest::get(url).await?.bytes().await?;
    fs::write(file_path, &resp)?;
    Ok(resp.to_vec())
}

pub async fn download_json<T: DeserializeOwned>(url: &str, file_path: &PathBuf) -> Result<T, anyhow::Error> {
    let c = download_file(url, file_path).await?;
    let s = String::from_utf8(c)?;
    Ok(serde_json::from_str(&s).unwrap())
}

pub async fn parse_bing_feed(args: Args) -> Result<(), anyhow::Error> {
    let url = crate::bing_wall_feed::build_url(&args);
    let p = PathBuf::from("bing-feed.json");
    let resp: BingWallpaperFeed = download_json(&url, &p).await?;
    let s = args.save_to();
    for image in resp.images.iter() {
        let img_url = format!("{}{}_{}.jpg", WALLPAPER_URL_BASE, image.urlbase, args.resolution());
        let file_name = format!("{}-{}", image.startdate, filename_from_img_url(&img_url));
        let mut pp = s.clone();
        pp.push(file_name);
        download_file(&img_url, &pp).await.unwrap();
    }
    Ok(())
}

pub fn filename_from_img_url(url: &str) -> String {
    url.split("=").last().unwrap().replace("OHR.", "")
}
