#![forbid(unsafe_code)]

//! Downloads Pico Technology oscilloscope drivers.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! This helper crate allows the distribution of software that can communicate with every Pico oscilloscope
//! without the need to ship every driver binary.
//!
//! Supported platforms:
//!  - Windows (32 or 64 bit)
//!  - macOS (64 bit)
//!  - Linux (64 bit or ARM for Raspberry Pi)
//!
//! All binaries are stored in Amazon AWS S3 storage and can be browsed [here](https://pico-drivers.s3.eu-west-2.amazonaws.com/).
//!
//! Some of these binaries have been modified using `install_name_tool` or `rpath` after download from the
//! Pico website which allows them to be loaded dynamically from non-standard paths. Pico singing certificates remain.
//!
//! File hashes are validated after download to ensure the hosted files have not been tampered with.
//! These hashes are generated using the `generate-hashes` example.
//!
//! `download_drivers_to_cache()` downloads the passed drivers and their dependencies and `cache_resolution()`
//! returns a `Resolution` that can be used to resolve the downloaded binaries.
//!
//! `let enumerator = DeviceEnumerator::with_resolution(cache_resolution());`

use hashes::get_expected_driver_hash;
use http_req::request;
use paths::get_cache_dir;
use pico_common::Driver;
use pico_driver::Resolution;
use ring::digest::{Context, Digest, SHA256};
use std::{fs, io::Read, path::Path};
use thiserror::Error;

mod hashes;
mod paths;

#[derive(Error, Debug)]
pub enum DriverDownloadError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP request Error: {0}")]
    HttpRequestError(#[from] http_req::error::Error),

    #[error("HTTP response Error: {0}")]
    HttpResponseError(http_req::response::StatusCode),

    #[error("Invalid driver hash")]
    HashMismatch,
}

/// Gets a `Resolution` that points to the download cache location
pub fn cache_resolution() -> Resolution {
    Resolution::Custom(get_cache_dir())
}

/// Downloads the requested drivers and any dependencies to the supplied path.
pub fn download_drivers<P: AsRef<Path>>(
    drivers: &[Driver],
    path: P,
) -> Result<(), DriverDownloadError> {
    let driver_dir = path.as_ref().to_path_buf();

    let required_files = [drivers, &Driver::get_dependencies_for_platform()].concat();

    fs::create_dir_all(&driver_dir)?;

    for driver in required_files {
        let file_path = driver_dir.join(&driver.get_binary_name());

        if file_path.exists() {
            match driver {
                Driver::PicoIPP | Driver::IOMP5 => {
                    continue;
                }
                _ => {
                    fs::remove_file(&file_path)?;
                }
            }
        }

        download_driver(driver, &driver_dir)?;
    }

    Ok(())
}

/// Downloads the requested drivers and any dependencies to the cache location.
///
/// Drivers downloaded using this function can be loaded using the `Resolution`
/// returned from `cache_resolution()`.
///
/// `DeviceEnumerator::with_resolution(cache_resolution())`
pub fn download_drivers_to_cache(drivers: &[Driver]) -> Result<(), DriverDownloadError> {
    download_drivers(drivers, get_cache_dir())
}

fn sha256_digest_for_file<P: AsRef<Path>>(path: P) -> Result<Digest, DriverDownloadError> {
    let mut src_file = fs::File::open(&path)?;

    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = src_file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn download_driver(driver: Driver, dst_dir: &Path) -> Result<(), DriverDownloadError> {
    let name = driver.get_binary_name();

    let url = format!(
        "https://pico-drivers.s3.eu-west-2.amazonaws.com/{}/{}/{}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        name
    );

    let dst_temp_path = dst_dir.join(name.to_string() + ".temp");

    let mut dst_file = fs::File::create(&dst_temp_path)?;
    let response = request::get(url, &mut dst_file)?;

    if response.status_code().is_success() {
        let computed_hash = format!("{:?}", sha256_digest_for_file(&dst_temp_path)?);

        let expected_hash = get_expected_driver_hash(driver);

        if computed_hash == expected_hash {
            let dst_path = dst_dir.join(name);
            fs::copy(&dst_temp_path, dst_path)?;
            fs::remove_file(dst_temp_path)?;

            Ok(())
        } else {
            Err(DriverDownloadError::HashMismatch)
        }
    } else {
        Err(DriverDownloadError::HttpResponseError(
            response.status_code(),
        ))
    }
}
