//! Safe wrapper for the USB DrDAQ educational data logger driver.
//!
//! Like the PL1000, the DrDAQ has a native streaming buffer: confirmed from the PicoLog 6
//! reference (`driver-dr-daq/src/driver.ts`), `run` starts the device collecting continuously
//! into its own buffer, and `getValues`/`getValuesF` drains it. DrDAQ's channels are fixed
//! onboard sensors rather than generic analog inputs, and the driver returns already-scaled
//! engineering-unit floats (`UsbDrDaqGetValuesF`) rather than raw ADC counts, so there is no
//! separate max-value/scaling step like the PL1000.
//!
//! This wrapper only covers what continuous channel streaming needs (open/close/ping, unit info,
//! channel capability info, interval/run/stop, and reading the float value buffer). DrDAQ also
//! has an onboard signal generator, RGB LED, digital I/O and pulse counting, none of which are in
//! scope for a waveform capture SDK - they are left unwrapped.

use crate::LibraryResolution;
use pico_common::{
    Driver, DrDAQChannel, DrDAQChannelInfo, DrDAQInfo, FromPicoStr, PicoError, PicoInfo,
    PicoResult, PicoStatus,
};
use pico_sys_dynamic::drdaq::{DrDAQLoader, BLOCK_METHOD, _BLOCK_METHOD_BM_SINGLE, _BLOCK_METHOD_BM_STREAM};
use std::{fmt, iter, ops::Deref, path::Path, sync::Arc};

pub struct DrDAQDriverInternal {
    bindings: DrDAQLoader,
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

    /// The valid engineering-unit range and display resolution for a channel
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
            },
            "channel_info",
        )
    }

    /// Sets the sample interval and the channels to collect, returning the interval the driver
    /// actually settled on, in microseconds
    ///
    /// `ideal_samples` is a sizing hint for the block the driver prepares internally, not a
    /// sample count limit - streaming keeps running past it.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_interval(
        &self,
        handle: i16,
        channels: &[DrDAQChannel],
        interval_us: u32,
        ideal_samples: u32,
    ) -> PicoResult<u32> {
        let mut interval_us = interval_us;
        let mut channel_ids: Vec<u32> = channels.iter().map(|&c| u32::from(c)).collect();

        let status = unsafe {
            self.bindings.UsbDrDaqSetInterval(
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

        let status = unsafe { self.bindings.UsbDrDaqRun(handle, num_values, method) };
        PicoStatus::from(status).to_result((), "run")
    }

    /// Drains whatever samples have landed in the device's buffer since the last call
    ///
    /// Returns already-scaled engineering-unit values, interleaved channel-major within each
    /// sample set (i.e. `[ch0_t0, ch1_t0, ch0_t1, ch1_t1, ...]`), and a per-channel over-range
    /// bitmask (bit `n` set means the `n`th requested channel is over-range in at least one
    /// returned sample).
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

/// A loaded USB DrDAQ driver
#[derive(Clone)]
pub struct DrDAQDriver(Arc<DrDAQDriverInternal>);

impl DrDAQDriver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(DrDAQDriver(Arc::new(DrDAQDriverInternal {
            bindings: unsafe { DrDAQLoader::new(path.as_ref())? },
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
