//! Safe wrapper for the ADC-20/ADC-24 high-resolution data logger driver (`Driver::PicoHRDL`).
//!
//! Like the PL1000/DrDAQ, the HRDL has a native streaming buffer: confirmed from the PicoLog 6
//! reference (`driver-hrdl/src/driver.ts`), `HRDLRun` starts the device collecting continuously
//! into its own buffer, and `HRDLGetValues` drains it.
//!
//! HRDL's error model is different from every other driver in this crate, though: most calls
//! return a bare `i16` success flag (0 = fail, non-zero = ok) instead of a `PICO_STATUS`, and the
//! reason for a failure has to be fetched separately by reading the `HRDL_SETTINGS` info field
//! back as a decimal string and parsing it into a [`HRDLError`]. [`HRDLDriverInternal::wrap_with_get_last_error`]
//! centralizes that, the same role [`crate::tc08::TC08DriverInternal::wrap_with_get_last_error`]
//! plays for the TC-08 (different transport, same idea).
//!
//! One quirk carried over from the PicoLog 6 reference and worth flagging as uncertain: a call
//! can report failure (return 0) while `HRDL_SETTINGS` reports `SE_OK`. The reference treats that
//! combination as success rather than an error (`throwLastError` only throws when the settings
//! error is not `SE_OK`), so this wrapper does the same - but the underlying reason for that
//! combination is not documented anywhere accessible, so it is reproduced rather than explained.

use crate::LibraryResolution;
use pico_common::{
    Driver, HRDLChannel, HRDLConversionTime, HRDLError, HRDLInfo, HRDLRange, MainsRejectionFreq,
    PicoError, PicoInfo, PicoResult, PicoStatus,
};
use pico_sys_dynamic::picohrdl::{
    HRDLLoader, enBlockMethod_HRDL_BM_BLOCK, enBlockMethod_HRDL_BM_STREAM,
    enHRDLInfo_HRDL_SETTINGS, HRDL_BLOCK_METHOD,
};
use std::{cmp::Ordering, fmt, ops::Deref, path::Path, sync::Arc};

pub struct HRDLDriverInternal {
    bindings: HRDLLoader,
}

impl HRDLDriverInternal {
    /// Reads a string info field from the device, if the call succeeds.
    ///
    /// Unlike the other loggers, `HRDLGetUnitInfo` has no required-size out-param - it returns
    /// the string length (or 0 on failure) directly.
    fn get_info_string_raw(&self, handle: i16, info_type: i16) -> Option<String> {
        let mut buffer = vec![0i8; 256];

        let len = unsafe {
            self.bindings
                .HRDLGetUnitInfo(handle, buffer.as_mut_ptr(), buffer.len() as i16, info_type)
        };

        if len <= 0 {
            return None;
        }

        let bytes: Vec<u8> = buffer[..len as usize].iter().map(|&b| b as u8).collect();
        Some(String::from_utf8_lossy(&bytes).trim_matches(char::from(0)).to_string())
    }

    /// Reads the last configuration error reported for the unit
    ///
    /// If the settings field itself cannot be read, there is no more specific error available, so
    /// this falls back to `COMMUNICATION_FAILED` rather than panicking.
    fn get_last_error(&self, handle: i16) -> HRDLError {
        self.get_info_string_raw(handle, enHRDLInfo_HRDL_SETTINGS as i16)
            .and_then(|text| text.trim().parse::<i32>().ok())
            .and_then(HRDLError::from_code)
            .unwrap_or(HRDLError::COMMUNICATION_FAILED)
    }

    /// Runs a driver call that reports success as a non-zero return, converting a failure into
    /// the reason reported via the `HRDL_SETTINGS` info field.
    ///
    /// Reproduces a quirk from the PicoLog 6 reference: if the settings error reads back as
    /// `SE_OK` despite the call reporting failure, that is treated as success too.
    fn wrap_with_get_last_error<F: FnOnce() -> i16>(&self, handle: i16, func: F) -> PicoStatus {
        if func() != 0 {
            return PicoStatus::OK;
        }

        self.get_last_error(handle).to_status()
    }

    /// Opens the next unit that is not already open
    ///
    /// `HRDLOpenUnit` takes no parameters and returns a handle directly: 0 when no unit is found,
    /// a positive handle on success. A negative return is not documented for the plain (non-async)
    /// open call in the published headers; it is treated here as "open in progress", matching the
    /// meaning `HRDLOpenUnitProgress` gives a negative progress value - flagged as best-effort
    /// rather than a confirmed behavior.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let result = unsafe { self.bindings.HRDLOpenUnit() };

