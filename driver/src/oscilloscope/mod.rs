//! The oscilloscope drivers, with one API for every range
//!
//! Each range has its own C driver with a different API.
//! Each wrapper, such as [`PS2000ADriver`], implements [`OscilloscopeDriverInternal`] and the traits in [`features`].
//! [`OscilloscopeDriver`] holds any of them, so the code that uses it does not depend on the range.
//!
//! - Each call on a unit takes the handle that [`OscilloscopeDriverInternal::open_unit`] gave.
//! - A feature that the range or the model does not have returns [`PicoError::not_supported`].
//!   [`OscilloscopeDriverInternal::capabilities`] tells which features a unit has.

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

use crate::{DriverLoadError, LibraryResolution};
use parking_lot::RwLock;
use pico_common::{
    BandwidthLimit, Downsample, DownsampleKind, Driver, FromPicoStr, OscilloscopeChannelConfig,
    PicoChannel, PicoError, PicoInfo, PicoRange, PicoResolution, PicoResult, PicoStatus,
    PowerSource, ProbeState, RateLimit, SampleType, ScopeCapabilities, ScopeDriver,
};
use std::{collections::HashMap, fmt, ops::Deref, sync::Arc};
use version_compare::Version;

/// One unit that [`OscilloscopeDriverInternal::enumerate_units`] found
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct EnumerationResult {
    /// The model name, such as `2206B`
    pub variant: String,
    /// The batch and serial number, such as `CW356/055`
    pub serial: String,
}

/// A buffer that the driver writes samples into
///
/// - The variant must be a type that the driver takes, from [`StreamingDriver::sample_types`].
/// - Clones share the same memory.
/// - The driver keeps a pointer to the memory, so keep the buffer and do not change its length while the driver holds it.
#[derive(Clone, Debug)]
pub enum SampleBuffer {
    /// 8-bit samples
    I8(Arc<RwLock<Vec<i8>>>),
    /// 16-bit samples
    I16(Arc<RwLock<Vec<i16>>>),
    /// 32-bit samples, for a sum downsample
    I32(Arc<RwLock<Vec<i32>>>),
}

impl SampleBuffer {
    /// Makes a buffer of `len` samples of `sample_type`, all zero
    pub fn new(sample_type: SampleType, len: usize) -> Self {
        match sample_type {
            SampleType::I8 => SampleBuffer::I8(Arc::new(RwLock::new(vec![0; len]))),
            SampleType::I16 => SampleBuffer::I16(Arc::new(RwLock::new(vec![0; len]))),
            SampleType::I32 => SampleBuffer::I32(Arc::new(RwLock::new(vec![0; len]))),
        }
    }

    /// The type of the samples
    pub fn sample_type(&self) -> SampleType {
        match self {
            SampleBuffer::I8(_) => SampleType::I8,
            SampleBuffer::I16(_) => SampleType::I16,
            SampleBuffer::I32(_) => SampleType::I32,
        }
    }

    /// The number of samples
    pub fn len(&self) -> usize {
        match self {
            SampleBuffer::I8(b) => b.read().len(),
            SampleBuffer::I16(b) => b.read().len(),
            SampleBuffer::I32(b) => b.read().len(),
        }
    }

