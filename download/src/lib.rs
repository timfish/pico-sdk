#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

//! Downloads Pico Technology oscilloscope and data logger drivers.
//!
//! With this crate, your software can use every Pico instrument, but you do not ship the driver binaries with it.
//! It downloads the drivers that you ask for to a cache directory, and it does not download them again.
//! It covers every driver this SDK wraps: the oscilloscopes, the data loggers (usbtc08, pl1000, picohrdl, usbpt104, usbdrdaq, plcm3) and `picoipp`, which ps4000 and ps6000 load at runtime.
//!
//! # Quick start
//!
//! ```no_run
//! use pico_common::Driver;
//! use pico_download::{available_drivers, cache_resolution, download_drivers_to_cache};
//! use pico_driver::DriverLoad;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let available = available_drivers();
//! let drivers: Vec<Driver> = [Driver::PS2000A, Driver::TC08]
//!     .into_iter()
//!     .filter(|driver| available.contains(driver))
//!     .collect();
//!
//! // Downloads only the drivers that are not in the cache.
//! download_drivers_to_cache(&drivers)?;
//!
//! for driver in drivers {
//!     driver.load(&cache_resolution())?;
//!     println!("loaded {driver}");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # How the parts fit
//!
//! ```text
//!  &[Driver]                      the drivers you ask for
//!         │
//!         ▼
//!  download_drivers_to_cache()    adds picoipp for ps4000 and ps6000
//!         │
//!         ▼
//!  driver_binary() ──► DriverBinary   file name, size and SHA-256 for this platform
//!         │
//!         ▼
//!  download_url()                 BASE_URL or PICO_DRIVERS_BASE_URL, then BUNDLE
//!         │
//!         ▼
//!  SHA-256 check                  a bad file gives DriverDownloadError::HashMismatch
//!         │
//!         ▼
//!  get_cache_dir()                one directory for each bundle and platform
//!         │
//!         ▼
//!  cache_resolution() ──► LibraryResolution   pico_driver loads the drivers from here
//! ```
//!
//! # Platforms
//!
//! - Windows: x86, x86_64 and aarch64.
//! - macOS: x86_64 and aarch64.
//! - Linux: x86_64, aarch64 and arm (Raspberry Pi).
//!
//! Pico do not build every driver for every platform.
//! Use [`available_drivers`] or [`driver_binary`] to find if a driver is available.
//!
//! - Windows x86_64, macOS x86_64 and Linux x86_64 have every driver.
//! - Windows x86 has no psospa.
//! - Windows aarch64, macOS aarch64 and Linux aarch64 have no ps4000, ps6000, usbdrdaq or picoipp.
//! - Linux arm has ps2000, ps4000, ps6000 and the six data logger drivers only.
//!
//! # Bundles and the cache
//!
//! Each version of this crate uses one *bundle* of drivers, which comes from the public Pico installers.
//! `BUNDLE` is the name of the bundle.
//! `BINARIES` lists each binary in the bundle, with its Pico version and its SHA-256 hash.
//! The hashes are in the compiled crate, so the crate does not accept a changed download.
//!
//! The bundle name is part of the cache path.
//! So two applications that use different versions of this crate get different cache directories, and they do not change the files of each other.
//!
//! The binaries are assets of a [GitHub release](https://github.com/timfish/pico-sdk/releases), with one release for each bundle.
//! Set the `PICO_DRIVERS_BASE_URL` environment variable to download them from a mirror.
//! The crate still checks each file against its hash.
//!
//! # Main types
//!
//! - [`download_drivers_to_cache`], [`download_drivers`]: download drivers and their dependencies.
//! - [`cache_resolution`]: tells [`pico_driver`] to load the drivers from the cache.
//! - [`DriverDownloadError`]: why a download failed.
//! - [`available_drivers`], [`driver_binary`], [`DriverBinary`]: the binaries for each platform.
//! - [`get_cache_dir`], [`cached_driver_path`]: where the drivers go.
//! - [`download_url`], `BASE_URL`, `BASE_URL_ENV_VAR`: where the drivers come from.
//!
//! # Dependencies
//!
//! - [`pico_common`] gives [`Driver`].
//! - [`pico_driver`] gives [`LibraryResolution`], which loads the drivers from the cache.

