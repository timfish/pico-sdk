use parking_lot::RwLock;
use pico_common::{EtsConfig, PicoError, PicoResult};
use std::sync::Arc;

/// Equivalent-time sampling, in block mode.
pub trait EtsDriver {
    /// Turns ETS on, or off with `None`. Returns the interval between
    /// samples in picoseconds while on.
    fn set_ets(&self, _handle: i16, _ets: Option<&EtsConfig>) -> PicoResult<Option<f64>> {
        Err(PicoError::not_supported("set_ets"))
    }
    /// The buffer the driver writes each sample's time into, in
    /// femtoseconds.
    fn set_ets_time_buffer(&self, _handle: i16, _buffer: Arc<RwLock<Vec<i64>>>) -> PicoResult<()> {
        Err(PicoError::not_supported("set_ets_time_buffer"))
    }
}
