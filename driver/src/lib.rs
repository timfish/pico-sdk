//! Common, safe wrappers for Pico Technology oscilloscope drivers.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! Each Pico Technology oscilloscope relies on a native driver for communication and this driver will
//! vary depending on the device product range. Each of these drivers has an interface which differs by
//! either a few function arguments or a vastly differing API.
//!
//! `PS2000Driver`, `PS2000ADriver`, `PS3000ADriver`, `PS4000Driver`,
//! `PS4000ADriver`, `PS5000ADriver`, `PS6000Driver` and `PS6000ADriver` wrap
//! their corresponding loaders and expose a safe, common API by implementing
//! the `OscilloscopeDriver` trait. These can be constructed with a `Resolution` which tells the wrapper where
//! to resolve the dynamic library from.
//!
//! # Examples
//! Using the raw safe bindings to open and configure the first available device:
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use pico_common::{OscilloscopeChannelConfig, Driver, PicoChannel, PicoCoupling, PicoInfo, PicoRange};
//! use pico_driver::{LibraryResolution, oscilloscope::{self, OscilloscopeDriver}};
//!
//! // Load the ps2000 driver library with the default resolution
//! let driver = oscilloscope::PS2000Driver::load(&LibraryResolution::Default)?;
//! // Load the ps4000a driver library from the applications root directory
//! let driver = oscilloscope::PS4000ADriver::load(&LibraryResolution::AppRoot)?;
//!
//! // Open the first device
//! let handle = driver.open_unit(None)?;
//! let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
//!
//! let ch_config = OscilloscopeChannelConfig {
//!     coupling: PicoCoupling::DC,
//!     range: PicoRange::X1_PROBE_2V,
//!     offset: None,
//! };
//!
//! driver.enable_channel(handle, PicoChannel::A, &ch_config)?;
//! # Ok(())
//! # }
//! ```

#![allow(clippy::upper_case_acronyms)]

mod resolution;

pub use resolution::{DriverLoader, LibraryResolution};
pub mod kernel_driver;
pub mod oscilloscope;
pub mod tc08;

use std::fmt;

#[derive(Clone)]
pub enum PicoDriver {
    Oscilloscope(oscilloscope::ArcDriver),
    TC08(tc08::ArcDriver),
}

impl fmt::Debug for PicoDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Oscilloscope(_) => f.debug_tuple("ScopeDriver").finish(),
            Self::TC08(_) => f.debug_tuple("TC08Driver").finish(),
        }
    }
}

impl<'a> From<&'a PicoDriver> for &'a oscilloscope::ArcDriver {
    fn from(value: &'a PicoDriver) -> Self {
        match value {
            PicoDriver::Oscilloscope(driver) => driver,
            _ => panic!("Cannot convert {value:?} to OscilloscopeDriver"),
        }
    }
}