use http_req::{request::Request, uri::Uri};
use pico_common::Driver;
use pico_driver::LibraryResolution;
use sha2::{Digest, Sha256};
use std::{
    convert::TryFrom,
    fmt::Write as _,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
use thiserror::Error;

mod manifest;
mod paths;

pub use manifest::{lookup as driver_binary, DriverBinary, BASE_URL, BINARIES, BUNDLE};
pub use paths::get_cache_dir;

/// The name of the environment variable that replaces [`BASE_URL`]
///
/// - Use it to download the drivers from a mirror.
/// - Each file must still have the SHA-256 hash in [`BINARIES`], so a mirror cannot supply a different driver.
pub const BASE_URL_ENV_VAR: &str = "PICO_DRIVERS_BASE_URL";

/// Why a driver download failed
#[derive(Error, Debug)]
pub enum DriverDownloadError {
    /// A file or directory operation failed
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    /// The URL was not valid, or the HTTP request failed
    #[error("HTTP request Error: {0}")]
    HttpRequestError(#[from] http_req::error::Error),

    /// The server sent a status code that is not a success
    #[error("HTTP response Error: {0}")]
    HttpResponseError(http_req::response::StatusCode),

    /// The bundle has no binary of the driver for this platform
    #[error("{driver} is not published for {os}-{arch} in driver bundle {bundle}")]
    Unavailable {
        /// The driver that you asked for
        driver: Driver,
        /// The operating system, as in [`std::env::consts::OS`]
        os: &'static str,
        /// The CPU architecture, as in [`std::env::consts::ARCH`]
        arch: &'static str,
        /// The bundle name, as in [`BUNDLE`]
        bundle: &'static str,
    },

    /// The SHA-256 hash of the downloaded file is not the hash in [`BINARIES`]
    #[error(
        "Downloaded {driver} does not match its expected hash (expected {expected}, got {actual})"
    )]
    HashMismatch {
        /// The driver that was downloaded
        driver: Driver,
        /// The hash in [`BINARIES`], in lowercase hex
        expected: &'static str,
        /// The hash of the downloaded file, in lowercase hex
        actual: String,
    },
}

/// The [`LibraryResolution`] that loads the drivers from [`get_cache_dir`]
///
/// - Use it with [`download_drivers_to_cache`].
///
/// ```no_run
/// use pico_common::Driver;
/// use pico_download::cache_resolution;
/// use pico_driver::DriverLoad;
///
/// let driver = Driver::PS2000A.load(&cache_resolution());
/// ```
pub fn cache_resolution() -> LibraryResolution {
    LibraryResolution::Custom(get_cache_dir())
}

/// Downloads the drivers and their dependencies to the directory `path`
///
/// - It makes `path` if it is not there.
/// - It also downloads the dependencies of each driver (picoipp for ps4000 and ps6000), when the bundle has them for this platform.
/// - It does not download a file that is already in `path` with the size in [`BINARIES`].
///   It does not check the hash of a file that is already there.
///   Thus you can call it each time your application starts.
/// - It writes each download to a temporary file in `path`, then renames it.
///   Thus a part file never has the name of a driver, also when more than one process downloads to `path`.
/// - It stops at the first error.
///   The drivers that it downloaded before the error stay in `path`.
/// - Returns Err [`DriverDownloadError::Unavailable`] when the bundle has no binary of a driver for this platform.
/// - Returns Err [`DriverDownloadError::HashMismatch`] when a downloaded file has the wrong SHA-256 hash.
/// - Returns Err [`DriverDownloadError::HttpRequestError`] or [`DriverDownloadError::HttpResponseError`] when the HTTP request fails.
/// - Returns Err [`DriverDownloadError::IoError`] when a file or directory operation fails.
///
/// ```no_run
/// use pico_common::Driver;
/// use pico_download::download_drivers;
/// use pico_driver::LibraryResolution;
///
/// let dir = std::path::PathBuf::from("drivers");
/// download_drivers(&[Driver::PS4000], &dir)?;
/// let resolution = LibraryResolution::Custom(dir);
/// # Ok::<(), pico_download::DriverDownloadError>(())
/// ```
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

/// Downloads the drivers and their dependencies to [`get_cache_dir`]
///
/// - Load the drivers with [`cache_resolution`].
/// - It works as [`download_drivers`] does, with the same errors.
/// - Panics when [`get_cache_dir`] panics.
///
/// ```no_run
/// use pico_common::Driver;
/// use pico_download::download_drivers_to_cache;
///
/// download_drivers_to_cache(&[Driver::PS2000A, Driver::TC08])?;
/// # Ok::<(), pico_download::DriverDownloadError>(())
/// ```
pub fn download_drivers_to_cache(drivers: &[Driver]) -> Result<(), DriverDownloadError> {
    download_drivers(drivers, get_cache_dir())
}

/// The drivers that the bundle has for this platform
///
/// - The platform is the target of the build.
/// - The list includes [`Driver::PicoIPP`] where the bundle has it.
///
/// ```
/// let drivers = pico_download::available_drivers();
/// println!("{} drivers for this platform", drivers.len());
/// ```
pub fn available_drivers() -> Vec<Driver> {
    BINARIES
        .iter()
        .filter(|binary| binary.os == std::env::consts::OS && binary.arch == std::env::consts::ARCH)
        .map(|binary| binary.driver)
        .collect()
}

