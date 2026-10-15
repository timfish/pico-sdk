//! Safe wrapper for the ADC-20/ADC-24 high-resolution data logger driver (`Driver::PicoHRDL`).
//!
//! Like the PL1000/DrDAQ, the HRDL driver keeps a streaming buffer: "the driver constantly polls
//! the device, and samples are placed in a buffer until retrieved by your application"
//! (ADC-20/ADC-24 Programmer's Guide, adc20.en r2, section 4.1 Recording methods). `HRDLRun` with
//! `BM_STREAM` starts it, and each `HRDLGetValues` call returns the readings since the last call
//! (sections 5.15 and 6.1.2).
//!
//! HRDL's error model is different from every other driver in this crate, though: most calls
//! return 1 on success and 0 on failure instead of a `PICO_STATUS`, and the reason for a failure
//! is read separately from the `HRDL_SETTINGS` info field as a decimal string (sections 5.9,
//! 5.16 and 5.18), parsed into a [`HRDLError`]. [`HRDLDriverInternal::wrap_with_get_last_error`]
//! centralizes that, the same role [`crate::tc08::TC08DriverInternal::wrap_with_get_last_error`]
//! plays for the TC-08 (different transport, same idea).

mod outputs;

pub use outputs::outputs;

use crate::LibraryResolution;
use parking_lot::Mutex;
use pico_common::{
    Driver, HRDLChannel, HRDLConversionTime, HRDLError, HRDLInfo, HRDLRange, MainsRejectionFreq,
    PicoError, PicoInfo, PicoResult, PicoStatus,
};
use pico_sys_dynamic::picohrdl::{
    enBlockMethod_HRDL_BM_BLOCK, enBlockMethod_HRDL_BM_STREAM, enHRDLInfo_HRDL_ERROR,
    enHRDLInfo_HRDL_SETTINGS, HRDLLoader, HRDL_BLOCK_METHOD,
};
use std::{cmp::Ordering, collections::HashMap, fmt, ops::Deref, path::Path, sync::Arc};

pub struct HRDLDriverInternal {
    bindings: HRDLLoader,
    /// The digital I/O pins of each open unit, as last set.
    digital_io: Mutex<HashMap<i16, outputs::DigitalIo>>,
}

impl HRDLDriverInternal {
    /// Reads a string info field from the device, if the call succeeds.
    ///
    /// Unlike the other loggers, `HRDLGetUnitInfo` has no required-size out-param - it returns
    /// the string length (or 0 on failure) directly.
    fn get_info_string_raw(&self, handle: i16, info_type: i16) -> Option<String> {
        let mut buffer = vec![0i8; 256];

        let len = unsafe {
            self.bindings.HRDLGetUnitInfo(
                handle,
                buffer.as_mut_ptr(),
                buffer.len() as i16,
                info_type,
            )
        };

        if len <= 0 {
            return None;
        }

        let bytes: Vec<u8> = buffer[..len as usize].iter().map(|&b| b as u8).collect();
        Some(
            String::from_utf8_lossy(&bytes)
                .trim_matches(char::from(0))
                .to_string(),
        )
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
    /// A 0 return is a failure (sections 5.16 and 5.18). If the settings field reads `SE_OK`
    /// even so, no settings error caused it, and the call reports `OPERATION_FAILED`.
    fn wrap_with_get_last_error<F: FnOnce() -> i16>(&self, handle: i16, func: F) -> PicoStatus {
        if func() != 0 {
            return PicoStatus::OK;
        }

        failed_status(self.get_last_error(handle))
    }

    /// Why the last `HRDLOpenUnit` failed: with a handle no unit has, `HRDLGetUnitInfo` gives
    /// the `HRDL_ERROR` code of the last unit that failed to open (section 5.9)
    fn get_open_error(&self) -> PicoStatus {
        self.get_info_string_raw(0, enHRDLInfo_HRDL_ERROR as i16)
            .and_then(|text| text.trim().parse::<i32>().ok())
            .map(open_error_status)
            .unwrap_or(PicoStatus::OPERATION_FAILED)
    }

    /// Opens the next unit that is not already open
    ///
    /// `HRDLOpenUnit` returns a handle, 0 when no unit is found, or -1 when a unit fails to open
    /// (section 5.11); the driver supports up to 20 units.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let result = unsafe { self.bindings.HRDLOpenUnit() };

        match result.cmp(&0) {
            Ordering::Greater => Ok(result),
            Ordering::Equal => Err(PicoStatus::NOT_FOUND),
            Ordering::Less => Err(self.get_open_error()),
        }
    }

