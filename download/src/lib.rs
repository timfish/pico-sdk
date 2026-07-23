#![forbid(unsafe_code)]

//! Downloads Pico Technology oscilloscope drivers.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! This helper crate allows the distribution of software that can communicate with every Pico
//! oscilloscope without the need to ship every driver binary.
//!
//! Supported platforms:
//!  - Windows (x86, x86_64 or aarch64)
//!  - macOS (x86_64 or aarch64)
//!  - Linux (x86_64, aarch64 or arm for Raspberry Pi)
//!
//! Pico do not build every driver for every platform, so use [`driver_binary`] or
//! [`available_drivers`] if you need to know up front whether a particular driver can be
//! downloaded here.
//!
//! Each version of this crate is built against one *bundle* of drivers, extracted from Pico's
//! public installers by the `generate-manifest` example. [`BUNDLE`] identifies it, and every
//! binary in that bundle is listed in [`BINARIES`] along with its upstream version and SHA-256
//! hash. Those hashes are compiled into the crate, so a download that has been tampered with in
//! transit is rejected.
//!
//! The bundle identifier is also part of the cache path, which means downloaded drivers are
//! immutable: two applications built against different versions of this crate get their own
//! cache directory rather than overwriting each other's drivers.
//!
//! [`download_drivers_to_cache`] downloads the passed drivers and [`cache_resolution`] returns
//! a `Resolution` that can be used to resolve them.
//!
//! `let enumerator = DeviceEnumerator::with_resolution(cache_resolution());`

use http_req::{request::Request, uri::Uri};
use pico_common::Driver;
use pico_driver::LibraryResolution;
use ring::digest::{Context, SHA256};
use std::{
    fmt::Write as _,
    fs,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};
use thiserror::Error;

mod manifest;
mod paths;

pub use manifest::{lookup as driver_binary, DriverBinary, BASE_URL, BINARIES, BUNDLE};
pub use paths::get_cache_dir;

/// Release assets are served from object storage behind a redirect
const MAX_REDIRECTS: usize = 5;

/// Overrides [`BASE_URL`], for mirroring the drivers somewhere else
///
/// Only where they're fetched from changes. Binaries still have to hash to what the manifest
/// says, so a mirror can't substitute a different driver.
pub const BASE_URL_ENV_VAR: &str = "PICO_DRIVERS_BASE_URL";

#[derive(Error, Debug)]
pub enum DriverDownloadError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP request Error: {0}")]
    HttpRequestError(#[from] http_req::error::Error),

    #[error("Invalid URL: {0}")]
    UrlParseError(#[from] http_req::error::ParseErr),

    #[error("HTTP response Error: {0}")]
    HttpResponseError(http_req::response::StatusCode),

    #[error("Too many redirects while downloading {0}")]
    TooManyRedirects(String),

    #[error("{driver} is not published for {os}-{arch} in driver bundle {bundle}")]
    Unavailable {
        driver: Driver,
        os: &'static str,
        arch: &'static str,
        bundle: &'static str,
    },

    #[error(
        "Downloaded {driver} does not match its expected hash (expected {expected}, got {actual})"
    )]
    HashMismatch {
        driver: Driver,
        expected: &'static str,
        actual: String,
    },
}

/// Gets a `Resolution` that points to the download cache location
pub fn cache_resolution() -> LibraryResolution {
    LibraryResolution::Custom(get_cache_dir())
}

/// Downloads the requested drivers to the supplied path.
///
/// Drivers already present at `path` are left alone, so this is cheap to call on every start
/// up. Returns [`DriverDownloadError::Unavailable`] if Pico do not publish one of the drivers
/// for the current platform.
pub fn download_drivers<P: AsRef<Path>>(
    drivers: &[Driver],
    path: P,
) -> Result<(), DriverDownloadError> {
    let driver_dir = path.as_ref();

    fs::create_dir_all(driver_dir)?;

    // Only ps4000 and ps6000 have a runtime dependency (picoipp), so gather the dependencies of
    // just the requested drivers rather than fetching picoipp for everything.
    let mut dependencies = std::collections::BTreeSet::new();
    for &driver in drivers {
        download_driver(driver, driver_dir)?;
        dependencies.extend(driver.dependencies().iter().copied());
    }

    // A needed dependency has to sit alongside its driver in the cache. Skipped on targets where
    // Pico don't publish it (e.g. Linux armhf), where the driver loads without it.
    for dependency in dependencies {
        if driver_binary(dependency, std::env::consts::OS, std::env::consts::ARCH).is_some() {
            download_driver(dependency, driver_dir)?;
        }
    }

    Ok(())
}

/// Downloads the requested drivers to the cache location.
///
/// Drivers downloaded using this function can be loaded using the `Resolution`
/// returned from `cache_resolution()`.
///
/// `DeviceEnumerator::with_resolution(cache_resolution())`
pub fn download_drivers_to_cache(drivers: &[Driver]) -> Result<(), DriverDownloadError> {
    download_drivers(drivers, get_cache_dir())
}

