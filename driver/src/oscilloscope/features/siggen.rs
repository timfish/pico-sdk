use pico_common::{PicoError, PicoResult, SigGenConfig, SigGenInfo};

/// The signal generator, through the legacy or the properties API.
/// It is an output: a change does not stop a stream.
pub trait SigGenDriver {
    fn sig_gen_info(&self, _handle: i16) -> PicoResult<SigGenInfo> {
        Err(PicoError::not_supported("sig_gen_info"))
    }
    /// Sets the output, or turns it off with `None`. Returns the values
    /// the driver set: the properties API reports the frequencies it
    /// took, the legacy API reports none.
    fn sig_gen_apply(
        &self,
        _handle: i16,
        _config: Option<&SigGenConfig>,
    ) -> PicoResult<Option<SigGenConfig>> {
        Err(PicoError::not_supported("sig_gen_apply"))
    }
    /// Fires a software trigger. With a gate trigger, `true` runs the
    /// output and `false` holds it.
    fn sig_gen_software_trigger(&self, _handle: i16, _state: bool) -> PicoResult<()> {
        Err(PicoError::not_supported("sig_gen_software_trigger"))
    }
    /// Holds the output at its present voltage (properties API only).
    fn sig_gen_pause(&self, _handle: i16) -> PicoResult<()> {
        Err(PicoError::not_supported("sig_gen_pause"))
    }
    /// Goes on after [`SigGenDriver::sig_gen_pause`].
    fn sig_gen_restart(&self, _handle: i16) -> PicoResult<()> {
        Err(PicoError::not_supported("sig_gen_restart"))
    }
}