    /// Tells if the buffer has no samples
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The `i16` memory, for drivers that take no other type
    ///
    /// - Returns Err with `RATIO_MODE_DOES_NOT_SUPPORT_DATATYPE` when the buffer is not [`SampleBuffer::I16`].
    ///   `context` goes into the error.
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

/// The calls that every oscilloscope driver has
///
/// - Each feature, such as block mode or the trigger, is a trait of its own in [`features`].
/// - Each method that takes a `handle` needs a unit that [`OscilloscopeDriverInternal::open_unit`] opened.
/// - The default methods are for a range that does not have the feature.
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
    /// The driver this wrapper is for
    fn get_driver(&self) -> Driver;
    /// The version of the driver library, such as `2.1.30.1878`
    ///
    /// - It needs no open unit.
    fn get_version(&self) -> PicoResult<String>;
    /// The path of the loaded library file, as the driver gives it
    ///
    /// - Returns `None` when the driver does not give its path.
    fn get_path(&self) -> PicoResult<Option<String>>;
    /// Lists the units that the driver finds
    ///
    /// - The ps2000 driver has no list call, so its wrapper opens each unit to read its serial, then closes it.
    ///   It does not find a unit that is open.
    ///
    /// ```no_run
    /// use pico_common::ScopeDriver;
    /// use pico_driver::{oscilloscope::OscilloscopeDriver, LibraryResolution};
    ///
    /// let scope = OscilloscopeDriver::load(ScopeDriver::PS5000A, &LibraryResolution::Default)?;
    /// for unit in scope.enumerate_units()? {
    ///     println!("{} {}", unit.variant, unit.serial);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>>;
    /// Opens the unit with `serial`, or the first unit with `None`, and returns its handle
    ///
    /// - Close the unit with [`OscilloscopeDriverInternal::close`].
    /// - Returns Err with `NOT_FOUND` when no unit matches.
    ///
    /// ```no_run
    /// use pico_common::{PicoInfo, ScopeDriver};
    /// use pico_driver::{oscilloscope::OscilloscopeDriver, LibraryResolution};
    ///
    /// let scope = OscilloscopeDriver::load(ScopeDriver::PS2000A, &LibraryResolution::Default)?;
    /// let handle = scope.open_unit(Some("CW356/055"))?;
    /// println!("{}", scope.get_unit_info(handle, PicoInfo::VARIANT_INFO)?);
    /// scope.close(handle)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16>;
    /// Checks that the unit is still connected
    fn ping_unit(&self, handle: i16) -> PicoResult<()>;
    /// The ADC count at the top of the range
    ///
    /// - Use it to scale samples to volts.
    /// - On a range with a choice of resolution, it changes with the resolution.
    fn maximum_value(&self, handle: i16) -> PicoResult<i16>;
    /// Closes the unit
    fn close(&self, handle: i16) -> PicoResult<()>;
    /// One information string of the unit
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String>;
    /// The ranges that `channel` takes
    ///
    /// - Leaves out each range code that [`PicoRange`] does not name.
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>>;
    /// The resolutions the unit has, each with its maximum ADC value
    ///
    /// - Empty, or one resolution, for a unit with a fixed resolution.
    fn get_resolutions(&self, _handle: i16) -> PicoResult<Vec<(PicoResolution, i16)>> {
        Ok(Vec::new())
    }
    /// The resolution of the unit now
    ///
    /// - Returns `None` when the resolution is fixed.
    fn get_resolution(&self, _handle: i16) -> Option<PicoResolution> {
        None
    }
    /// Sets the resolution of the unit
    ///
    /// - The change starts with the next stream.
    /// - Returns Err with `RESOLUTION_NOT_SUPPORTED_BY_VARIANT` on a unit with a fixed resolution.
    fn set_resolution(&self, _handle: i16, _resolution: PicoResolution) -> PicoResult<()> {
        Err(PicoError::from_status(
            PicoStatus::RESOLUTION_NOT_SUPPORTED_BY_VARIANT,
            "set_resolution",
        ))
    }
    /// The resolutions that allow fewer channels than the unit has, each with the most channels it allows
    ///
    /// - Empty when each resolution allows all channels.
    /// - [`check_channel_limit`] uses this list.
    fn get_resolution_channel_limits(&self, _handle: i16) -> Vec<(PicoResolution, u8)> {
        Vec::new()
    }
    /// The fastest sample rate for each number of analog channels on, at each resolution
    ///
    /// - Empty when the driver cannot tell.
    fn get_rate_limits(&self, _handle: i16) -> Vec<RateLimit> {
        Vec::new()
    }
    /// The power source of the unit
    ///
    /// - Returns `None` for a unit with no choice of power.
    fn get_power_source(&self, _handle: i16) -> Option<PowerSource> {
        None
    }
    /// Tells the driver which power source to use
    ///
    /// - The unit loses its settings, so set up the channels and the stream again after this call.
    /// - Returns Err with `POWER_SUPPLY_REQUEST_INVALID` on a unit with no choice of power.
    fn set_power_source(&self, _handle: i16, _power: PowerSource) -> PicoResult<()> {
        Err(PicoError::from_status(
            PicoStatus::POWER_SUPPLY_REQUEST_INVALID,
            "set_power_source",
        ))
    }
    /// The channels that work only on the external power supply
    fn get_supply_channels(&self, _handle: i16) -> Vec<PicoChannel> {
        Vec::new()
    }
    /// The features of the unit other than streaming
    ///
    /// - The default has all features off.
    fn capabilities(&self, _handle: i16) -> PicoResult<ScopeCapabilities> {
        Ok(ScopeCapabilities::default())
    }
    /// The probe state of each channel that the unit gave one for
    ///
    /// - Empty on a unit that does not detect probes.
    /// - A channel with a probe takes the ranges of the probe in [`get_channel_ranges`](Self::get_channel_ranges).
    fn probes(&self, _handle: i16) -> Vec<ProbeState> {
        Vec::new()
    }
    /// The probe states that the unit gave since the last call, oldest first
    ///
    /// - Each entry is the new state of one channel.
    fn take_probe_changes(&self, _handle: i16) -> Vec<ProbeState> {
        Vec::new()
    }
    /// Turns on a channel with `config`
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
    ) -> PicoResult<()>;
    /// Turns off a channel
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()>;
    /// Removes all the data buffers that the driver holds for the unit
    ///
    /// - Some drivers add buffers instead of replacing the buffer of a channel.
    ///   They keep the buffers from one stream to the next, and refuse a buffer of a different length.
    ///   Call this before you give new buffers to such a driver.
    fn clear_data_buffers(&self, _handle: i16) -> PicoResult<()> {
        Ok(())
    }
    /// Stops a stream or a block capture
    fn stop(&self, handle: i16) -> PicoResult<()>;
    /// Checks that the library is not older than the oldest version this crate was tested with
    ///
    /// - The ps2000 driver needs 3.0.30.1878 or later, ps6000a needs 1.0.54.2438 or later, and the other ranges need 2.1.30.1878 or later.
    ///   There is no minimum for psospa.
    /// - [`DriverLoad::load`](crate::DriverLoad::load) does not call it.
    /// - Returns Err with [`DriverLoadError::VersionError`] when the library is older.
    fn check_version(&self) -> Result<(), DriverLoadError> {
        let loaded_str = &self.get_version()?;
        let loaded_version = Version::from(loaded_str);

        let driver = self.get_driver();
        let Some(required_str) = min_required_version(driver) else {
            tracing::debug!(?driver, version = %loaded_str, "no minimum version to check");
            return Ok(());
        };
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

/// A loaded oscilloscope driver of any range
///
/// - It dereferences to [`OscilloscopeDriverInternal`], so call the methods of that trait and of the [`features`] traits on it.
/// - Clones share the loaded library.
#[derive(Clone)]
pub struct OscilloscopeDriver(Arc<dyn OscilloscopeDriverInternal>);

impl OscilloscopeDriver {
    /// Wraps the driver of one range, such as [`PS2000ADriver`]
    pub fn new<D: OscilloscopeDriverInternal + 'static>(driver: D) -> Self {
        OscilloscopeDriver(Arc::new(driver))
    }

    /// Loads the library of `driver` that `resolution` finds
    ///
    /// - Use [`DriverLoad::load`](crate::DriverLoad::load) instead when the program picks the driver when it runs.
    /// - It does not check the driver version.
    /// - Returns Err when the library does not load.
    ///
    /// ```no_run
    /// use pico_common::ScopeDriver;
    /// use pico_driver::{oscilloscope::OscilloscopeDriver, LibraryResolution};
    ///
    /// let driver = OscilloscopeDriver::load(ScopeDriver::PS2000A, &LibraryResolution::Default)?;
    /// println!("{driver:?}");
    /// # Ok::<(), pico_driver::DriverLoadError>(())
    /// ```
    pub fn load(
        driver: ScopeDriver,
        resolution: &LibraryResolution,
    ) -> Result<Self, DriverLoadError> {
        let path = resolution.get_path(Driver::from(driver));

        Ok(match driver {
            ScopeDriver::PS2000 => Self::new(PS2000Driver::new(path)?),
            ScopeDriver::PS2000A => Self::new(PS2000ADriver::new(path)?),
            ScopeDriver::PS3000A => Self::new(PS3000ADriver::new(path)?),
            ScopeDriver::PS4000 => Self::new(PS4000Driver::new(path)?),
            ScopeDriver::PS4000A => Self::new(PS4000ADriver::new(path)?),
            ScopeDriver::PS5000A => Self::new(PS5000ADriver::new(path)?),
            ScopeDriver::PS6000 => Self::new(PS6000Driver::new(path)?),
            ScopeDriver::PS6000A => Self::new(PS6000ADriver::new(path)?),
            ScopeDriver::PSOSPA => Self::new(PSOSPADriver::new(path)?),
        })
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

/// The range for a range code that the driver lists for a channel
///
/// - Returns Err with `INVALID_VOLTAGE_RANGE` for a code that [`PicoRange`] does not name.
/// - The error text shows the code.
pub(crate) fn channel_range(code: i32) -> PicoResult<PicoRange> {
    PicoRange::try_from(code).map_err(|e| {
        PicoError::from_status(
            PicoStatus::INVALID_VOLTAGE_RANGE,
            &format!("get_channel_ranges: {e}"),
        )
    })
}

/// The ranges for the range codes that the driver lists for a channel
///
/// - Skips each code that [`PicoRange`] does not name, and logs a warning with the channel and the code.
pub(crate) fn channel_ranges(
    channel: PicoChannel,
    codes: impl IntoIterator<Item = i32>,
) -> Vec<PicoRange> {
    codes
        .into_iter()
        .filter_map(|code| {
            let range = PicoRange::try_from(code).ok();
            if range.is_none() {
                tracing::warn!(?channel, code, "skipping unknown range code");
            }
            range
        })
        .collect()
}

/// Checks that `resolution` allows `enabled` channels
///
/// - `limits` comes from [`OscilloscopeDriverInternal::get_resolution_channel_limits`].
/// - Returns Err with `INVALID_NUMBER_CHANNELS_FOR_RESOLUTION` when `limits` allows fewer channels at `resolution`.
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

/// The minimum driver version tested with these wrappers, or `None`
/// where no minimum is known. No version is checked for `None`: a
/// guessed number could refuse a driver that works.
fn min_required_version(driver: Driver) -> Option<&'static str> {
    match driver {
        Driver::PS2000 => Some("3.0.30.1878"),
        Driver::PS2000A
        | Driver::PS3000A
        | Driver::PS4000
        | Driver::PS4000A
        | Driver::PS5000A
        | Driver::PS6000 => Some("2.1.30.1878"),
        Driver::PS6000A => Some("1.0.54.2438"),
        Driver::PSOSPA
        | Driver::PL1000
        | Driver::PLCM3
        | Driver::PT104
        | Driver::TC08
        | Driver::PicoHRDL
        | Driver::DrDAQ => None,
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
    fn channel_range_error_keeps_code() {
        let error = channel_range(999).unwrap_err();
        assert_eq!(error.status, PicoStatus::INVALID_VOLTAGE_RANGE);
        assert!(error.to_string().contains("999"));
    }

    #[test]
    fn channel_ranges_skips_unknown_codes() {
        assert_eq!(
            channel_ranges(PicoChannel::A, [2, 20, 3]),
            vec![PicoRange::X1_PROBE_50MV, PicoRange::X1_PROBE_100MV]
        );
    }

    #[test]
    fn each_family_reports_its_own_driver() {
        let families = [
            (ps2000::PS2000Driver::DRIVER, "2000"),
            (ps2000a::PS2000ADriver::DRIVER, "2000A"),
            (ps3000a::PS3000ADriver::DRIVER, "3000A"),
            (ps4000::PS4000Driver::DRIVER, "4000"),
            (ps4000a::PS4000ADriver::DRIVER, "4000A"),
            (ps5000a::PS5000ADriver::DRIVER, "5000A"),
            (ps6000::PS6000Driver::DRIVER, "6000"),
            (ps6000a::PS6000ADriver::DRIVER, "6000A"),
            (psospa::PSOSPADriver::DRIVER, "OSPA"),
        ];
        for (driver, name) in families {
            assert_eq!(driver, name.parse::<Driver>().unwrap(), "{name}");
        }
    }

    #[test]
    fn min_required_version_is_known_or_none_for_every_driver() {
        use Driver::*;
        let all = [
            PS2000, PS2000A, PS3000A, PS4000, PS4000A, PS5000A, PS6000, PS6000A, PSOSPA, PL1000,
            PLCM3, PT104, TC08, PicoHRDL, DrDAQ,
        ];
        for driver in all {
            let version = min_required_version(driver);
            assert_eq!(
                version.is_some(),
                driver.is_scope() && driver != PSOSPA,
                "{driver:?}"
            );
        }
        assert_eq!(min_required_version(PS6000A), Some("1.0.54.2438"));
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
