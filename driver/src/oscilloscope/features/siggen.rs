use pico_common::{PicoError, PicoResult, SigGenConfig, SigGenInfo};

/// The signal generator, through the legacy API or the properties API of the driver
///
/// - A change does not stop a stream.
/// - While the scope streams, the ps2000a and psospa drivers return `BUSY` for most changes (ps2000a guide §3.52).
///   A change of frequency only, or off on psospa, goes through.
/// - Each call returns Err with `NOT_USED` when the library lacks a function that it needs.
pub trait SigGenDriver {
    /// The shapes and limits of the signal generator of the unit
    fn sig_gen_info(&self, _handle: i16) -> PicoResult<SigGenInfo> {
        Err(PicoError::not_supported("sig_gen_info"))
    }
    /// Sets the output, or turns it off with `None`
    ///
    /// - Returns the values that the driver set.
    ///   The properties API gives the frequencies it set, and the legacy API gives `None`.
    fn sig_gen_apply(
        &self,
        _handle: i16,
        _config: Option<&SigGenConfig>,
    ) -> PicoResult<Option<SigGenConfig>> {
        Err(PicoError::not_supported("sig_gen_apply"))
    }
    /// Fires a software trigger
    ///
    /// - With a gate trigger, `true` runs the output and `false` holds it.
    fn sig_gen_software_trigger(&self, _handle: i16, _state: bool) -> PicoResult<()> {
        Err(PicoError::not_supported("sig_gen_software_trigger"))
    }
    /// Holds the output at its present voltage
    ///
    /// - Only the properties API has this call.
    fn sig_gen_pause(&self, _handle: i16) -> PicoResult<()> {
        Err(PicoError::not_supported("sig_gen_pause"))
    }
    /// Starts the output again after [`SigGenDriver::sig_gen_pause`]
    fn sig_gen_restart(&self, _handle: i16) -> PicoResult<()> {
        Err(PicoError::not_supported("sig_gen_restart"))
    }
}
