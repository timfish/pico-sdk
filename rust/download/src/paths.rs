use directories::ProjectDirs;
use std::path::PathBuf;

pub fn get_cache_dir() -> PathBuf {
    let dir = ProjectDirs::from("com", "Pico Technology", "Drivers")
        .expect("Cannot determine cache directory");

    dir.cache_dir().join(std::env::consts::ARCH)
}