    /// Iterates every connected unit, opening each one
    ///
    /// The caller owns every handle this yields and must close the ones it does not keep. The
    /// iteration ends after the first error, as a unit that fails to open would fail again.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        let mut done = false;
        std::iter::from_fn(move || {
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
            .ok_or_else(|| {
                PicoError::from_status(failed_status(self.get_last_error(handle)), "get_unit_info")
            })?;

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
    ///
    /// Not to be called while the unit is collecting, after [`Self::run`] (section 5.1).
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
    ///
    /// The interval must be more than the conversion time times the analog channels that are
    /// on, as conversion is sequential (section 5.18).
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

    /// Starts collecting samples into the driver's buffer
    ///
    /// `stream = true` runs continuously (`HRDL_BM_STREAM`); `false` collects a single block
    /// (`HRDL_BM_BLOCK`) of `num_values` samples per channel. In a stream, `num_values` must hold
    /// what arrives between two calls to [`Self::get_values`] (section 5.15).
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

    /// Reads the samples taken since the last call
    ///
    /// Returns the raw interleaved ADC counts (channel-major within each sample set, i.e.
    /// `[ch0_t0, ch1_t0, ch0_t1, ch1_t1, ...]`, in channel number order) and an over-range bit
    /// field by channel number: the least significant bit is channel 1, set when that channel is
    /// over range in at least one returned sample (section 5.10). `HRDLGetValues` returns 0 both
    /// when it fails and when there are no new values, so a 0 reads as no values.
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

        // The guide gives no negative return; read one as a failure.
        if sample_sets < 0 {
            return Err(PicoError::from_status(
                failed_status(self.get_last_error(handle)),
                "get_values",
            ));
        }

        values.truncate(num_channels * sample_sets as usize);
        Ok((values, overflow))
    }

    /// Ends a stream; needed in streaming mode for the unit to be ready for the next capture
    /// (section 5.20)
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) {
        unsafe { self.bindings.HRDLStop(handle) };
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        self.digital_io.lock().remove(&handle);
        self.wrap_with_get_last_error(handle, || unsafe { self.bindings.HRDLCloseUnit(handle) })
            .to_result((), "close_unit")
    }
}

/// The status for a call that returned 0, from the `HRDL_SETTINGS` code read after it
fn failed_status(error: HRDLError) -> PicoStatus {
    match error.to_status() {
        PicoStatus::OK => PicoStatus::OPERATION_FAILED,
        status => status,
    }
}

/// The status for an `HRDL_ERROR` code (section 5.9, Error codes)
fn open_error_status(code: i32) -> PicoStatus {
    match code {
        1 => PicoStatus::KERNEL_DRIVER_TOO_OLD,
        2 => PicoStatus::NOT_FOUND,
        3 => PicoStatus::FW_FAIL,
        4 => PicoStatus::OS_NOT_SUPPORTED,
        5 => PicoStatus::MAX_UNITS_OPENED,
        _ => PicoStatus::OPERATION_FAILED,
    }
}

/// A loaded ADC-20/ADC-24 (PicoHRDL) driver
#[derive(Clone)]
pub struct HRDLDriver(Arc<HRDLDriverInternal>);

impl HRDLDriver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(HRDLDriver(Arc::new(HRDLDriverInternal {
            bindings: unsafe { HRDLLoader::new(path.as_ref())? },
            digital_io: Mutex::new(HashMap::new()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failed_call_is_an_error_even_with_no_settings_error() {
        assert_eq!(failed_status(HRDLError::OK), PicoStatus::OPERATION_FAILED);
        assert_eq!(
            failed_status(HRDLError::INVALID_CHANNEL),
            PicoStatus::INVALID_CHANNEL
        );
    }

    #[test]
    fn open_errors_map_to_statuses() {
        assert_eq!(open_error_status(2), PicoStatus::NOT_FOUND);
        assert_eq!(open_error_status(5), PicoStatus::MAX_UNITS_OPENED);
        assert_eq!(open_error_status(0), PicoStatus::OPERATION_FAILED);
    }
}
