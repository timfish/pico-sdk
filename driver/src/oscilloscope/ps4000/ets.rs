use super::PS4000Driver;
use crate::oscilloscope::{
    features::{ets_args, ets_buffer_len},
    EtsDriver,
};
use parking_lot::RwLock;
use pico_common::{EtsConfig, PicoResult, PicoStatus};
use std::sync::Arc;

// PicoScope 4000 Series Programmer's Guide, ps4000SetEts and ps4000SetEtsTimeBuffer; 4226 and 4227 only.
impl EtsDriver for PS4000Driver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_ets(&self, handle: i16, ets: Option<&EtsConfig>) -> PicoResult<Option<f64>> {
        let (mode, cycles, interleave) = ets_args(ets)?;
        let mut picoseconds = 0i32;
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000SetEts(handle, mode, cycles, interleave, &mut picoseconds)
        })
        .to_result((), "set_ets")?;
        Ok(ets.map(|_| picoseconds as f64))
    }

    #[tracing::instrument(level = "debug", skip(self, buffer), err(level = "warn", Display))]
    fn set_ets_time_buffer(&self, handle: i16, buffer: Arc<RwLock<Vec<i64>>>) -> PicoResult<()> {
        let len: i32 = ets_buffer_len(&buffer)?;
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000SetEtsTimeBuffer(handle, buffer.write().as_mut_ptr(), len)
        })
        .to_result((), "set_ets_time_buffer")
    }
}
