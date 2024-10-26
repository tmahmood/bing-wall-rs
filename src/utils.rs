use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tracing::debug;

pub fn is_cached_file_outdated(before_date: u64, path: &PathBuf) -> bool {
    let now = SystemTime::now();
    let last_changed = path.metadata().unwrap()
        .modified().unwrap();
    debug!("is_cached_file_outdated: {:?}", last_changed);
    match now.duration_since(last_changed) {
        Ok(e) => e.as_secs() > before_date,
        Err(_) => true
    }
}

pub fn timestamp() -> Duration {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
}