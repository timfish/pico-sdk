//! Safe wrapper for the USB DrDAQ educational data logger driver.
//!
//! Like the PL1000, the DrDAQ driver keeps a streaming buffer: `UsbDrDaqRun` with `BM_STREAM`
//! starts a continuous capture, and each `UsbDrDaqGetValues`/`UsbDrDaqGetValuesF` call returns
//! the readings taken since the last call (USB DrDAQ Programmer's Guide, usbdrdaqpg.en r9,
//! section 2.5 Capture modes). DrDAQ's channels are fixed onboard sensors rather than generic
//! analog inputs, and the driver applies each sensor's scaling, but the values it returns may
//! still need dividing by the channel's divider from `UsbDrDaqGetChannelInfo` to move the decimal
//! point (sections 2.6, 3.3 and 3.12); [`DrDAQChannelInfo::scale`] does that.
//!
//! This wrapper only covers what continuous channel streaming needs (open/close/ping, unit info,
//! channel capability info, interval/run/stop, and reading the float value buffer), and the
//! outputs: GPIO, PWM, the signal generator and the RGB LED ([`outputs`],
//! [`DrDAQDriverInternal::set_output`]). Digital inputs and pulse counting are not wrapped.

mod outputs;

pub use outputs::{outputs, sig_gen_info};

use crate::LibraryResolution;
use pico_common::{
    DrDAQChannel, DrDAQChannelInfo, DrDAQInfo, Driver, FromPicoStr, PicoError, PicoInfo,
    PicoResult, PicoStatus,
};
use pico_sys_dynamic::drdaq::{
    DrDAQLoader, _BLOCK_METHOD_BM_SINGLE, _BLOCK_METHOD_BM_STREAM, BLOCK_METHOD,
};
use std::{fmt, iter, ops::Deref, path::Path, sync::Arc};

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

    /// Opens the next unit that is not already open
    ///
    /// Returns `PicoStatus::NOT_FOUND` once every connected unit has been opened.
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

    /// Pings the unit to check the connection is still alive
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.UsbDrDaqPingUnit(handle) };
        PicoStatus::from(status).to_result((), "ping_unit")
    }

    /// The range, decimal places and divider of the scaling a channel uses (section 3.3)
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

    /// Sets the interval between sample sets and the channels to collect, returning the interval
    /// the driver set, in microseconds
    ///
    /// `UsbDrDaqSetInterval` takes the time for a whole block of `ideal_samples` per channel and
    /// returns the time it set (usbdrdaqpg.en r9, section 3.21), so the interval is multiplied up
    /// on the way in and divided down on the way out. `ideal_samples` is used only for that timing
    /// calculation; it does not limit a stream. In `BM_STREAM` mode the shortest interval between
    /// two samples is 10 µs per channel.
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

    /// Starts collecting samples into the driver's buffer
    ///
    /// `stream = true` runs continuously (`BM_STREAM`); `false` collects a single block
    /// (`BM_SINGLE`) of `num_values` samples per channel. In a stream, `num_values` must hold
    /// what arrives between two calls to [`Self::get_values`], and the total over all channels is
    /// at most 1,000,010 samples (sections 2.5 and 3.19).
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

    /// Reads the samples taken since the last call
    ///
    /// Returns the values from `UsbDrDaqGetValuesF`, before the channel's divider, interleaved
    /// channel-major within each sample set (i.e. `[ch0_t0, ch1_t0, ch0_t1, ch1_t1, ...]`) in
    /// channel number order whatever order the channels were set in, and an over-range bit field
    /// by channel number: the least significant bit is channel 1, set when that channel is over
    /// range in at least one returned sample (sections 3.11 and 3.12).
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

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.UsbDrDaqStop(handle) };
        PicoStatus::from(status).to_result((), "stop")
    }

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

/// A loaded USB DrDAQ driver
#[derive(Clone)]
pub struct DrDAQDriver(Arc<DrDAQDriverInternal>);

impl DrDAQDriver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(DrDAQDriver(Arc::new(DrDAQDriverInternal {
            bindings: crate::Kept::new(unsafe { DrDAQLoader::new(path.as_ref())? }),
        })))
    }

    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
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
