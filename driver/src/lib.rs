/*!
Dynamic loading, unsafe and safe wrappers for Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

Each Pico Technology oscilloscope relies on a native driver for communication and this driver will
vary depending on the device product range. Each of these drivers has an interface which differs by
either a few function arguments or a vastly differing API. This crate caters for these through two
code paths, one for the ps2000 driver and one for the the rest.

`LoaderPS2000` dynamically loads the ps2000 driver and exposes the `unsafe` function calls and
`LoaderCommon` does the same for every other supported driver.

`DriverPS2000` and `DriverCommon` wrap the loaders and expose a safe, common API by implementing
the `PicoDriver` trait. These can be constructed with a `Resolution` which tells the wrapper where
to resolve the dynamic library from. The `LoadDriverExt` trait supplies a shortcut to load a driver
directly from the `Driver` enum via `try_load` and `try_load_with_resolution`.

# Examples
Using the raw safe bindings to open and configure the first available device:
```no_run
# fn run() -> Result<(),Box<dyn std::error::Error>> {
use pico_common::{ChannelConfig, Driver, PicoChannel, PicoCoupling, PicoInfo, PicoRange};
use pico_driver::{LoadDriverExt, Resolution};

// Load the ps2000 driver library with the default resolution
let driver = Driver::PS2000.try_load()?;
// Load the ps4000a driver library from the applications root directory
let driver = Driver::PS4000A.try_load_with_resolution(&Resolution::AppRoot)?;

// Open the first device
let handle = driver.open_unit(None)?;
let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;

let ch_config = ChannelConfig {
    enabled: true,
    coupling: PicoCoupling::DC,
    range: PicoRange::X1_PROBE_2V,
    offset: 0.0
};

driver.set_channel(handle, PicoChannel::A, &ch_config)?;
# Ok(())
# }
```

*/

pub use common::{DriverCommon, LoaderCommon};
use pico_common::{
    ChannelConfig, Driver, PicoChannel, PicoError, PicoInfo, PicoRange, PicoResult, SampleConfig,
};
pub use ps2000::{DriverPS2000, LoaderPS2000};
pub use resolution::{DependencyLoader, Resolution};
use std::{collections::HashMap, pin::Pin, sync::Arc};
use thiserror::Error;
use version_compare::Version;

mod common;
mod ps2000;
mod resolution;

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

/// Encapsulates the two different callbacks we can expect from Pico drivers
pub enum CallbackType {
    Common {
        start_index: usize,
        sample_count: usize,
    },
    PS2000(HashMap<PicoChannel, Vec<i16>>),
}

/// Common trait implemented for every driver
pub trait PicoDriver: Send + Sync {
    /// Gets the underlying driver type
    fn get_driver(&self) -> Driver;
    /// Gets the driver version string
    fn get_version(&self) -> PicoResult<String>;
    /// Gets the path to the loaded driver
    fn get_path(&self) -> PicoResult<Option<String>>;
    /// Returns a list of discovered serial numbers
    fn enumerate_units(&self) -> PicoResult<Vec<String>>;
    /// Opens a device, optionally with a specific serial number
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16>;
    /// Ping a unit to see if it's still connected
    fn ping_unit(&self, handle: i16) -> PicoResult<()>;
    /// Get the maximum expected ADC value. This is required to scale to volts
    fn maximum_value(&self, handle: i16) -> PicoResult<i16>;
    /// Close the specified unit
    fn close_unit(&self, handle: i16) -> PicoResult<()>;
    /// Get one of the unit info strings
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String>;
    /// Get valid ranges for the specified channel
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>>;
    /// Set up a channel with the supplied config
    fn set_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &ChannelConfig,
    ) -> PicoResult<()>;
    /// Find out if this driver allocates its own buffers. Only the ps2000
    /// driver does this
    fn allocates_own_buffers(&self) -> bool;
    /// Give the driver a buffer to write data into
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: &Pin<Vec<i16>>,
        buffer_len: usize,
    ) -> PicoResult<()>;
    /// Starts the device streaming
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &SampleConfig,
    ) -> PicoResult<SampleConfig>;
    /// Gets the latest streaming values. The ps2000 has a different callback to
    /// the rest of the drivers.
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        callback: Box<dyn FnMut(CallbackType) + 'a>,
    ) -> PicoResult<()>;
    /// Stops the device streaming
    fn stop_streaming(&self, handle: i16) -> PicoResult<()>;
    /// Check that the driver meets the minimum version tested with this wrapper
    fn check_version(&self) -> Result<(), DriverLoadError> {
        let loaded_str = &self.get_version()?;
        let loaded_version = Version::from(loaded_str);

        #[allow(clippy::expect_fun_call)]
        let required_str = get_min_required_version(self.get_driver());
        let required_version = Version::from(required_str);

        if loaded_version < required_version {
            Err(DriverLoadError::VersionError {
                found: loaded_str.to_string(),
                required: required_str.to_string(),
            })
        } else {
            Ok(())
        }
    }
}

pub type ArcDriver = Arc<Box<dyn PicoDriver>>;

/// Gets the minimum supported driver version
fn get_min_required_version(driver: Driver) -> &'static str {
    match driver {
        Driver::PS2000 => "3.0.30.1878",
        Driver::PS2000A
        | Driver::PS3000A
        | Driver::PS4000
        | Driver::PS4000A
        | Driver::PS5000A
        | Driver::PS6000 => "2.1.30.1878",
        _ => panic!(format!(
            "We don't know the minimum required version for the {:?} driver!",
            driver,
        )),
    }
}

/// Shortcuts for loading drivers directly from the `Driver` enum.
pub trait LoadDriverExt {
    fn try_load(&self) -> Result<ArcDriver, DriverLoadError>;
    fn try_load_with_resolution(
        &self,
        resolution: &Resolution,
    ) -> Result<ArcDriver, DriverLoadError>;
}

impl LoadDriverExt for Driver {
    fn try_load(&self) -> Result<ArcDriver, DriverLoadError> {
        self.try_load_with_resolution(&Default::default())
    }

    fn try_load_with_resolution(
        &self,
        resolution: &Resolution,
    ) -> Result<ArcDriver, DriverLoadError> {
        Ok(Arc::new(match self {
            Driver::PS2000 => Box::new(DriverPS2000::new(&resolution)?),
            Driver::PicoIPP | Driver::IOMP5 => panic!("This type of driver cannot be loaded"),
            _ => Box::new(DriverCommon::new(*self, &resolution)?),
        }))
    }
}
