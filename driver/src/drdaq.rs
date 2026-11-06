//! The USB DrDAQ educational data logger driver
//!
//! - The driver keeps a stream buffer.
//!   `UsbDrDaqRun` with `BM_STREAM` starts a continuous capture, and each `UsbDrDaqGetValues` or `UsbDrDaqGetValuesF` call returns the values since the last call (USB DrDAQ Programmer's Guide, usbdrdaqpg.en r9, section 2.5 Capture modes).
//! - The channels are fixed sensors on the board, and the driver scales the value of each sensor.
//!   You may still need to divide a value by the divider of the channel from `UsbDrDaqGetChannelInfo` (sections 2.6, 3.3 and 3.12).
//!   [`DrDAQChannelInfo::scale`] does that.
//! - The wrapper covers what a continuous stream of the channels needs, and the outputs: GPIO, PWM, the signal generator and the RGB LED ([`outputs`], [`DrDAQDriverInternal::set_output`]).
//!   It does not wrap the digital inputs or the pulse counter.

mod outputs;

pub use outputs::{outputs, sig_gen_info};

use crate::{DriverLoadError, LibraryResolution};
use pico_common::{
    DrDAQChannel, DrDAQChannelInfo, DrDAQInfo, Driver, FromPicoStr, PicoError, PicoInfo,
    PicoResult, PicoStatus,
};
use pico_sys_dynamic::drdaq::{
    DrDAQLoader, _BLOCK_METHOD_BM_SINGLE, _BLOCK_METHOD_BM_STREAM, BLOCK_METHOD,
};
use std::{fmt, iter, ops::Deref, path::Path, sync::Arc};

/// The methods of the DrDAQ driver
///
/// - Get it through [`DrDAQDriver`], which dereferences to it.
/// - Each method that takes a `handle` needs a unit that [`DrDAQDriverInternal::open_unit`] opened.
pub struct DrDAQDriverInternal {
    bindings: crate::Kept<DrDAQLoader>,
}

impl DrDAQDriverInternal {
    /// Reads a string info field from the device.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn get_info_string(&self, handle: i16, info_type: u32) -> PicoResult<String> {
        let mut buffer = vec![0i8; 256];
        let mut required_size: i16 = 0;

        let status = unsafe {
            self.bindings.UsbDrDaqGetUnitInfo(
                handle,
                buffer.as_mut_ptr(),
                buffer.len() as i16,
                &mut required_size as *mut i16,
                info_type,
            )
        };