        match result.cmp(&0) {
            Ordering::Greater => Ok(result),
            Ordering::Equal => Err(PicoStatus::NOT_FOUND),
            Ordering::Less => Err(PicoStatus::OPEN_OPERATION_IN_PROGRESS),
        }
    }

    /// Iterates every connected unit, opening each one
    ///
    /// The caller owns every handle this yields and must close the ones it does not keep.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        std::iter::from_fn(|| match self.open_next_unit() {
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
                    Some(serial) => match self.get_unit_info(handle) {
                        Ok(info) if info.serial == serial => break Ok(handle),
                        Ok(_) => handles_to_close.push(handle),
                        Err(error) => {
                            handles_to_close.push(handle);
                            break Err(error);
                        }
                    },
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
    pub fn get_unit_info(&self, handle: i16) -> PicoResult<HRDLInfo> {
        let serial = self
            .get_info_string_raw(handle, i16::from(PicoInfo::BATCH_AND_SERIAL))
            .ok_or_else(|| PicoError::from_status(self.get_last_error(handle).to_status(), "get_unit_info"))?;

        Ok(HRDLInfo {
            handle: Arc::new(handle),
            serial,
            hardware_version: self
                .get_info_string_raw(handle, i16::from(PicoInfo::HARDWARE_VERSION))
                .unwrap_or_default(),
            variant: self
                .get_info_string_raw(handle, i16::from(PicoInfo::VARIANT_INFO))
                .unwrap_or_default(),
        })
    }

    /// Pings the unit to check the connection is still alive
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe { self.bindings.HRDLAcknowledge(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_mains_rejection(&self, handle: i16, freq: MainsRejectionFreq) -> PicoResult<()> {
        let sixty_hertz: i16 = match freq {
            MainsRejectionFreq::_50Hz => 0,
            MainsRejectionFreq::_60Hz => 1,
        };

        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.HRDLSetMains(handle, sixty_hertz)
        })
        .to_result((), "set_mains_rejection")
    }

    /// Enables (or disables) a channel with a voltage range
    ///
    /// `single_ended = false` uses differential measurement between adjacent channel pairs.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_analog_channel(
        &self,
        handle: i16,
        channel: HRDLChannel,
        enabled: bool,
        range: HRDLRange,
        single_ended: bool,
    ) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.HRDLSetAnalogInChannel(
                handle,
                i16::from(channel),
                enabled as i16,
                i16::from(range),
                single_ended as i16,
            )
        })
        .to_result((), "set_analog_channel")
    }

    /// Disables a channel
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn disable_channel(&self, handle: i16, channel: HRDLChannel) -> PicoResult<()> {
        self.set_analog_channel(handle, channel, false, HRDLRange::default(), true)
    }

    /// Sets the sample interval (in milliseconds) and the conversion time used for every enabled
    /// channel
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_interval(
        &self,
        handle: i16,
        sample_interval_ms: i32,
        conversion_time: HRDLConversionTime,
    ) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings
                .HRDLSetInterval(handle, sample_interval_ms, i16::from(conversion_time))
        })
        .to_result((), "set_interval")
    }

    /// The full-scale ADC counts for a channel's currently configured range
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn min_max_adc_counts(&self, handle: i16, channel: HRDLChannel) -> PicoResult<(i32, i32)> {
        let mut min: i32 = 0;
        let mut max: i32 = 0;

        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.HRDLGetMinMaxAdcCounts(
                handle,
                &mut min as *mut i32,
                &mut max as *mut i32,
                i16::from(channel),
            )
        })
        .to_result((min, max), "min_max_adc_counts")
    }

    /// Starts collecting samples into the device's internal buffer
    ///
    /// `stream = true` runs continuously (`HRDL_BM_STREAM`); `false` collects a single block
    /// (`HRDL_BM_BLOCK`) of `num_values` samples per channel.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn run(&self, handle: i16, num_values: i32, stream: bool) -> PicoResult<()> {
        let method: HRDL_BLOCK_METHOD = if stream {
            enBlockMethod_HRDL_BM_STREAM
        } else {
            enBlockMethod_HRDL_BM_BLOCK
        };

        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.HRDLRun(handle, num_values, method as i16)
        })
        .to_result((), "run")
    }

    /// Whether the device has finished collecting the block started by [`Self::run`]
    ///
    /// Only meaningful for single-block collection; streaming mode is drained continuously with
    /// [`Self::get_values`] instead.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn ready(&self, handle: i16) -> bool {
        unsafe { self.bindings.HRDLReady(handle) > 0 }
    }

    /// Drains whatever samples have landed in the device's buffer since the last call
    ///
    /// Returns the raw interleaved ADC counts (channel-major within each sample set, i.e.
    /// `[ch0_t0, ch1_t0, ch0_t1, ch1_t1, ...]`) and a per-channel over-range bitmask (bit `n` set
    /// means the `n`th requested channel is over-range in at least one returned sample).
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_values(
        &self,
        handle: i16,
        num_channels: usize,
        max_sample_sets: i32,
    ) -> PicoResult<(Vec<i32>, i16)> {
        if num_channels == 0 {
            return Ok((Vec::new(), 0));
        }

        let mut values = vec![0i32; num_channels * max_sample_sets as usize];
        let mut overflow: i16 = 0;

        let sample_sets = unsafe {
            self.bindings.HRDLGetValues(
                handle,
                values.as_mut_ptr(),
                &mut overflow as *mut i16,
                max_sample_sets,
            )
        };

        if sample_sets < 0 {
            return Err(PicoError::from_status(
                self.get_last_error(handle).to_status(),
                "get_values",
            ));
        }

        values.truncate(num_channels * sample_sets as usize);
        Ok((values, overflow))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) {
        unsafe { self.bindings.HRDLStop(handle) };
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe { self.bindings.HRDLCloseUnit(handle) })
            .to_result((), "close_unit")
    }
}

/// A loaded ADC-20/ADC-24 (PicoHRDL) driver
#[derive(Clone)]
pub struct HRDLDriver(Arc<HRDLDriverInternal>);

impl HRDLDriver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(HRDLDriver(Arc::new(HRDLDriverInternal {
            bindings: unsafe { HRDLLoader::new(path.as_ref())? },
        })))
    }

    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
        HRDLDriver::new(resolution.get_path(Driver::PicoHRDL))
    }
}

impl fmt::Debug for HRDLDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HRDLDriver").finish()
    }
}

impl Deref for HRDLDriver {
    type Target = HRDLDriverInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
