//! The ADC-20 and ADC-24 high-resolution data logger driver (`Driver::PicoHRDL`)
//!
//! - The driver keeps a stream buffer.
//!   "The driver constantly polls the device, and samples are placed in a buffer until retrieved by your application" (ADC-20/ADC-24 Programmer's Guide, adc20.en r2, section 4.1 Recording methods).
//!   `HRDLRun` with `BM_STREAM` starts it, and each `HRDLGetValues` call returns the values since the last call (sections 5.15 and 6.1.2).
//! - Most driver calls return 1 for success and 0 for failure, not a `PICO_STATUS`.
//!   The cause of a failure is a decimal string in the `HRDL_SETTINGS` information field (sections 5.9, 5.16 and 5.18).
//!   The wrapper reads it as a [`HRDLError`], so each method returns a normal `PicoResult`.

mod outputs;

pub use outputs::outputs;

use crate::LibraryResolution;
use parking_lot::Mutex;
use pico_common::{
    Driver, FromPicoStr, HRDLChannel, HRDLConversionTime, HRDLError, HRDLInfo, HRDLRange,
    MainsRejectionFreq, PicoError, PicoInfo, PicoResult, PicoStatus,
};
use pico_sys_dynamic::picohrdl::{
    enBlockMethod_HRDL_BM_BLOCK, enBlockMethod_HRDL_BM_STREAM, enHRDLInfo_HRDL_ERROR,
    enHRDLInfo_HRDL_SETTINGS, HRDLLoader, HRDL_BLOCK_METHOD,
};
use std::{cmp::Ordering, collections::HashMap, fmt, ops::Deref, path::Path, sync::Arc};

/// The methods of the ADC-20 and ADC-24 driver
///
/// - Get it through [`HRDLDriver`], which dereferences to it.
/// - Each method that takes a `handle` needs a unit that [`HRDLDriverInternal::open_unit`] opened.
pub struct HRDLDriverInternal {
    bindings: crate::Kept<HRDLLoader>,
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

        Some(buffer.from_pico_i8_string(len as usize))
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

    /// Opens the next unit that is not open, and returns its handle
    ///
    /// - `HRDLOpenUnit` returns a handle, 0 when it finds no unit, or -1 when a unit does not open (section 5.11).
    /// - The driver takes up to 20 units.
    /// - Returns Err with `PicoStatus::NOT_FOUND` when all connected units are open.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let result = unsafe { self.bindings.HRDLOpenUnit() };

        match result.cmp(&0) {
            Ordering::Greater => Ok(result),
            Ordering::Equal => Err(PicoStatus::NOT_FOUND),
            Ordering::Less => Err(self.get_open_error()),
        }
    }

    /// Opens each connected unit that is not open, and gives its handle
    ///
    /// - You own each handle, so close the units that you do not keep.
    /// - It stops after the first error, as a unit that does not open would fail again.
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

    /// Opens the unit with `serial`, or the next unit with `None`, and returns its handle
    ///
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

    /// The serial, hardware version and variant of the unit
    ///
    /// - The version and the variant are empty when the driver does not give them.
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

    /// Checks that the unit is still connected
    ///
    /// - Do not call it while the unit collects samples, after [`Self::run`] (section 5.1).
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe { self.bindings.HRDLAcknowledge(handle) })
            .to_result((), "ping_unit")
    }

    /// Sets the mains frequency that the unit filters out
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

    /// Turns a channel on or off, with a voltage range
    ///
    /// - With `single_ended` set to `false`, the unit measures the difference between the two channels of a pair.
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

    /// Turns a channel off
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn disable_channel(&self, handle: i16, channel: HRDLChannel) -> PicoResult<()> {
        self.set_analog_channel(handle, channel, false, HRDLRange::default(), true)
    }

    /// Sets the sample interval in ms, and the conversion time of each channel that is on
    ///
    /// - The interval must be more than the conversion time multiplied by the number of analog channels that are on, as the unit converts one channel at a time (section 5.18).
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

    /// The lowest and highest ADC counts of the range that the channel has now
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

    /// Starts to collect samples into the driver buffer
    ///
    /// - With `stream` set to `true` it runs continuously (`HRDL_BM_STREAM`).
    ///   With `false` it collects one block (`HRDL_BM_BLOCK`) of `num_values` samples for each channel.
    /// - In a stream, `num_values` must hold the samples that come between two calls to [`Self::get_values`] (section 5.15).
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

    /// Tells if the unit has collected the block that [`Self::run`] started
    ///
    /// - Use it only for one block.
    ///   In a stream, call [`Self::get_values`] again and again.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn ready(&self, handle: i16) -> bool {
        unsafe { self.bindings.HRDLReady(handle) > 0 }
    }

    /// Reads the samples since the last call
    ///
    /// - Returns the raw ADC counts.
    ///   Each sample set has one value for each channel, in channel number order, as `[ch0_t0, ch1_t0, ch0_t1, ch1_t1, ...]`.
    /// - Also returns an over-range bit field by channel number.
    ///   Bit 0 is channel 1, and a bit is set when its channel is over range in one or more of the samples (section 5.10).
    /// - `HRDLGetValues` returns 0 when it fails and when it has no new values, so the wrapper reads 0 as no values.
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

    /// Stops the capture
    ///
    /// - After a stream, the unit needs this call before the next capture (section 5.20).
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) {
        unsafe { self.bindings.HRDLStop(handle) };
    }

    /// Closes the unit
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

/// A loaded ADC-20 and ADC-24 (PicoHRDL) driver
///
/// - It dereferences to [`HRDLDriverInternal`], which has the methods.
/// - Clones share the loaded library.
#[derive(Clone)]
pub struct HRDLDriver(Arc<HRDLDriverInternal>);

impl HRDLDriver {
    /// Loads the library file at `path`
    ///
    /// - The library stays loaded until the process ends.
    /// - Returns Err when the library does not load.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(HRDLDriver(Arc::new(HRDLDriverInternal {
            bindings: crate::Kept::new(unsafe { HRDLLoader::new(path.as_ref())? }),
            digital_io: Mutex::new(HashMap::new()),
        })))
    }

    /// Loads the library that `resolution` finds
    ///
    /// - Returns Err when the library does not load.
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