/// Every driver this crate can download for the platform it was built for.
pub fn available_drivers() -> Vec<Driver> {
    BINARIES
        .iter()
        .filter(|binary| binary.os == std::env::consts::OS && binary.arch == std::env::consts::ARCH)
        .map(|binary| binary.driver)
        .collect()
}

/// The path a driver is downloaded to in the cache, whether or not it's there yet.
pub fn cached_driver_path(driver: Driver) -> Result<PathBuf, DriverDownloadError> {
    Ok(get_cache_dir().join(binary_for_this_platform(driver)?.file_name))
}

/// Where a binary is downloaded from.
pub fn download_url(binary: &DriverBinary) -> String {
    let base = std::env::var(BASE_URL_ENV_VAR).unwrap_or_else(|_| BASE_URL.to_string());

    format!(
        "{}/{}/{}",
        base.trim_end_matches('/'),
        BUNDLE,
        binary.asset_name
    )
}

fn download_driver(driver: Driver, driver_dir: &Path) -> Result<(), DriverDownloadError> {
    let binary = binary_for_this_platform(driver)?;
    let dst_path = driver_dir.join(binary.file_name);

    if is_cached(&dst_path, binary) {
        return Ok(());
    }

    // Downloading to a process specific temporary name and publishing by rename means a
    // partly downloaded driver is never visible under the name drivers are loaded from, even
    // if several processes populate the same cache at once
    let tmp_path = driver_dir.join(format!("{}.{}.tmp", binary.file_name, std::process::id()));

    let result = download_verified(binary, &tmp_path).and_then(|_| {
        match fs::rename(&tmp_path, &dst_path) {
            Ok(()) => Ok(()),
            // Another process may have got there first, and on Windows we cannot replace a
            // driver that something currently has loaded. Either way, if the destination now
            // holds this binary there is nothing to fix.
            Err(_) if is_cached(&dst_path, binary) => Ok(()),
            Err(error) => Err(error.into()),
        }
    });

    let _ = fs::remove_file(&tmp_path);

    result
}

/// Looks up the binary published for `driver` on the platform we're running on.
fn binary_for_this_platform(driver: Driver) -> Result<&'static DriverBinary, DriverDownloadError> {
    driver_binary(driver, std::env::consts::OS, std::env::consts::ARCH).ok_or(
        DriverDownloadError::Unavailable {
            driver,
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
            bundle: BUNDLE,
        },
    )
}

/// Whether `path` already holds this binary.
///
/// Only the size is checked. Cache directories are keyed by bundle and nothing in them is ever
/// modified in place, so re-hashing megabytes of driver on every start up would buy nothing.
/// Downloads are still hashed on the way in.
fn is_cached(path: &Path, binary: &DriverBinary) -> bool {
    fs::metadata(path)
        .map(|meta| meta.is_file() && meta.len() == binary.size)
        .unwrap_or(false)
}

/// Downloads a binary to `tmp_path`, failing if it doesn't hash to what the manifest expects.
fn download_verified(binary: &DriverBinary, tmp_path: &Path) -> Result<(), DriverDownloadError> {
    let mut tmp_file = fs::File::create(tmp_path)?;
    download_to(&download_url(binary), &mut tmp_file)?;
    drop(tmp_file);

    let actual = sha256_hex_for_file(tmp_path)?;

    if actual == binary.sha256 {
        Ok(())
    } else {
        Err(DriverDownloadError::HashMismatch {
            driver: binary.driver,
            expected: binary.sha256,
            actual,
        })
    }
}

/// Fetches `url` into `file`, following redirects.
fn download_to(url: &str, file: &mut fs::File) -> Result<(), DriverDownloadError> {
    let mut url = url.to_string();

    for _ in 0..MAX_REDIRECTS {
        // `http_req` writes the body of every response it gets, so rewind to keep a redirect
        // page from being prepended to the driver
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;

        let uri: Uri = url.parse()?;
        let response = Request::new(&uri).send(file)?;
        let status = response.status_code();

        if status.is_redirect() {
            match response.headers().get("Location") {
                Some(location) => {
                    url = location.to_string();
                    continue;
                }
                None => return Err(DriverDownloadError::HttpResponseError(status)),
            }
        }

        return if status.is_success() {
            Ok(())
        } else {
            Err(DriverDownloadError::HttpResponseError(status))
        };
    }

    Err(DriverDownloadError::TooManyRedirects(url))
}

fn sha256_hex_for_file<P: AsRef<Path>>(path: P) -> Result<String, DriverDownloadError> {
    let mut src_file = fs::File::open(&path)?;

    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 8192];

    loop {
        let count = src_file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    let mut hex = String::with_capacity(64);
    for byte in context.finish().as_ref() {
        let _ = write!(hex, "{:02x}", byte);
    }

    Ok(hex)
}
