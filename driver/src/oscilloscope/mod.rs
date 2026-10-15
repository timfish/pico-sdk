//! Safe wrappers for Pico Technology oscilloscope drivers.
//!
//! Every oscilloscope driver exposes the same capabilities through slightly different C APIs, so
//! each wrapper implements [`OscilloscopeDriverInternal`] and callers work against
//! [`OscilloscopeDriver`] without caring which product range they have.
//!
//! Data loggers are a separate family with a different API shape entirely - see
//! [`crate::tc08`].

mod dependencies;
pub mod features;
mod models;
pub mod outputs;
mod siggen;
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

pub use features::{
    AuxIoDriver, BlockDriver, BlockReady, ChannelExtrasDriver, DigitalPortDriver, EtsDriver,
    LedDriver, RapidBlockDriver, SigGenDriver, StreamingDriver, TriggerDriver,
};

use crate::DriverLoadError;
use parking_lot::RwLock;
use pico_common::{
    BandwidthLimit, Downsample, DownsampleKind, Driver, FromPicoStr, OscilloscopeChannelConfig,
    PicoChannel, PicoError, PicoInfo, PicoRange, PicoResolution, PicoResult, PicoStatus,
    PowerSource, RateLimit, SampleType, ScopeCapabilities,
};
use std::{collections::HashMap, fmt, ops::Deref, sync::Arc};
use version_compare::Version;

/// Serial and variant pairs returned by driver enumeration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct EnumerationResult {
    pub variant: String,
    pub serial: String,
}

/// A buffer the driver writes samples into, of the type the driver was
/// asked for.
#[derive(Clone, Debug)]
pub enum SampleBuffer {
    I8(Arc<RwLock<Vec<i8>>>),
    I16(Arc<RwLock<Vec<i16>>>),
    I32(Arc<RwLock<Vec<i32>>>),
}

impl SampleBuffer {
    /// A zeroed buffer of `len` samples of `sample_type`.
    pub fn new(sample_type: SampleType, len: usize) -> Self {
        match sample_type {
            SampleType::I8 => SampleBuffer::I8(Arc::new(RwLock::new(vec![0; len]))),
            SampleType::I16 => SampleBuffer::I16(Arc::new(RwLock::new(vec![0; len]))),
            SampleType::I32 => SampleBuffer::I32(Arc::new(RwLock::new(vec![0; len]))),
        }
    }