        PicoStatus::from(status).to_result((), "get_info_string")?;
        Ok(buffer.from_pico_i8_string(required_size as usize))
    }

    /// Opens the next unit that is not open, and returns its handle
    ///
    /// - Returns Err with `PicoStatus::NOT_FOUND` when all connected units are open.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let mut handle: i16 = 0;

        let status = unsafe { self.bindings.UsbDrDaqOpenUnit(&mut handle as *mut i16) };

        match PicoStatus::from(status) {
            PicoStatus::OK if handle > 0 => Ok(handle),
            PicoStatus::OK | PicoStatus::NOT_FOUND | PicoStatus::NOT_RESPONDING => {
                Err(PicoStatus::NOT_FOUND)
            }
            status => Err(status),
        }
    }

    /// Opens each connected unit that is not open, and gives its handle
    ///
    /// - You own each handle, so close the units that you do not keep.
    /// - It stops after the first error, as an open that fails would fail again.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        let mut done = false;
        iter::from_fn(move || {
            if done {
                return None;
            }
            match self.open_next_unit() {
                Ok(handle) => Some(Ok(handle)),
                Err(PicoStatus::NOT_FOUND) => None,
                Err(status) => {
                    done = true;
                    Some(Err(PicoError::from_status(status, "open_next_unit")))
                }
            }
        })
    }

    /// Opens the unit with `serial`, or the next unit with `None`, and returns its handle
    ///
    /// - A unit matches when its batch and serial string is `serial`, without white space at the ends.
    /// - To find a serial, it must open other units.
    ///   It closes them again before it returns.
    /// - Returns Err with `NOT_FOUND` when no unit matches.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        let mut handles_to_close = Vec::new();

        let result = loop {
            match self.open_next_unit() {
                Ok(handle) => match serial {
                    None => break Ok(handle),
                    Some(serial) => {
                        match self.get_info_string(handle, u32::from(PicoInfo::BATCH_AND_SERIAL)) {
                            Ok(found) if crate::serial_matches(&found, serial) => break Ok(handle),
                            Ok(_) => handles_to_close.push(handle),
                            Err(error) => {
                                handles_to_close.push(handle);
                                break Err(error);
                            }
                        }
                    }
                },
                Err(status) => break Err(PicoError::from_status(status, "open_unit")),
            }
        };

        for handle in handles_to_close {
            let _ = self.close_unit(handle);
        }

        result
    }

    /// The serial, variant and hardware version of the unit
    ///
    /// - The variant and the version are empty when the driver does not give them.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16) -> PicoResult<DrDAQInfo> {
        let serial = self.get_info_string(handle, u32::from(PicoInfo::BATCH_AND_SERIAL))?;
        let variant = self
            .get_info_string(handle, u32::from(PicoInfo::VARIANT_INFO))
            .unwrap_or_default();
        let hardware_version = self
            .get_info_string(handle, u32::from(PicoInfo::HARDWARE_VERSION))
            .unwrap_or_default();

        Ok(DrDAQInfo {
            handle: Arc::new(handle),
            serial,
            variant,
            hardware_version,
        })
    }

    /// Checks that the unit is still connected
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.UsbDrDaqPingUnit(handle) };
        PicoStatus::from(status).to_result((), "ping_unit")
    }

    /// The range, the decimal places and the divider of the scale of a channel (section 3.3)
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn channel_info(&self, handle: i16, channel: DrDAQChannel) -> PicoResult<DrDAQChannelInfo> {
        let mut min: f32 = 0.0;
        let mut max: f32 = 0.0;
        let mut places: i16 = 0;
        let mut divider: i16 = 0;

        let status = unsafe {
            self.bindings.UsbDrDaqGetChannelInfo(
                handle,
                &mut min as *mut f32,
                &mut max as *mut f32,
                &mut places as *mut i16,
                &mut divider as *mut i16,
                u32::from(channel),
            )
        };

        PicoStatus::from(status).to_result(
            DrDAQChannelInfo {
                min,
                max,
                decimal_places: places,
                divider,
            },
            "channel_info",
        )
    }

    /// Sets the channels to collect and the interval between sample sets
    ///
    /// - Returns the interval in microseconds that the driver set.
    /// - `UsbDrDaqSetInterval` takes the time for a block of `ideal_samples` for each channel, and returns the time it set (usbdrdaqpg.en r9, section 3.21).
    ///   So the wrapper multiplies the interval by `ideal_samples`, and divides the result by it.
    /// - `ideal_samples` is only for that calculation.
    ///   It does not limit a stream.
    /// - In `BM_STREAM` mode the shortest interval between two samples is 10 µs for each channel.
    /// - Returns Err with `INVALID_SAMPLE_INTERVAL` when the block time does not fit in a `u32`.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_interval(
        &self,
        handle: i16,
        channels: &[DrDAQChannel],
        interval_us: u32,
        ideal_samples: u32,
    ) -> PicoResult<u32> {
        let ideal_samples = ideal_samples.max(1);
        let mut us_for_block = interval_us.checked_mul(ideal_samples).ok_or_else(|| {
            PicoError::from_status(PicoStatus::INVALID_SAMPLE_INTERVAL, "set_interval")
        })?;
        let mut channel_ids: Vec<u32> = channels.iter().map(|&c| u32::from(c)).collect();

        let status = unsafe {
            self.bindings.UsbDrDaqSetInterval(
                handle,
                &mut us_for_block as *mut u32,
                ideal_samples,
                channel_ids.as_mut_ptr(),
                channel_ids.len() as i16,
            )
        };

        PicoStatus::from(status).to_result(
            interval_from_block(us_for_block, ideal_samples),
            "set_interval",
        )
    }

    /// Starts to collect samples into the driver buffer
    ///
    /// - With `stream` set to `true` it runs continuously (`BM_STREAM`).
    ///   With `false` it collects one block (`BM_SINGLE`) of `num_values` samples for each channel.
    /// - In a stream, `num_values` must hold the samples that come between two calls to [`Self::get_values`].
    /// - The total for all channels is at most 1,000,010 samples (sections 2.5 and 3.19).
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn run(&self, handle: i16, num_values: u32, stream: bool) -> PicoResult<()> {
        let method: BLOCK_METHOD = if stream {
            _BLOCK_METHOD_BM_STREAM
        } else {
            _BLOCK_METHOD_BM_SINGLE
        };

        let status = unsafe { self.bindings.UsbDrDaqRun(handle, num_values, method) };
        PicoStatus::from(status).to_result((), "run")
    }

    /// Reads the samples since the last call
    ///
    /// - Returns the values from `UsbDrDaqGetValuesF`, before the divider of the channel.
    /// - Each sample set has one value for each channel, in channel number order, as `[ch0_t0, ch1_t0, ch0_t1, ch1_t1, ...]`.
    ///   The order does not depend on the order in which you set the channels.
    /// - Also returns an over-range bit field by channel number.
    ///   Bit 0 is channel 1, and a bit is set when its channel is over range in one or more of the samples (sections 3.11 and 3.12).
    /// - Returns no values when the driver has no new samples.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_values(
        &self,
        handle: i16,
        num_channels: usize,
        max_sample_sets: u32,
    ) -> PicoResult<(Vec<f32>, u16)> {
        if num_channels == 0 {
            return Ok((Vec::new(), 0));
        }

        let mut values = vec![0f32; num_channels * max_sample_sets as usize];
        let mut num_values = max_sample_sets;
        let mut overflow: u16 = 0;
        let mut trigger_index: u32 = 0;

        let status = unsafe {
            self.bindings.UsbDrDaqGetValuesF(
                handle,
                values.as_mut_ptr(),
                &mut num_values as *mut u32,
                &mut overflow as *mut u16,
                &mut trigger_index as *mut u32,
            )
        };

        match PicoStatus::from(status) {
            PicoStatus::NO_SAMPLES_AVAILABLE => Ok((Vec::new(), 0)),
            PicoStatus::OK | PicoStatus::WARNING_REPEAT_VALUE => {
                values.truncate(num_channels * num_values as usize);
                Ok((values, overflow))
            }
            status => Err(PicoError::from_status(status, "get_values")),
        }
    }

    /// Stops the capture
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.UsbDrDaqStop(handle) };
        PicoStatus::from(status).to_result((), "stop")
    }

    /// Closes the unit
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.UsbDrDaqCloseUnit(handle) };
        PicoStatus::from(status).to_result((), "close_unit")
    }
}

