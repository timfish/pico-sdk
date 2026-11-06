//! The PicoLog 1000 Series data logger driver
//!
//! - The driver keeps a stream buffer.
//!   In `BM_STREAM` mode "every time pl1000GetValues() is called, it returns the new readings since the last time it was called", after `pl1000Run` starts the capture (PicoLog 1000 Series Programmer's Guide, pl1000pg-8, sections 3.1 and 4.12).
//!   Your code calls for the data; the driver does not call back.
//! - The driver has no list call.
//!   Each `pl1000OpenUnit` call opens one unit (section 4.7), and the driver takes up to 64 units (section 2.1).
//!   To find the units, open each one and close the ones you do not want.
//! - All channels share one input range, so there is no range for each channel.
//!   `pl1000MaxValue` gives the full-scale ADC value of the connected variant (sections 3.2 and 4.6).

mod outputs;

pub use outputs::outputs;

use crate::{DriverLoadError, LibraryResolution};
use pico_common::{
    Driver, FromPicoStr, PL1000Channel, PL1000Info, PicoError, PicoInfo, PicoResult, PicoStatus,
};
use pico_sys_dynamic::pl1000::{
    PL1000Loader, _BLOCK_METHOD_BM_SINGLE, _BLOCK_METHOD_BM_STREAM, BLOCK_METHOD,
};
use std::{iter, ops::Deref, path::Path, sync::Arc};

/// The methods of the PicoLog 1000 Series driver
///
/// - Get it through [`PL1000Driver`], which dereferences to it.
/// - Each method that takes a `handle` needs a unit that [`PL1000DriverInternal::open_unit`] opened.
pub struct PL1000DriverInternal {
    bindings: crate::Kept<PL1000Loader>,
}

impl PL1000DriverInternal {
    /// Reads a string info field from the device.
    ///
    /// The PL1000 API uses C-string out-parameters with a required-size out-param, the same
    /// pattern as the PT-104 and other loggers in this family of drivers.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn get_info_string(&self, handle: i16, info_type: u32) -> PicoResult<String> {
        let mut buffer = vec![0i8; 256];
        let mut required_size: i16 = 0;

        let status = unsafe {
            self.bindings.pl1000GetUnitInfo(
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

        let status = unsafe { self.bindings.pl1000OpenUnit(&mut handle as *mut i16) };

        match PicoStatus::from(status) {
            PicoStatus::OK if handle > 0 => Ok(handle),
            PicoStatus::OK => Err(PicoStatus::NOT_FOUND),
            PicoStatus::NOT_FOUND | PicoStatus::NOT_RESPONDING => Err(PicoStatus::NOT_FOUND),
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

    /// The full-scale ADC value of the fixed input range of the unit
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn max_value(&self, handle: i16) -> PicoResult<u16> {
        let mut value: u16 = 0;

        let status = unsafe { self.bindings.pl1000MaxValue(handle, &mut value as *mut u16) };

        PicoStatus::from(status).to_result(value, "max_value")
    }

    /// The serial, variant and full-scale ADC value of the unit
    ///
    /// - The variant is empty when the driver does not give it.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16) -> PicoResult<PL1000Info> {
        let serial = self.get_info_string(handle, u32::from(PicoInfo::BATCH_AND_SERIAL))?;
        let variant = self
            .get_info_string(handle, u32::from(PicoInfo::VARIANT_INFO))
            .unwrap_or_default();
        let max_value = self.max_value(handle)?;

        Ok(PL1000Info {
            handle: Arc::new(handle),
            serial,
            variant,
            max_value,
        })
    }

    /// Checks that the unit is still connected
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.pl1000PingUnit(handle) };
        PicoStatus::from(status).to_result((), "ping_unit")
    }

