use super::PS5000ADriver;
use crate::oscilloscope::{
    features::{ets_args, ets_buffer_len},
    EtsDriver,
};
use parking_lot::RwLock;
use pico_common::{EtsConfig, EtsLimits, PicoResult, PicoStatus};
use std::sync::Arc;

// ps5000apg.en r4 §3.5.3, §4.51–4.53; 8-bit resolution only.
impl EtsDriver for PS5000ADriver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_ets(&self, handle: i16, ets: Option<&EtsConfig>) -> PicoResult<Option<f64>> {
        let (mode, cycles, interleave) = ets_args(ets)?;
        let mut picoseconds = 0i32;
        PicoStatus::from(unsafe {
            self.bindings
                .ps5000aSetEts(handle, mode, cycles, interleave, &mut picoseconds)
        })
        .to_result((), "set_ets")?;
        Ok(ets.map(|_| picoseconds as f64))
    }

    #[tracing::instrument(level = "debug", skip(self, buffer), err(level = "warn", Display))]
    fn set_ets_time_buffer(&self, handle: i16, buffer: Arc<RwLock<Vec<i64>>>) -> PicoResult<()> {
        let len: i32 = ets_buffer_len(&buffer)?;
        PicoStatus::from(unsafe {
            self.bindings
                .ps5000aSetEtsTimeBuffer(handle, buffer.write().as_mut_ptr(), len)
        })
        .to_result((), "set_ets_time_buffer")
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn ets_limits(&self, handle: i16) -> PicoResult<EtsLimits> {
        let (mut cycles, mut interleave) = (0i16, 0i16);
        PicoStatus::from(unsafe {
            self.bindings
                .ps5000aGetMaxEtsValues(handle, &mut cycles, &mut interleave)
        })
        .to_result(
            EtsLimits {
                cycles: cycles.max(0) as u16,
                interleave: interleave.max(0) as u16,
            },
            "ets_limits",
        )
    }
}
