//! Safe wrapper for the PicoLog 1000 series data logger driver.
//!
//! Unlike the TC-08/PT-104 loggers, the PL1000 has a native streaming buffer: `pl1000Run` starts
//! the device collecting into its own buffer (block or continuous stream), and `pl1000GetValues`
//! drains whatever has landed so far. This is the same shape as the oscilloscope drivers, just
//! polled from user code rather than delivered by callback - confirmed from the PicoLog 6
//! reference (`driver-pl-1000/src/driver.ts`), which always streams (`pollingRequired` there
//! tracks USB open-detection polling, not the data acquisition mode).
//!
//! Two quirks worth knowing about:
//!
//! - There is no enumerate call. `pl1000OpenUnit` opens the next unattached unit and returns a
//!   non-positive handle when there are none left, so enumeration means opening every unit and
//!   closing the ones that were not wanted.
//! - Every channel shares one input range; there is no per-channel range setting like the
//!   oscilloscopes. `pl1000MaxValue` reports the full-scale ADC reading for the connected variant.

use crate::LibraryResolution;
use pico_common::{
    Driver, FromPicoStr, PicoError, PicoInfo, PicoResult, PicoStatus, PL1000Channel, PL1000Info,
};
use pico_sys_dynamic::pl1000::{
    PL1000Loader, BLOCK_METHOD, _BLOCK_METHOD_BM_SINGLE, _BLOCK_METHOD_BM_STREAM,
};
use std::{iter, ops::Deref, path::Path, sync::Arc};

pub struct PL1000DriverInternal {
    bindings: PL1000Loader,
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
    /// The caller owns every handle this yields and must close the ones it does not keep.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        iter::from_fn(|| match self.open_next_unit() {
            Ok(handle) => Some(Ok(handle)),
            Err(PicoStatus::NOT_FOUND) => None,
            Err(status) => Some(Err(PicoError::from_status(status, "open_next_unit"))),
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

    /// Sets the sample interval and the channels to collect, returning the interval the driver
    /// actually settled on
    ///
    /// `ideal_samples` is a sizing hint for the block the driver prepares internally, not a
    /// sample count limit - streaming keeps running past it.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_interval(
        &self,
        handle: i16,
        channels: &[PL1000Channel],
        interval_us: u32,
        ideal_samples: u32,
    ) -> PicoResult<u32> {
        let mut interval_us = interval_us;
        let mut channel_ids: Vec<i16> = channels.iter().map(|&c| i16::from(c)).collect();

        let status = unsafe {
            self.bindings.pl1000SetInterval(
                handle,
                &mut interval_us as *mut u32,
                ideal_samples,
                channel_ids.as_mut_ptr(),
                channel_ids.len() as i16,
            )
        };

        PicoStatus::from(status).to_result(interval_us, "set_interval")
    }

    /// Starts collecting samples into the device's internal buffer
    ///
    /// `stream = true` runs continuously (`BM_STREAM`); `false` collects a single block
    /// (`BM_SINGLE`) of `num_values` samples per channel.
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

    /// Drains whatever samples have landed in the device's buffer since the last call
    ///
    /// Returns the raw interleaved samples (channel-major within each sample set, i.e.
    /// `[ch0_t0, ch1_t0, ch0_t1, ch1_t1, ...]`) and a per-channel over-range bitmask (bit `n` set
    /// means the `n`th requested channel is over-range in at least one returned sample).
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

/// A loaded PicoLog 1000 series driver
#[derive(Clone)]
pub struct PL1000Driver(Arc<PL1000DriverInternal>);

impl PL1000Driver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(PL1000Driver(Arc::new(PL1000DriverInternal {
            bindings: unsafe { PL1000Loader::new(path.as_ref())? },
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