    /// Sets the channels to collect and the interval between sample sets
    ///
    /// - Returns the interval in microseconds that the driver set.
    /// - `pl1000SetInterval` takes the time for a block of `ideal_samples` for each channel, and returns the time it will take (pl1000pg-8, section 4.14).
    ///   So the wrapper multiplies the interval by `ideal_samples`, and divides the result by it.
    /// - `ideal_samples` is only for that calculation.
    ///   It does not limit a stream.
    /// - The guide limits `ideal_samples` multiplied by the number of channels to 1,000,000.
    /// - The wrapper uses fewer samples when the block time does not fit in a `u32`.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_interval(
        &self,
        handle: i16,
        channels: &[PL1000Channel],
        interval_us: u32,
        ideal_samples: u32,
    ) -> PicoResult<u32> {
        let ideal_samples = block_samples(interval_us, ideal_samples);
        let mut us_for_block = interval_us.checked_mul(ideal_samples).ok_or_else(|| {
            PicoError::from_status(PicoStatus::INVALID_SAMPLE_INTERVAL, "set_interval")
        })?;
        let mut channel_ids: Vec<i16> = channels.iter().map(|&c| i16::from(c)).collect();

        let status = unsafe {
            self.bindings.pl1000SetInterval(
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
    /// - In a stream, `num_values` must hold the samples that come between two calls to [`Self::get_values`] (pl1000pg-8, section 3.1).
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn run(&self, handle: i16, num_values: u32, stream: bool) -> PicoResult<()> {
        let method: BLOCK_METHOD = if stream {
            _BLOCK_METHOD_BM_STREAM
        } else {
            _BLOCK_METHOD_BM_SINGLE
        };

        let status = unsafe { self.bindings.pl1000Run(handle, num_values, method) };
        PicoStatus::from(status).to_result((), "run")
    }

    /// Reads the samples since the last call
    ///
    /// - Returns the raw samples.
    ///   Each sample set has one value for each channel, as `[ch0_t0, ch1_t0, ch0_t1, ch1_t1, ...]`.
    /// - Also returns an over-range bit field by channel number.
    ///   Bit 0 is channel 1, and a bit is set when its channel is over range in one or more of the samples (pl1000pg-8, section 4.5).
    /// - Returns no values when the driver has no new samples.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_values(
        &self,
        handle: i16,
        num_channels: usize,
        max_sample_sets: u32,
    ) -> PicoResult<(Vec<u16>, u16)> {
        if num_channels == 0 {
            return Ok((Vec::new(), 0));
        }

        let mut values = vec![0u16; num_channels * max_sample_sets as usize];
        let mut num_values = max_sample_sets;
        let mut overflow: u16 = 0;
        let mut trigger_index: u32 = 0;

        let status = unsafe {
            self.bindings.pl1000GetValues(
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
        let status = unsafe { self.bindings.pl1000Stop(handle) };
        PicoStatus::from(status).to_result((), "stop")
    }

    /// Closes the unit
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.pl1000CloseUnit(handle) };
        PicoStatus::from(status).to_result((), "close_unit")
    }
}

/// Samples per channel for a block of `interval_us` sets: `ideal_samples`, or fewer where the
/// block time would not fit in a `u32`
fn block_samples(interval_us: u32, ideal_samples: u32) -> u32 {
    ideal_samples.min(u32::MAX / interval_us.max(1)).max(1)
}

/// The interval between sample sets from the time `pl1000SetInterval` gives for a block,
/// rounded to the nearest microsecond
fn interval_from_block(us_for_block: u32, ideal_samples: u32) -> u32 {
    ((us_for_block as u64 + ideal_samples as u64 / 2) / ideal_samples as u64) as u32
}

/// Each function that the library must have to load, and whether it has it
///
/// - Leaves out the functions of the outputs, as `set_output` checks them first.
fn required(b: &PL1000Loader) -> Vec<(&'static str, bool)> {
    crate::required_functions!(
        b,
        [
            pl1000OpenUnit,
            pl1000CloseUnit,
            pl1000GetUnitInfo,
            pl1000Run,
            pl1000Stop,
            pl1000MaxValue,
            pl1000SetInterval,
            pl1000GetValues,
            pl1000PingUnit,
        ]
    )
}

/// A loaded PicoLog 1000 Series driver
///
/// - It dereferences to [`PL1000DriverInternal`], which has the methods.
/// - Clones share the loaded library.
#[derive(Clone)]
pub struct PL1000Driver(Arc<PL1000DriverInternal>);

impl PL1000Driver {
    /// Loads the library file at `path`
    ///
    /// - The library stays loaded until the process ends.
    /// - Returns Err when the library does not load, or lacks a function that this wrapper calls.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DriverLoadError> {
        let bindings = crate::Kept::new(unsafe { PL1000Loader::new(path.as_ref())? });
        crate::require_functions(&required(&bindings))?;
        Ok(PL1000Driver(Arc::new(PL1000DriverInternal { bindings })))
    }

    /// Loads the library that `resolution` finds
    ///
    /// - Returns Err when the library does not load, or lacks a function that this wrapper calls.
    pub fn load(resolution: &LibraryResolution) -> Result<Self, DriverLoadError> {
        PL1000Driver::new(resolution.get_path(Driver::PL1000))
    }
}

impl std::fmt::Debug for PL1000Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PL1000Driver").finish()
    }
}

impl Deref for PL1000Driver {
    type Target = PL1000DriverInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_block_time_fits_in_a_u32() {
        assert_eq!(block_samples(2_000, 1_000), 1_000);
        assert_eq!(block_samples(10_000_000, 1_000), 429);
        assert_eq!(block_samples(u32::MAX, 1_000), 1);
        assert_eq!(block_samples(0, 1_000), 1_000);
    }

    #[test]
    fn the_block_time_divides_into_the_interval() {
        assert_eq!(interval_from_block(2_000_000, 1_000), 2_000);
        assert_eq!(interval_from_block(10_499, 1_000), 10);
        assert_eq!(interval_from_block(10_500, 1_000), 11);
    }
}
