use crate::manifest::BUNDLE;
use directories::ProjectDirs;
use std::path::PathBuf;

/// The cache directory for the drivers of this bundle and platform
///
/// - The path is `<cache>/<BUNDLE>/<os>-<arch>`.
/// - `<cache>` is `~/.cache/drivers` on Linux, `~/Library/Caches/com.Pico-Technology.Drivers` on macOS, and `%LOCALAPPDATA%\Pico Technology\Drivers\cache` on Windows.
///   On Linux, `$XDG_CACHE_HOME` replaces `~/.cache` when it is set.
/// - Two applications that use different bundles thus get different directories.
/// - It does not make the directory.
/// - Panics when the operating system gives no home directory.
pub fn get_cache_dir() -> PathBuf {
    let dir = ProjectDirs::from("com", "Pico Technology", "Drivers")
        .expect("Cannot determine cache directory");

    dir.cache_dir().join(BUNDLE).join(format!(
        "{}-{}",
        std::env::consts::OS,
        std::env::consts::ARCH
    ))
}