    pub fn sample_type(&self) -> SampleType {
        match self {
            SampleBuffer::I8(_) => SampleType::I8,
            SampleBuffer::I16(_) => SampleType::I16,
            SampleBuffer::I32(_) => SampleType::I32,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            SampleBuffer::I8(b) => b.read().len(),
            SampleBuffer::I16(b) => b.read().len(),
            SampleBuffer::I32(b) => b.read().len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The `i16` buffer, for drivers that take no other type.
    pub fn as_i16(&self, context: &str) -> PicoResult<&Arc<RwLock<Vec<i16>>>> {
        match self {
            SampleBuffer::I16(b) => Ok(b),
            _ => Err(PicoError::from_status(
                PicoStatus::RATIO_MODE_DOES_NOT_SUPPORT_DATATYPE,
                context,
            )),
        }
    }
}

/// The ratio and mode code for a family whose streaming downsample is
/// average only, given the family's codes for no mode and for average.
pub(crate) fn average_only<M>(
    downsample: Option<Downsample>,
    none: M,
    average: M,
) -> PicoResult<(u32, M)> {
    match Downsample::effective(downsample)? {
        None => Ok((1, none)),
        Some(Downsample {
            ratio,
            mode: DownsampleKind::Average,
        }) => Ok((ratio, average)),
        Some(_) => Err(PicoError::from_status(
            PicoStatus::RATIO_MODE_NOT_SUPPORTED,
            "downsample",
        )),
    }
}

/// The ratio, ratio mode and data type codes of the ps6000a and psospa
/// drivers for a stream with `downsample` into buffers of `sample_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PicoStreamMode {
    pub ratio: u64,
    pub mode: u32,
    pub data_type: u32,
}

impl PicoStreamMode {
    pub(crate) fn new(
        downsample: Option<Downsample>,
        sample_type: SampleType,
    ) -> PicoResult<PicoStreamMode> {
        use pico_sys_dynamic::psospa::{
            enPicoDataType_PICO_INT16_T, enPicoDataType_PICO_INT32_T, enPicoDataType_PICO_INT8_T,
            enPicoRatioMode_PICO_RATIO_MODE_AGGREGATE, enPicoRatioMode_PICO_RATIO_MODE_AVERAGE,
            enPicoRatioMode_PICO_RATIO_MODE_DECIMATE, enPicoRatioMode_PICO_RATIO_MODE_RAW,
            enPicoRatioMode_PICO_RATIO_MODE_SUM,
        };

        let downsample = Downsample::effective(downsample)?;
        let (ratio, mode) = match downsample {
            None => (1, enPicoRatioMode_PICO_RATIO_MODE_RAW),
            Some(Downsample {
                ratio,
                mode: DownsampleKind::Average,
            }) => (ratio, enPicoRatioMode_PICO_RATIO_MODE_AVERAGE),
            Some(Downsample {
                ratio,
                mode: DownsampleKind::Sum,
            }) => (ratio, enPicoRatioMode_PICO_RATIO_MODE_SUM),
            Some(Downsample {
                ratio,
                mode: DownsampleKind::Aggregate,
            }) => (ratio, enPicoRatioMode_PICO_RATIO_MODE_AGGREGATE),
            Some(Downsample {
                ratio,
                mode: DownsampleKind::Decimate,
            }) => (ratio, enPicoRatioMode_PICO_RATIO_MODE_DECIMATE),
        };
        let fits = matches!(
            (downsample.map(|d| d.mode), sample_type),
            (
                None | Some(DownsampleKind::Aggregate | DownsampleKind::Decimate),
                SampleType::I8 | SampleType::I16
            ) | (Some(DownsampleKind::Average), SampleType::I16)
                | (Some(DownsampleKind::Sum), SampleType::I32)
        );
        if !fits {
            return Err(PicoError::from_status(
                PicoStatus::RATIO_MODE_DOES_NOT_SUPPORT_DATATYPE,
                "set_data_buffer",
            ));
        }
        let data_type = match sample_type {
            SampleType::I8 => enPicoDataType_PICO_INT8_T,
            SampleType::I16 => enPicoDataType_PICO_INT16_T,
            SampleType::I32 => enPicoDataType_PICO_INT32_T,
        };
        Ok(PicoStreamMode {
            ratio: ratio as u64,
            mode,
            data_type,
        })
    }
}

/// The pointer the driver writes `buffer`'s samples through. The caller
/// keeps the buffer alive while the driver holds the pointer.
pub(crate) fn buffer_pointer(buffer: &SampleBuffer) -> *mut std::ffi::c_void {
    match buffer {
        SampleBuffer::I8(b) => b.write().as_mut_ptr() as *mut std::ffi::c_void,
        SampleBuffer::I16(b) => b.write().as_mut_ptr() as *mut std::ffi::c_void,
        SampleBuffer::I32(b) => b.write().as_mut_ptr() as *mut std::ffi::c_void,
    }
}

/// Common trait implemented for every oscilloscope driver. Each feature
/// is a trait of its own; see [`features`].
pub trait OscilloscopeDriverInternal:
    StreamingDriver
    + BlockDriver
    + RapidBlockDriver
    + TriggerDriver
    + EtsDriver
    + DigitalPortDriver
    + SigGenDriver
    + AuxIoDriver
    + ChannelExtrasDriver
    + LedDriver
    + fmt::Debug
    + Send
    + Sync
{
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
    /// Empty, or the one resolution, for a unit whose resolution is fixed.
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
    /// Resolutions of the unit that allow fewer channels than it has,
    /// each with the most channels it allows. Empty when every
    /// resolution allows every channel.
    fn get_resolution_channel_limits(&self, _handle: i16) -> Vec<(PicoResolution, u8)> {
        Vec::new()
    }
    /// The fastest the unit samples with each count of analog channels
    /// on, at each resolution. Empty where the driver cannot say.
    fn get_rate_limits(&self, _handle: i16) -> Vec<RateLimit> {
        Vec::new()
    }
    /// Where the unit takes its power from, or `None` for a unit with
    /// no choice of power.
    fn get_power_source(&self, _handle: i16) -> Option<PowerSource> {
        None
    }
    /// Tell the driver which power to run on. Settings on the unit are
    /// lost, so channels and streams are set up again after it.
    fn set_power_source(&self, _handle: i16, _power: PowerSource) -> PicoResult<()> {
        Err(PicoError::from_status(
            PicoStatus::POWER_SUPPLY_REQUEST_INVALID,
            "set_power_source",
        ))
    }
    /// Channels that run only on the external supply.
    fn get_supply_channels(&self, _handle: i16) -> Vec<PicoChannel> {
        Vec::new()
    }
    /// What the unit can do beyond streaming. Every feature is off by
    /// default.
    fn capabilities(&self, _handle: i16) -> PicoResult<ScopeCapabilities> {
        Ok(ScopeCapabilities::default())
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
    /// Remove every data buffer given to the driver. Drivers that add
    /// buffers, rather than replace them per channel, keep them across
    /// streams and refuse a buffer of another length.
    fn clear_data_buffers(&self, _handle: i16) -> PicoResult<()> {
        Ok(())
    }
    /// Stops a stream or a block capture
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

/// The power each open handle of a ps3000a, ps4000a or ps5000a driver
/// runs on, as open and `ChangePowerSource` left it. Their
/// `CurrentPowerSource` says `OK` where a unit has one power state,
/// which hides a USB 3 unit on a USB 2 port, so the state from open is
/// kept as well.
#[derive(Default)]
pub(crate) struct PowerStates(RwLock<HashMap<i16, PowerSource>>);

impl PowerStates {
    /// The status an open returned, before `ChangePowerSource`.
    pub(crate) fn opened(&self, handle: i16, status: PicoStatus) {
        let mut states = self.0.write();
        match PowerSource::from_status(status) {
            Some(power) => states.insert(handle, power),
            None => states.remove(&handle),
        };
    }

    pub(crate) fn set(&self, handle: i16, power: PowerSource) {
        self.0.write().insert(handle, power);
    }

    pub(crate) fn closed(&self, handle: i16) {
        self.0.write().remove(&handle);
    }

    /// The power from what `CurrentPowerSource` returned, else from
    /// open or the last change.
    pub(crate) fn current(&self, handle: i16, status: PicoStatus) -> Option<PowerSource> {
        PowerSource::from_status(status).or_else(|| self.0.read().get(&handle).copied())
    }
}

/// The fastest rate with the first 1 to `channels` analog channels on
/// at each of `resolutions`, from `min_interval`, which gives the
/// driver's shortest interval in seconds for a set of channel flags
/// (bit 0 for channel A) at a resolution, or `None` where it refuses.
pub(crate) fn rate_limits(
    resolutions: &[PicoResolution],
    channels: u8,
    mut min_interval: impl FnMut(u32, PicoResolution) -> Option<f64>,
) -> Vec<RateLimit> {
    let mut limits = Vec::new();
    for resolution in resolutions {
        for on in 1..=channels.min(31) {
            let flags = (1u32 << on) - 1;
            match min_interval(flags, *resolution) {
                Some(interval) if interval > 0.0 => limits.push(RateLimit {
                    resolution: Some(*resolution),
                    channels: on,
                    max_samples_per_second: 1.0 / interval,
                }),
                _ => {}
            }
        }
    }
    limits
}

/// The number of analog channels a variant name gives: its second
/// character on every scope family.
pub(crate) fn variant_channel_count(variant: &str) -> Option<u8> {
    variant.get(1..2)?.parse().ok()
}

/// The family's code for a channel's bandwidth `limit`, refusing one
/// the model does not list in `offered`.
pub(crate) fn bandwidth_code<T: Copy>(
    limit: Option<BandwidthLimit>,
    offered: &[BandwidthLimit],
    full: T,
    codes: &[(BandwidthLimit, T)],
) -> PicoResult<T> {
    let Some(limit) = limit else {
        return Ok(full);
    };
    codes
        .iter()
        .find(|(l, _)| *l == limit && offered.contains(l))
        .map(|(_, code)| *code)
        .ok_or_else(|| {
            PicoError::from_status(
                PicoStatus::BANDWIDTH_NOT_SUPPORTED,
                &format!("bandwidth limit {limit}"),
            )
        })
}

/// Refuses a stream of `enabled` channels at `resolution` when `limits`
/// allow fewer.
pub fn check_channel_limit(
    limits: &[(PicoResolution, u8)],
    resolution: PicoResolution,
    enabled: usize,
) -> PicoResult<()> {
    match limits.iter().find(|(r, _)| *r == resolution) {
        Some((_, most)) if enabled > *most as usize => Err(PicoError::from_status(
            PicoStatus::INVALID_NUMBER_CHANNELS_FOR_RESOLUTION,
            &format!("{resolution} allows at most {most} channels, {enabled} are on"),
        )),
        _ => Ok(()),
    }
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
    fn each_family_gets_its_own_average_code() {
        use pico_sys_dynamic::{ps2000a, ps3000a, ps4000, ps4000a, ps5000a, ps6000};
        let avg = Some(Downsample::average(16));
        assert_eq!(
            average_only(
                avg,
                ps2000a::enPS2000ARatioMode_PS2000A_RATIO_MODE_NONE,
                ps2000a::enPS2000ARatioMode_PS2000A_RATIO_MODE_AVERAGE
            )
            .unwrap(),
            (16, 4)
        );
        assert_eq!(
            average_only(
                avg,
                ps3000a::enPS3000ARatioMode_PS3000A_RATIO_MODE_NONE,
                ps3000a::enPS3000ARatioMode_PS3000A_RATIO_MODE_AVERAGE
            )
            .unwrap(),
            (16, 4)
        );
        assert_eq!(
            average_only(
                avg,
                ps4000a::enPS4000ARatioMode_PS4000A_RATIO_MODE_NONE,
                ps4000a::enPS4000ARatioMode_PS4000A_RATIO_MODE_AVERAGE
            )
            .unwrap(),
            (16, 4)
        );
        assert_eq!(
            average_only(
                avg,
                ps5000a::enPS5000ARatioMode_PS5000A_RATIO_MODE_NONE,
                ps5000a::enPS5000ARatioMode_PS5000A_RATIO_MODE_AVERAGE
            )
            .unwrap(),
            (16, 4)
        );
        // The ps4000 and ps6000 headers number average 2, not 4.
        assert_eq!(
            average_only(
                avg,
                ps4000::enRatioMode_RATIO_MODE_NONE,
                ps4000::enRatioMode_RATIO_MODE_AVERAGE
            )
            .unwrap(),
            (16, 2)
        );
        assert_eq!(
            average_only(
                avg,
                ps6000::enPS6000RatioMode_PS6000_RATIO_MODE_NONE,
                ps6000::enPS6000RatioMode_PS6000_RATIO_MODE_AVERAGE
            )
            .unwrap(),
            (16, 2)
        );
        assert_eq!(average_only(None, 0u32, 4).unwrap(), (1, 0));
        assert_eq!(
            average_only(Some(Downsample::average(1)), 0u32, 4).unwrap(),
            (1, 0)
        );
        assert_eq!(
            average_only(Some(Downsample::sum(4)), 0u32, 4)
                .unwrap_err()
                .status,
            PicoStatus::RATIO_MODE_NOT_SUPPORTED
        );
    }

    #[test]
    fn pico_stream_modes_match_buffer_types() {
        use pico_sys_dynamic::psospa::*;
        let raw8 = PicoStreamMode::new(None, SampleType::I8).unwrap();
        assert_eq!(raw8.mode, enPicoRatioMode_PICO_RATIO_MODE_RAW);
        assert_eq!(raw8.data_type, enPicoDataType_PICO_INT8_T);
        assert_eq!(raw8.ratio, 1);

        let avg = PicoStreamMode::new(Some(Downsample::average(32)), SampleType::I16).unwrap();
        assert_eq!(
            (avg.ratio, avg.mode, avg.data_type),
            (
                32,
                enPicoRatioMode_PICO_RATIO_MODE_AVERAGE,
                enPicoDataType_PICO_INT16_T
            )
        );

        let sum = PicoStreamMode::new(Some(Downsample::sum(32)), SampleType::I32).unwrap();
        assert_eq!(
            (sum.ratio, sum.mode, sum.data_type),
            (
                32,
                enPicoRatioMode_PICO_RATIO_MODE_SUM,
                enPicoDataType_PICO_INT32_T
            )
        );

        for (downsample, sample_type) in [
            (None, SampleType::I32),
            (Some(Downsample::average(4)), SampleType::I8),
            (Some(Downsample::sum(4)), SampleType::I16),
        ] {
            assert_eq!(
                PicoStreamMode::new(downsample, sample_type)
                    .unwrap_err()
                    .status,
                PicoStatus::RATIO_MODE_DOES_NOT_SUPPORT_DATATYPE
            );
        }
    }

    #[test]
    fn sample_buffers_hold_their_type() {
        let buffer = SampleBuffer::new(SampleType::I8, 10);
        assert_eq!((buffer.sample_type(), buffer.len()), (SampleType::I8, 10));
        assert!(buffer.as_i16("test").is_err());
        assert!(SampleBuffer::new(SampleType::I16, 1).as_i16("test").is_ok());
    }

    #[test]
    fn power_states_keep_open_state_where_the_driver_says_ok() {
        let states = PowerStates::default();
        // A 4-channel unit on USB power.
        states.opened(1, PicoStatus::POWER_SUPPLY_NOT_CONNECTED);
        assert_eq!(states.current(1, PicoStatus::OK), Some(PowerSource::Usb));
        assert_eq!(
            states.current(1, PicoStatus::POWER_SUPPLY_CONNECTED),
            Some(PowerSource::External)
        );
        // A 2-channel USB 3 unit on a USB 2 port reads OK later.
        states.opened(2, PicoStatus::USB3_0_DEVICE_NON_USB3_0_PORT);
        assert_eq!(
            states.current(2, PicoStatus::OK),
            Some(PowerSource::UsbNonUsb3Port)
        );
        // One with no choice of power.
        states.opened(3, PicoStatus::OK);
        assert_eq!(states.current(3, PicoStatus::OK), None);
        states.set(3, PowerSource::External);
        assert_eq!(
            states.current(3, PicoStatus::OK),
            Some(PowerSource::External)
        );
        states.closed(3);
        assert_eq!(states.current(3, PicoStatus::OK), None);
    }

    #[test]
    fn variant_names_give_their_channel_count() {
        assert_eq!(variant_channel_count("5444D"), Some(4));
        assert_eq!(variant_channel_count("3206D MSO"), Some(2));
        assert_eq!(variant_channel_count("6824E"), Some(8));
        assert_eq!(variant_channel_count(""), None);
    }

    #[test]
    fn rate_limits_skip_refused_sets() {
        let limits = rate_limits(&[PicoResolution::DR_12BIT], 4, |flags, _| {
            (flags.count_ones() <= 2).then_some(1e-9)
        });
        assert_eq!(limits.len(), 2);
        assert_eq!(limits[1].channels, 2);
        assert_eq!(limits[1].max_samples_per_second.round(), 1e9);
    }

    #[test]
    fn parse_enum_result_tolerates_an_empty_or_malformed_list() {
        assert!(parse_enum_result(&[], 0).is_empty());
        assert!(parse("").is_empty());
        assert!(parse("garbage").is_empty());
    }
}
