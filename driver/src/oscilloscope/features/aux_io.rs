use pico_common::{AuxIoMode, PicoError, PicoResult};

/// The AUX I/O connector as an input, a logic level or trigger out.
pub trait AuxIoDriver {
    fn set_aux_io_mode(&self, _handle: i16, _mode: AuxIoMode) -> PicoResult<()> {
        Err(PicoError::not_supported("set_aux_io_mode"))
    }
}
