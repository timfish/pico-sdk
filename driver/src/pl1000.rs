//! Safe wrapper for the PicoLog 1000 series data logger driver.
//!
//! Unlike the TC-08/PT-104 loggers, the PL1000 has a streaming buffer in its driver. The
//! PicoLog 1000 Series Programmer's Guide (pl1000pg-8, section 3.1 Capture modes) says that in
//! `BM_STREAM` mode "every time pl1000GetValues() is called, it returns the new readings since
//! the last time it was called", after `pl1000Run` starts the capture (section 4.12). The data is
//! polled from user code, not delivered by callback.
//!
//! Two quirks worth knowing about:
//!
//! - There is no enumerate call. Each `pl1000OpenUnit` call opens one unit (section 4.7; the
//!   driver supports up to 64 units, section 2.1), so enumeration means opening every unit and
//!   closing the ones that were not wanted.
//! - Every channel shares one input range; there is no per-channel range setting like the
//!   oscilloscopes. `pl1000MaxValue` reports the full-scale ADC reading for the connected variant
//!   (sections 3.2 and 4.6).

mod outputs;

pub use outputs::outputs;

use crate::LibraryResolution;
use pico_common::{
    Driver, FromPicoStr, PL1000Channel, PL1000Info, PicoError, PicoInfo, PicoResult, PicoStatus,
};
use pico_sys_dynamic::pl1000::{
    PL1000Loader, _BLOCK_METHOD_BM_SINGLE, _BLOCK_METHOD_BM_STREAM, BLOCK_METHOD,
};
use std::{iter, ops::Deref, path::Path, sync::Arc};

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

    /// Opens the next unit that is not already open
    ///
    /// Returns `PicoStatus::NOT_FOUND` once every connected unit has been opened.
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

    /// Iterates every connected unit, opening each one
    ///
    /// The caller owns every handle this yields and must close the ones it does not keep. The
    /// iteration ends after the first error, as an open that fails would fail again.
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

    /// Opens a unit, optionally with a specific serial number
    ///
    /// Because units can only be found by opening them, looking for a particular serial means
    /// opening others along the way. Those are closed again before returning.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        let mut handles_to_close = Vec::new();

        let result = loop {
            match self.open_next_unit() {
                Ok(handle) => match serial {
                    None => break Ok(handle),
                    Some(serial) => {
                        match self.get_info_string(handle, u32::from(PicoInfo::BATCH_AND_SERIAL)) {
                            Ok(found) if found.contains(serial) => break Ok(handle),
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

    /// The full-scale ADC reading for the connected unit's fixed input range
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn max_value(&self, handle: i16) -> PicoResult<u16> {
        let mut value: u16 = 0;

        let status = unsafe { self.bindings.pl1000MaxValue(handle, &mut value as *mut u16) };

        PicoStatus::from(status).to_result(value, "max_value")
    }

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

    /// Pings the unit to check the connection is still alive
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.pl1000PingUnit(handle) };
        PicoStatus::from(status).to_result((), "ping_unit")
    }

    /// Sets the interval between sample sets and the channels to collect, returning the interval
    /// the driver set, in microseconds
    ///
    /// `pl1000SetInterval` takes the time for a whole block of `ideal_samples` per channel and
    /// returns the time it will take (pl1000pg-8, section 4.14), so the interval is multiplied up
    /// on the way in and divided down on the way out. `ideal_samples` is used only for that timing
    /// calculation; it does not limit a stream. The guide limits `ideal_samples` times the number
    /// of channels to 1,000,000.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_interval(
        &self,
        handle: i16,
        channels: &[PL1000Channel],
        interval_us: u32,
        ideal_samples: u32,
    ) -> PicoResult<u32> {
        let ideal_samples = ideal_samples.max(1);
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

    /// Starts collecting samples into the driver's buffer
    ///
    /// `stream = true` runs continuously (`BM_STREAM`); `false` collects a single block
    /// (`BM_SINGLE`) of `num_values` samples per channel. In a stream, `num_values` must hold
    /// what arrives between two calls to [`Self::get_values`] (pl1000pg-8, section 3.1).
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

    /// Reads the samples taken since the last call
    ///
    /// Returns the raw interleaved samples (channel-major within each sample set, i.e.
    /// `[ch0_t0, ch1_t0, ch0_t1, ch1_t1, ...]`) and an over-range bit field by channel number: the
    /// least significant bit is channel 1, set when that channel is over range in at least one
    /// returned sample (pl1000pg-8, section 4.5).
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

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.pl1000Stop(handle) };
        PicoStatus::from(status).to_result((), "stop")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.pl1000CloseUnit(handle) };
        PicoStatus::from(status).to_result((), "close_unit")
    }
}

/// The interval between sample sets from the time `pl1000SetInterval` gives for a block,
/// rounded to the nearest microsecond
fn interval_from_block(us_for_block: u32, ideal_samples: u32) -> u32 {
    ((us_for_block as u64 + ideal_samples as u64 / 2) / ideal_samples as u64) as u32
}

/// A loaded PicoLog 1000 series driver
#[derive(Clone)]
pub struct PL1000Driver(Arc<PL1000DriverInternal>);

impl PL1000Driver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(PL1000Driver(Arc::new(PL1000DriverInternal {
            bindings: crate::Kept::new(unsafe { PL1000Loader::new(path.as_ref())? }),
        })))
    }

    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
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
    fn the_block_time_divides_into_the_interval() {
        assert_eq!(interval_from_block(2_000_000, 1_000), 2_000);
        assert_eq!(interval_from_block(10_499, 1_000), 10);
        assert_eq!(interval_from_block(10_500, 1_000), 11);
    }
}
