use crate::manifest::BUNDLE;
use directories::ProjectDirs;
use std::path::PathBuf;

/// The directory drivers are downloaded into for this crate version and platform.
///
/// The bundle identifier is part of the path, so a machine running two applications built
/// against different versions of this crate gets a cache directory for each rather than the
/// two of them fighting over one set of files.
pub fn get_cache_dir() -> PathBuf {
    let dir = ProjectDirs::from("com", "Pico Technology", "Drivers")
        .expect("Cannot determine cache directory");

    dir.cache_dir().join(BUNDLE).join(format!(
        "{}-{}",
        std::env::consts::OS,
        std::env::consts::ARCH
    ))
}
