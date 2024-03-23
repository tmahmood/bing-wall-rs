use directories::BaseDirs;
use std::path::PathBuf;
use std::fs;
use tracing::{debug, info};
use crate::{APP_DIR, APP_GROUP};

pub fn check_directories() -> Result<(), anyhow::Error> {
    let dirs = vec![cache_dir(), config_dir()];
    for mut app_dir in dirs {
        debug!("dir: {:?}", app_dir);
        app_dir.push(APP_GROUP);
        app_dir.push(APP_DIR);
        if !app_dir.exists() {
            info!("creating: {:?}", app_dir);
            fs::create_dir_all(&app_dir).unwrap();
        }
    }
    let pic = pictures();
    if !pic.exists() {
        fs::create_dir_all(pic).unwrap();
    }
    Ok(())
}

pub fn cache_dir() -> PathBuf {
    match BaseDirs::new() {
        None => {
            PathBuf::from(".app_dir/cache")
        }
        Some(bd) => {
            PathBuf::from(bd.cache_dir())
        }
    }
}

pub fn config_dir() -> PathBuf {
    match BaseDirs::new() {
        None => {
            PathBuf::from(".app_dir/config")
        }
        Some(bd) => {
            PathBuf::from(bd.config_dir())
        }
    }
}

pub fn pictures() -> PathBuf {
    match BaseDirs::new() {
        None => {
            PathBuf::from(".app_dir/Pictures")
        }
        Some(bd) => {
            let mut pb = PathBuf::from(bd.home_dir());
            pb.push("Pictures");
            pb
        }
    }
}