/// The interval between sample sets from the time `UsbDrDaqSetInterval` gives for a block,
/// rounded to the nearest microsecond
fn interval_from_block(us_for_block: u32, ideal_samples: u32) -> u32 {
    ((us_for_block as u64 + ideal_samples as u64 / 2) / ideal_samples as u64) as u32
}

/// Checks that the library has each function that this wrapper calls
fn required(b: &DrDAQLoader) -> Result<(), DriverLoadError> {
    crate::required_functions!(
        b,
        [
            UsbDrDaqOpenUnit,
            UsbDrDaqCloseUnit,
            UsbDrDaqGetUnitInfo,
            UsbDrDaqRun,
            UsbDrDaqStop,
            UsbDrDaqSetInterval,
            UsbDrDaqGetValuesF,
            UsbDrDaqSetSigGenBuiltIn,
            UsbDrDaqSetSigGenArbitrary,
            UsbDrDaqStopSigGen,
            UsbDrDaqSetDO,
            UsbDrDaqSetPWM,
            UsbDrDaqEnableRGBLED,
            UsbDrDaqSetRGBLED,
            UsbDrDaqGetChannelInfo,
            UsbDrDaqPingUnit,
        ]
    )
}

/// A loaded USB DrDAQ driver
///
/// - It dereferences to [`DrDAQDriverInternal`], which has the methods.
/// - Clones share the loaded library.
#[derive(Clone)]
pub struct DrDAQDriver(Arc<DrDAQDriverInternal>);

impl DrDAQDriver {
    /// Loads the library file at `path`
    ///
    /// - The library stays loaded until the process ends.
    /// - Returns Err when the library does not load, or lacks a function that this wrapper calls.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DriverLoadError> {
        let bindings = crate::Kept::new(unsafe { DrDAQLoader::new(path.as_ref())? });
        required(&bindings)?;
        Ok(DrDAQDriver(Arc::new(DrDAQDriverInternal { bindings })))
    }

    /// Loads the library that `resolution` finds
    ///
    /// - Returns Err when the library does not load, or lacks a function that this wrapper calls.
    pub fn load(resolution: &LibraryResolution) -> Result<Self, DriverLoadError> {
        DrDAQDriver::new(resolution.get_path(Driver::DrDAQ))
    }
}

impl fmt::Debug for DrDAQDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DrDAQDriver").finish()
    }
}

impl Deref for DrDAQDriver {
    type Target = DrDAQDriverInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_block_time_divides_into_the_interval() {
        assert_eq!(interval_from_block(2_000_000, 1_000), 2_000);
        assert_eq!(interval_from_block(10_499, 1_000), 10);
    }
}
