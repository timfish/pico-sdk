use super::PS2000Driver;
use crate::oscilloscope::{features::ets_args, EtsDriver};
use pico_common::{EtsConfig, PicoError, PicoResult, PicoStatus};

/// `ps2000_set_ets` gives the sample time in ps, or 0 when ETS is off
/// or an argument is out of range (ps2000pg.en-12 §5.25). Not the 2202.
fn set_ets_result(on: bool, picoseconds: i32) -> PicoResult<Option<f64>> {
    match (on, picoseconds) {
        (false, _) => Ok(None),
        (true, ps) if ps > 0 => Ok(Some(ps as f64)),
        (true, _) => Err(PicoError::from_status(
            PicoStatus::INVALID_PARAMETER,
            "set_ets",
        )),
    }
}

// The ps2000 driver has no time buffer: ETS samples come with their
// times from `ps2000_get_times_and_values` (§3.3).
impl EtsDriver for PS2000Driver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_ets(&self, handle: i16, ets: Option<&EtsConfig>) -> PicoResult<Option<f64>> {
        let (mode, cycles, interleave) = ets_args(ets)?;
        let picoseconds = unsafe {
            self.bindings
                .ps2000_set_ets(handle, mode as i16, cycles, interleave)
        };
        set_ets_result(ets.is_some(), picoseconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_a_refusal_only_when_turning_ets_on() {
        assert_eq!(set_ets_result(false, 0).unwrap(), None);
        assert_eq!(set_ets_result(true, 2000).unwrap(), Some(2000.0));
        assert_eq!(
            set_ets_result(true, 0).unwrap_err().status,
            PicoStatus::INVALID_PARAMETER
        );
    }
}
