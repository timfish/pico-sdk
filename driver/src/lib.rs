//! Common, safe wrappers for Pico Technology drivers.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! Pico instruments fall into families that share almost nothing at the API level. Oscilloscopes
//! configure channels with a range, coupling and offset and stream through driver-owned buffers;
//! the TC-08 data logger configures channels with a thermocouple type and is polled for
//! temperatures. Rather than force both through one trait full of methods that only apply to half
//! the implementors, each family gets its own driver trait and its own configuration types.
//!
//! [`PicoDriver`] is the only place the families meet. Match on it once after enumeration and
//! everything downstream is statically typed again.
//!
//! # Examples
//! Using the raw safe bindings to open and configure the first available oscilloscope:
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use pico_common::{
//!     Driver, OscilloscopeChannelConfig, PicoChannel, PicoCoupling, PicoInfo, PicoRange,
//! };
//! use pico_driver::{DriverLoad, LibraryResolution, PicoDriver};
//!
//! // Load the ps2000 driver library with the default resolution
//! let driver = match Driver::PS2000.load(&LibraryResolution::Default)? {
//!     PicoDriver::Oscilloscope(driver) => driver,
//!     _ => unreachable!("ps2000 is an oscilloscope driver"),
//! };
//!
//! // Open the first device
//! let handle = driver.open_unit(None)?;
//! let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
//!
//! let ch_config = OscilloscopeChannelConfig {
//!     coupling: PicoCoupling::DC,
//!     range: PicoRange::X1_PROBE_2V,
//!     offset: 0.0
//! };
//!
//! driver.enable_channel(handle, PicoChannel::A, &ch_config)?;
//! # Ok(())
//! # }
//! ```

#![allow(clippy::upper_case_acronyms)]

pub mod cm3;
pub mod drdaq;
pub mod hrdl;
pub mod kernel_driver;
pub mod oscilloscope;
pub mod pl1000;
pub mod pt104;
mod resolution;
pub mod tc08;

pub use resolution::{DriverLoad, LibraryResolution};

use pico_common::PicoError;
use std::fmt;
use thiserror::Error;

/// Covers the various errors encountered when attempting to load drivers
#[derive(Error, Debug)]
pub enum DriverLoadError {
    #[error("Pico driver error: {0}")]
    DriverError(#[from] PicoError),

    #[error("Library load error: {0}")]
    LibloadingError(#[from] libloading::Error),

    #[error("Invalid Driver Version: Requires >= {required}, Found: {found}")]
    VersionError { found: String, required: String },
}

/// A loaded driver for any supported instrument family
///
/// Enumeration hands back a mixed list of these because a machine can have an oscilloscope and a
/// data logger plugged in at once. Match to get to the family specific driver.
#[derive(Clone)]
pub enum PicoDriver {
    DrDAQ(drdaq::DrDAQDriver),
    Oscilloscope(oscilloscope::OscilloscopeDriver),
    PicoHRDL(hrdl::HRDLDriver),
    PL1000(pl1000::PL1000Driver),
    PLCM3(cm3::PLCM3Driver),
    PT104(pt104::PT104Driver),
    TC08(tc08::TC08Driver),
}

impl fmt::Debug for PicoDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DrDAQ(driver) => driver.fmt(f),
            Self::Oscilloscope(driver) => driver.fmt(f),
            Self::PicoHRDL(driver) => driver.fmt(f),
            Self::PL1000(driver) => driver.fmt(f),
            Self::PLCM3(driver) => driver.fmt(f),
            Self::PT104(driver) => driver.fmt(f),
            Self::TC08(driver) => driver.fmt(f),
        }
    }
}
