#![allow(clippy::upper_case_acronyms)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

//! Loads the Pico Technology driver libraries and gives a safe Rust API for each instrument family.
//!
//! Each Pico instrument range has its own C driver library, such as `ps2000a` or `usbtc08`.
//! This crate loads a library at run time, binds its functions, and wraps them in safe methods.
//! It does not link to the libraries at build time, so a program builds and starts when no Pico driver is installed.
//! This crate contains unsafe code, as it calls the C functions of the drivers.
//!
//! The oscilloscope drivers all get one API: [`oscilloscope::OscilloscopeDriver`] and its feature traits.
//! Each data logger driver has an API of its own, as the data loggers have little in common.
//! [`PicoDriver`] holds a loaded driver of any family.
//!
//! # Quick start
//!
//! ```no_run
//! use pico_common::{PicoInfo, ScopeDriver};
//! use pico_driver::{oscilloscope::OscilloscopeDriver, LibraryResolution};
//!
//! let scope = OscilloscopeDriver::load(ScopeDriver::PS2000A, &LibraryResolution::Default)?;
//! println!("driver version {}", scope.get_version()?);
//!
//! for unit in scope.enumerate_units()? {
//!     println!("{} {}", unit.variant, unit.serial);
//! }
//!
//! let handle = scope.open_unit(None)?;
//! println!("opened {}", scope.get_unit_info(handle, PicoInfo::VARIANT_INFO)?);
//! scope.close(handle)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # How the parts fit
//!
//! ```text
//!  Driver + LibraryResolution     the resolution gives the path of the library
//!         │
//!         ▼
//!  DriverLoad::load()             loads the library and keeps it loaded
//!         │
//!         ▼
//!  PicoDriver                     one variant for each family
//!         │
//!         ▼
//!  OscilloscopeDriver ──► OscilloscopeDriverInternal and the feature traits
//!  TC08Driver, PT104Driver, ... ──► the methods of each data logger
//!         │
//!         ▼
//!  open_unit() ──► handle: i16    each call on a unit takes its handle
//!         │
//!         ▼
//!  close() or close_unit()
//! ```
//!
//! # Main types
//!
//! - [`DriverLoad`]: loads the library of a [`Driver`] and gives a [`PicoDriver`].
//! - [`LibraryResolution`]: where to find the library file.
//! - [`PicoDriver`]: a loaded driver of any family.
//! - [`DriverLoadError`]: why a driver did not load.
//! - [`oscilloscope::OscilloscopeDriver`]: a loaded oscilloscope driver of any range.
//! - [`oscilloscope::features`]: one trait for each oscilloscope feature, such as block mode or the trigger.
//! - [`tc08::TC08Driver`], [`pt104::PT104Driver`], [`cm3::PLCM3Driver`]: the TC-08, PT-104 and PicoLog CM3 data loggers.
//! - [`pl1000::PL1000Driver`], [`drdaq::DrDAQDriver`], [`hrdl::HRDLDriver`]: the PicoLog 1000, DrDAQ and ADC-20/ADC-24 data loggers.
//! - [`kernel_driver::is_missing`]: tells if the Pico kernel driver looks missing on Windows.
//!
//! # Dependencies
//!
//! - [`pico_common`] gives [`Driver`], [`PicoError`], and the configuration and information types of each family.
//! - [`pico_sys_dynamic`] gives the loaders that bind the C functions of each library at run time.
//!
//! # Cargo features
//!
//! - `serde`: adds `Serialize` and `Deserialize` to [`oscilloscope::EnumerationResult`].
//!
//! [`Driver`]: pico_common::Driver

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
use std::{fmt, mem::ManuallyDrop};
use thiserror::Error;

/// Holds a driver library's bindings and never unloads the library.
///
/// Drivers start threads of their own that can still run after the last
/// handle closes, as when a stream does not stop. Unloading the library
/// under such a thread crashes the process.
pub(crate) type Kept<T> = ManuallyDrop<T>;

/// Why a driver did not load
#[derive(Error, Debug)]
pub enum DriverLoadError {
    /// The driver returned an error
    #[error("Pico driver error: {0}")]
    DriverError(#[from] PicoError),

    /// The library file did not load, for example because it is not there
    #[error("Library load error: {0}")]
    LibloadingError(#[from] libloading::Error),

    /// The driver library is older than the oldest version this crate was tested with
    ///
    /// - Only [`OscilloscopeDriverInternal::check_version`](oscilloscope::OscilloscopeDriverInternal::check_version) returns it.
    #[error("Invalid Driver Version: Requires >= {required}, Found: {found}")]
    VersionError {
        /// The version of the loaded library
        found: String,
        /// The oldest version this crate accepts
        required: String,
    },
}

/// A loaded driver of any instrument family
///
/// - A computer can have an oscilloscope and a data logger connected at the same time, so a list of drivers can hold more than one family.
/// - Match on the variant to get the driver of the family.
/// - Clones share the loaded library.
#[derive(Clone)]
pub enum PicoDriver {
    /// The USB DrDAQ driver
    DrDAQ(drdaq::DrDAQDriver),
    /// An oscilloscope driver of any range
    Oscilloscope(oscilloscope::OscilloscopeDriver),
    /// The ADC-20 and ADC-24 driver
    PicoHRDL(hrdl::HRDLDriver),
    /// The PicoLog 1000 Series driver
    PL1000(pl1000::PL1000Driver),
    /// The PicoLog CM3 driver
    PLCM3(cm3::PLCM3Driver),
    /// The USB PT-104 driver
    PT104(pt104::PT104Driver),
    /// The USB TC-08 driver
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
