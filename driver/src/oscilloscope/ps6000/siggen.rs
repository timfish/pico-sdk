use super::PS6000Driver;
use crate::oscilloscope::{
    siggen::legacy::{legacy_calls_u32, legacy_sig_gen_driver, LegacyModel, LEGACY_TRIGGERS},
    OscilloscopeDriverInternal,
};
use pico_common::{PicoError, PicoResult};
use pico_sys_dynamic::ps6000::{MIN_DWELL_COUNT, PS6000_MIN_FREQUENCY, PS6000_SINE_MAX_FREQUENCY};

/// §3.45.1: the DAC runs at 200 MHz on every model.
fn of(arbitrary: bool) -> LegacyModel {
    LegacyModel {
        min_frequency_hz: PS6000_MIN_FREQUENCY,
        max_frequency_hz: PS6000_SINE_MAX_FREQUENCY,
        output_v: 2.0,
        dac_hz: 200e6,
        min_dwell_count: MIN_DWELL_COUNT,
        noise: true,
        prbs: true,
        arbitrary,
        continuous_run: false,
        trigger_sources: LEGACY_TRIGGERS,
    }
}

fn sig_gen_model(driver: &PS6000Driver, handle: i16) -> PicoResult<LegacyModel> {
    let caps = driver
        .capabilities(handle)?
        .sig_gen
        .ok_or_else(|| PicoError::not_supported("signal generator"))?;
    Ok(of(caps.arbitrary))
}

legacy_calls_u32!(
    PS6000Driver,
    sig_gen_model,
    ps6000SetSigGenBuiltInV2,
    ps6000SetSigGenArbitrary,
    ps6000SigGenFrequencyToPhase,
    ps6000SigGenArbitraryMinMaxValues,
    ps6000SigGenSoftwareControl
);
legacy_sig_gen_driver!(PS6000Driver);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_come_from_the_header() {
        assert_eq!(of(true).max_frequency_hz, 20e6);
        assert_eq!(of(true).min_dwell_count, 3);
    }
}
