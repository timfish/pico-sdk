//! Safe wrappers for Pico Technology oscilloscope drivers.
//!
//! Every oscilloscope driver exposes the same capabilities through slightly different C APIs, so
//! each wrapper implements [`OscilloscopeDriverInternal`] and callers work against
//! [`OscilloscopeDriver`] without caring which product range they have.
//!
//! Data loggers are a separate family with a different API shape entirely - see
//! [`crate::tc08`].

mod dependencies;
mod trampoline;

mod ps2000;
mod ps2000a;
mod ps3000a;
mod ps4000;
mod ps4000a;
mod ps5000a;
mod ps6000;
mod ps6000a;
mod psospa;

pub use ps2000::PS2000Driver;
pub use ps2000a::PS2000ADriver;
pub use ps3000a::PS3000ADriver;
pub use ps4000::PS4000Driver;
pub use ps4000a::PS4000ADriver;
pub use ps5000a::PS5000ADriver;
pub use ps6000::PS6000Driver;
pub use ps6000a::PS6000ADriver;
pub use psospa::PSOSPADriver;

use crate::DriverLoadError;
use parking_lot::RwLock;
use pico_common::{
    OscilloscopeChannelConfig, Driver, FromPicoStr, PicoChannel, PicoError, PicoInfo, PicoRange, PicoResolution, PicoResult,
    PicoStatus, OscilloscopeSampleConfig,
};
use std::{fmt, ops::Deref, sync::Arc};
use version_compare::Version;

/// Serial and variant pairs returned by driver enumeration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct EnumerationResult {
    pub variant: String,
    pub serial: String,
}

/// Common trait implemented for every oscilloscope driver
pub trait OscilloscopeDriverInternal: fmt::Debug + Send + Sync {
    /// Gets the underlying driver type
    fn get_driver(&self) -> Driver;
    /// Gets the driver version string
    fn get_version(&self) -> PicoResult<String>;
    /// Gets the path to the loaded driver
    fn get_path(&self) -> PicoResult<Option<String>>;
    /// Returns a list of discovered serial numbers
    fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>>;
    /// Opens a device, optionally with a specific serial number
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16>;
    /// Ping a unit to see if it's still connected
    fn ping_unit(&self, handle: i16) -> PicoResult<()>;
    /// Get the maximum expected ADC value. This is required to scale to volts
    fn maximum_value(&self, handle: i16) -> PicoResult<i16>;
    /// Close the specified unit
    fn close(&self, handle: i16) -> PicoResult<()>;
    /// Get one of the unit info strings
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String>;
    /// Get valid ranges for the specified channel
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>>;
    /// The resolutions the unit supports, each with its maximum ADC value.
    /// Empty for a unit whose resolution is fixed.
    fn get_resolutions(&self, _handle: i16) -> PicoResult<Vec<(PicoResolution, i16)>> {
        Ok(Vec::new())
    }
    /// The resolution the unit is set to, or `None` if it is fixed
    fn get_resolution(&self, _handle: i16) -> Option<PicoResolution> {
        None
    }
    /// Set the unit's resolution. Takes effect from the next stream start.
    fn set_resolution(&self, _handle: i16, _resolution: PicoResolution) -> PicoResult<()> {
        Err(PicoError::from_status(
            PicoStatus::RESOLUTION_NOT_SUPPORTED_BY_VARIANT,
            "set_resolution",
        ))
    }
    /// Set up a channel with the supplied config
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
    ) -> PicoResult<()>;
    /// Disable a channel
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()>;
    /// Give the driver a buffer to write data into
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: Arc<RwLock<Vec<i16>>>,
        buffer_len: usize,
    ) -> PicoResult<()>;
    /// Starts the device streaming
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &OscilloscopeSampleConfig,
        enabled_channels: u8,
    ) -> PicoResult<OscilloscopeSampleConfig>;
    /// Gets the latest streaming values
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        channels: &[PicoChannel],
        callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoResult<()>;
    /// Stops the device streaming
    fn stop(&self, handle: i16) -> PicoResult<()>;
    /// Check that the driver meets the minimum version tested with these wrappers
    fn check_version(&self) -> Result<(), DriverLoadError> {
        let loaded_str = &self.get_version()?;
        let loaded_version = Version::from(loaded_str);

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

/// A loaded oscilloscope driver, whichever product range it is for
///
/// This is a newtype rather than a bare `Arc<dyn ..>` so that family level behaviour has
/// somewhere to live and so [`crate::PicoDriver`] can hold it.
#[derive(Clone)]
pub struct OscilloscopeDriver(Arc<dyn OscilloscopeDriverInternal>);

impl OscilloscopeDriver {
    pub fn new<D: OscilloscopeDriverInternal + 'static>(driver: D) -> Self {
        OscilloscopeDriver(Arc::new(driver))
    }
}

impl Deref for OscilloscopeDriver {
    type Target = Arc<dyn OscilloscopeDriverInternal>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for OscilloscopeDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub(crate) fn get_version_string(input: &str) -> String {
    input
        .split([' ', ','])
        .next_back()
        .expect("Invalid version string")
        .to_string()
}

/// Parses `serial[variant],serial[variant]`. Some drivers answer OK with
/// an empty list, so an empty or malformed entry yields nothing.
pub(crate) fn parse_enum_result(buffer: &[i8], len: usize) -> Vec<EnumerationResult> {
    if len == 0 {
        return Vec::new();
    }

    buffer
        .from_pico_i8_string(len)
        .split(',')
        .filter_map(|device| {
            let (serial, variant) = device.split_once('[')?;

            Some(EnumerationResult {
                serial: serial.to_string(),
                variant: variant.trim_end_matches(']').to_string(),
            })
        })
        .collect()
}

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
        Driver::PS6000A => "1.0.54.2438",
        _ => panic!(
            "We don't know the minimum required version for the {:?} driver!",
            driver,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::ToPicoStr;

    fn parse(list: &str) -> Vec<(String, String)> {
        let buffer = list.into_pico_i8_string();
        parse_enum_result(&buffer, buffer.len())
            .into_iter()
            .map(|r| (r.serial, r.variant))
            .collect()
    }

    #[test]
    fn parse_enum_result_reads_each_unit() {
        assert_eq!(
            parse("CW356/055[2208A],13474/0004[5464E+MSO]"),
            vec![
                ("CW356/055".to_string(), "2208A".to_string()),
                ("13474/0004".to_string(), "5464E+MSO".to_string()),
            ]
        );
    }

    #[test]
    fn parse_enum_result_tolerates_an_empty_or_malformed_list() {
        assert!(parse_enum_result(&[], 0).is_empty());
        assert!(parse("").is_empty());
        assert!(parse("garbage").is_empty());
    }
}
