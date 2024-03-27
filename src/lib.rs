use std::fs;
use std::path::PathBuf;
use clap::Parser;
use serde::de::DeserializeOwned;
use tokio::task::JoinSet;
use tracing::{debug, info, trace, warn};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use crate::bing_wall_feed::{BingWallpaperFeed, WALLPAPER_URL_BASE};
use crate::cmdline::Args;
use crate::paths::cache_dir;
use crate::utils::is_cached_file_outdated;

pub const ONE_DAY_IN_SECS: u64 = 60 * 60 * 24;

pub mod bing_wall_feed;
pub mod cmdline;
pub mod utils;
pub mod paths;

pub const APP_GROUP: &str = "org.tm";
pub const APP_DIR: &str = "bing-wall-rs";

/// Sets up simple enviornment based tracing.
/// DO NOT call from library. Always from bin
pub fn set_tracing() -> Result<(), anyhow::Error> {
    let subscriber = FmtSubscriber::builder()
        .compact()
        .with_line_number(true)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

pub fn file_in_cache(file_name: &str) -> PathBuf {
    let mut f = cache_dir();
    f.push(file_name);
    f
}

pub async fn download_file(url: String, file_path: PathBuf) -> Result<Vec<u8>, anyhow::Error> {
    trace!("downloading: {} to: {:?}", url, file_path);
    if file_path.exists() {
        if !is_cached_file_outdated(ONE_DAY_IN_SECS, &file_path) {
            debug!("recent cache exists, not downloading");
            return Ok(fs::read(&file_path)?);
        }
        debug!("removing old cache {:?}", file_path);
        // old file, remove
        fs::remove_file(&file_path)?;
    }
    let resp = reqwest::get(url).await?.bytes().await?;
    fs::write(&file_path, &resp)?;
    Ok(resp.to_vec())
}

pub async fn download_json<T: DeserializeOwned>(url: &str, file_path: &PathBuf) -> Result<T, anyhow::Error> {
    debug!("{} {:?}", url, file_path);
    let c = download_file(url.to_string(), file_path.clone()).await?;
    let s = String::from_utf8(c)?;
    Ok(serde_json::from_str(&s).unwrap())
}

pub async fn parse_bing_feed(args: Args) -> Result<(), anyhow::Error> {
    let url = crate::bing_wall_feed::build_url(&args);
    let p = file_in_cache("bing-feed.json");
    let resp: BingWallpaperFeed = download_json(&url, &p).await?;
    let s = args.save_to();
    let mut hd = JoinSet::new();
    for image in resp.images.iter() {
        let img_url = format!("{}{}_{}.jpg", WALLPAPER_URL_BASE, image.urlbase, args.resolution());
        let file_name = format!("{}-{}", image.startdate, filename_from_img_url(&img_url));
        let mut pp = s.clone();
        pp.push(file_name);
        debug!("path: {:?}", pp);
        hd.spawn(download_file(img_url, pp));
    }
    let mut good = 0;
    let mut fail = 0;
    loop {
        let rs = hd.join_next().await;
        if rs.is_none() {
            break
        } else {
            if let Some(Ok(_img)) = rs {
                info!("Success");
                good += 1;
            } else {
                warn!("Failed to download a image");
                fail += 1;
            }
        }
    }
    info!("Download finish, success: {}, failed: {}", good, fail);
    Ok(())
}

pub fn filename_from_img_url(url: &str) -> String {
    url.split("=").last().unwrap().replace("OHR.", "")
}
