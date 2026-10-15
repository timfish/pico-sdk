use super::PS5000ADriver;
use crate::oscilloscope::{
    siggen::legacy::{legacy_calls_u32, legacy_sig_gen_driver, LegacyModel, LEGACY_TRIGGERS},
    OscilloscopeDriverInternal,
};
use pico_common::{PicoError, PicoResult};
use pico_sys_dynamic::ps5000a::{
    MIN_DWELL_COUNT, PS5000A_MIN_FREQUENCY, PS5000A_SINE_MAX_FREQUENCY,
};

/// §4.61.2: the DAC runs at 200 MHz on every model.
fn of(arbitrary: bool) -> LegacyModel {
    LegacyModel {
        min_frequency_hz: PS5000A_MIN_FREQUENCY,
        max_frequency_hz: PS5000A_SINE_MAX_FREQUENCY,
        output_v: 2.0,
        dac_hz: 200e6,
        min_dwell_count: MIN_DWELL_COUNT,
        noise: true,
        prbs: true,
        arbitrary,
        continuous_run: true,
        trigger_sources: LEGACY_TRIGGERS,
    }
}

fn sig_gen_model(driver: &PS5000ADriver, handle: i16) -> PicoResult<LegacyModel> {
    let caps = driver
        .capabilities(handle)?
        .sig_gen
        .ok_or_else(|| PicoError::not_supported("signal generator"))?;
    Ok(of(caps.arbitrary))
}

legacy_calls_u32!(
    PS5000ADriver,
    sig_gen_model,
    ps5000aSetSigGenBuiltInV2,
    ps5000aSetSigGenArbitrary,
    ps5000aSigGenFrequencyToPhase,
    ps5000aSigGenArbitraryMinMaxValues,
    ps5000aSigGenSoftwareControl
);
legacy_sig_gen_driver!(PS5000ADriver);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_come_from_the_header() {
        assert_eq!(of(true).max_frequency_hz, 20e6);
        assert_eq!(of(true).dac_hz, 200e6);
    }
}