/// The path of a driver in [`get_cache_dir`]
///
/// - It does not check if the file is there.
/// - Returns Err [`DriverDownloadError::Unavailable`] when the bundle has no binary of the driver for this platform.
/// - Panics when [`get_cache_dir`] panics.
pub fn cached_driver_path(driver: Driver) -> Result<PathBuf, DriverDownloadError> {
    Ok(get_cache_dir().join(binary_for_this_platform(driver)?.file_name))
}

/// The URL of a binary
///
/// - The URL is `<base>/<BUNDLE>/<asset_name>`.
/// - The base is the value of [`BASE_URL_ENV_VAR`] when it is set, or else [`BASE_URL`].
///   It removes each `/` at the end of the base.
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

/// Fetches `url` into `file`. `http_req` follows redirects itself (release assets are served
/// from object storage behind one) and writes only the final response body.
fn download_to(url: &str, file: &mut fs::File) -> Result<(), DriverDownloadError> {
    let uri = Uri::try_from(url)?;
    let status = Request::new(&uri).send(file)?.status_code();

    if status.is_success() {
        Ok(())
    } else {
        Err(DriverDownloadError::HttpResponseError(status))
    }
}

fn sha256_hex_for_file<P: AsRef<Path>>(path: P) -> Result<String, DriverDownloadError> {
    let mut src_file = fs::File::open(&path)?;

    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let count = src_file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let mut hex = String::with_capacity(64);
    for byte in hasher.finalize() {
        let _ = write!(hex, "{:02x}", byte);
    }

    Ok(hex)
}

#[cfg(test)]
mod tests {
    use super::*;
    use enum_iterator::all;

    /// Every target, and the drivers Pico do not build for it. Everything else in
    /// `Driver` must be in the manifest.
    const TARGETS: &[(&str, &str, &[Driver])] = &[
        ("windows", "x86_64", &[]),
        ("windows", "x86", &[Driver::PSOSPA]),
        (
            "windows",
            "aarch64",
            &[
                Driver::PS4000,
                Driver::PS6000,
                Driver::DrDAQ,
                Driver::PicoIPP,
            ],
        ),
        ("macos", "x86_64", &[]),
        (
            "macos",
            "aarch64",
            &[
                Driver::PS4000,
                Driver::PS6000,
                Driver::DrDAQ,
                Driver::PicoIPP,
            ],
        ),
        ("linux", "x86_64", &[]),
        (
            "linux",
            "aarch64",
            &[
                Driver::PS4000,
                Driver::PS6000,
                Driver::DrDAQ,
                Driver::PicoIPP,
            ],
        ),
        (
            "linux",
            "arm",
            &[
                Driver::PS2000A,
                Driver::PS3000A,
                Driver::PS4000A,
                Driver::PS5000A,
                Driver::PS6000A,
                Driver::PSOSPA,
                Driver::PicoIPP,
            ],
        ),
    ];

    #[test]
    fn every_driver_is_published_for_every_target() {
        for &(os, arch, absent) in TARGETS {
            for driver in all::<Driver>() {
                let published = driver_binary(driver, os, arch).is_some();
                let expected = !absent.contains(&driver);
                assert_eq!(
                    published, expected,
                    "{driver} on {os}-{arch}: published {published}, expected {expected}"
                );
            }
        }
    }

    #[test]
    fn loggers_are_available_on_this_platform() {
        let this = (std::env::consts::OS, std::env::consts::ARCH);
        let Some(&(_, _, absent)) = TARGETS.iter().find(|&&(os, arch, _)| (os, arch) == this)
        else {
            return;
        };
        let available = available_drivers();
        for driver in [
            Driver::TC08,
            Driver::PL1000,
            Driver::PicoHRDL,
            Driver::PT104,
            Driver::DrDAQ,
            Driver::PLCM3,
        ] {
            if absent.contains(&driver) {
                assert!(binary_for_this_platform(driver).is_err(), "{driver}");
                continue;
            }
            assert!(available.contains(&driver), "{driver} not available");
            let binary = binary_for_this_platform(driver).unwrap();
            assert_eq!(binary.file_name, driver.get_binary_name());
        }
    }

    #[test]
    fn file_names_match_the_driver() {
        for binary in BINARIES {
            let base = binary.driver.to_string();
            let expected = match binary.os {
                "windows" => format!("{base}.dll"),
                "macos" => format!("lib{base}.dylib"),
                _ => format!("lib{base}.so"),
            };
            assert_eq!(binary.file_name, expected, "{}", binary.asset_name);
            assert_eq!(
                binary.asset_name,
                format!("{}-{}-{}", binary.os, binary.arch, binary.file_name)
            );
            assert_eq!(binary.sha256.len(), 64, "{}", binary.asset_name);
        }
    }

    #[test]
    fn one_binary_per_driver_and_target() {
        let mut seen = std::collections::BTreeSet::new();
        for binary in BINARIES {
            assert!(
                seen.insert((binary.driver, binary.os, binary.arch)),
                "{} listed twice",
                binary.asset_name
            );
        }
    }
}
