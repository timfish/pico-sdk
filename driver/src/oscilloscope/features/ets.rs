use parking_lot::RwLock;
use pico_common::{EtsConfig, EtsLimits, PicoError, PicoResult};
use std::sync::Arc;

/// Equivalent-time sampling, in block mode. A capture needs a trigger
/// and a signal that repeats.
pub trait EtsDriver {
    /// Turns ETS on, or off with `None`. Returns the interval between
    /// samples in picoseconds while on.
    fn set_ets(&self, _handle: i16, _ets: Option<&EtsConfig>) -> PicoResult<Option<f64>> {
        Err(PicoError::not_supported("set_ets"))
    }
    /// The buffer the driver writes each sample's time into, in
    /// femtoseconds. The caller keeps it alive while the driver holds
    /// it, and gives it again after any change of its length.
    fn set_ets_time_buffer(&self, _handle: i16, _buffer: Arc<RwLock<Vec<i64>>>) -> PicoResult<()> {
        Err(PicoError::not_supported("set_ets_time_buffer"))
    }
    /// The most cycles and interleaves the unit takes, where the driver
    /// says.
    fn ets_limits(&self, _handle: i16) -> PicoResult<EtsLimits> {
        Err(PicoError::not_supported("ets_limits"))
    }
}

/// Turns `ets` into the mode code, cycles and interleave of the
/// headers: 0 off, 1 fast, 2 slow on every family.
pub(crate) fn ets_args(ets: Option<&EtsConfig>) -> PicoResult<(u32, i16, i16)> {
    use pico_common::{EtsMode, PicoStatus};
    let Some(ets) = ets else {
        return Ok((0, 0, 0));
    };
    let mode = match ets.mode {
        EtsMode::Fast => 1,
        EtsMode::Slow => 2,
    };
    let to_i16 = |v: u16| {
        i16::try_from(v)
            .map_err(|_| PicoError::from_status(PicoStatus::INVALID_PARAMETER, "set_ets"))
    };
    Ok((mode, to_i16(ets.cycles)?, to_i16(ets.interleave)?))
}

/// The length of an ETS time buffer as the driver takes it.
pub(crate) fn ets_buffer_len<T: TryFrom<usize>>(buffer: &Arc<RwLock<Vec<i64>>>) -> PicoResult<T> {
    T::try_from(buffer.read().len()).map_err(|_| {
        PicoError::from_status(
            pico_common::PicoStatus::INVALID_PARAMETER,
            "set_ets_time_buffer",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::EtsMode;

    #[test]
    fn modes_map_to_the_header_codes() {
        assert_eq!(ets_args(None).unwrap(), (0, 0, 0));
        let fast = EtsConfig {
            mode: EtsMode::Fast,
            cycles: 20,
            interleave: 4,
        };
        assert_eq!(ets_args(Some(&fast)).unwrap(), (1, 20, 4));
        let slow = EtsConfig {
            mode: EtsMode::Slow,
            ..fast
        };
        assert_eq!(ets_args(Some(&slow)).unwrap(), (2, 20, 4));
        let big = EtsConfig {
            cycles: 40_000,
            ..fast
        };
        assert!(ets_args(Some(&big)).is_err());
    }

    #[test]
    fn every_family_header_numbers_ets_modes_alike() {
        use pico_sys_dynamic::{ps2000, ps2000a, ps3000a, ps4000, ps5000a, ps6000};
        let headers = [
            (
                ps2000::enPS2000EtsMode_PS2000_ETS_OFF,
                ps2000::enPS2000EtsMode_PS2000_ETS_FAST,
                ps2000::enPS2000EtsMode_PS2000_ETS_SLOW,
            ),
            (
                ps2000a::enPS2000AEtsMode_PS2000A_ETS_OFF,
                ps2000a::enPS2000AEtsMode_PS2000A_ETS_FAST,
                ps2000a::enPS2000AEtsMode_PS2000A_ETS_SLOW,
            ),
            (
                ps3000a::enPS3000AEtsMode_PS3000A_ETS_OFF,
                ps3000a::enPS3000AEtsMode_PS3000A_ETS_FAST,
                ps3000a::enPS3000AEtsMode_PS3000A_ETS_SLOW,
            ),
            (
                ps4000::enPS4000EtsMode_PS4000_ETS_OFF,
                ps4000::enPS4000EtsMode_PS4000_ETS_FAST,
                ps4000::enPS4000EtsMode_PS4000_ETS_SLOW,
            ),
            (
                ps5000a::enPS5000AEtsMode_PS5000A_ETS_OFF,
                ps5000a::enPS5000AEtsMode_PS5000A_ETS_FAST,
                ps5000a::enPS5000AEtsMode_PS5000A_ETS_SLOW,
            ),
            (
                ps6000::enPS6000EtsMode_PS6000_ETS_OFF,
                ps6000::enPS6000EtsMode_PS6000_ETS_FAST,
                ps6000::enPS6000EtsMode_PS6000_ETS_SLOW,
            ),
        ];
        let fast = EtsConfig {
            mode: EtsMode::Fast,
            cycles: 1,
            interleave: 1,
        };
        let slow = EtsConfig {
            mode: EtsMode::Slow,
            ..fast
        };
        for (off, fast_code, slow_code) in headers {
            assert_eq!(ets_args(None).unwrap().0, off);
            assert_eq!(ets_args(Some(&fast)).unwrap().0, fast_code);
            assert_eq!(ets_args(Some(&slow)).unwrap().0, slow_code);
        }
    }
}
