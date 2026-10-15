use super::PS3000ADriver;
use crate::oscilloscope::{
    siggen::legacy::{legacy_calls_u32, legacy_sig_gen_driver, LegacyModel, LEGACY_TRIGGERS},
    OscilloscopeDriverInternal,
};
use pico_common::{PicoError, PicoResult};
use pico_sys_dynamic::ps3000a::{
    MIN_DWELL_COUNT, PS3000A_MIN_FREQUENCY, PS3000A_SINE_MAX_FREQUENCY,
};

/// §4.54.2: the DAC rate is in the data sheet, 20 MS/s on every 3000 model.
fn of(arbitrary: bool) -> LegacyModel {
    LegacyModel {
        min_frequency_hz: PS3000A_MIN_FREQUENCY,
        max_frequency_hz: PS3000A_SINE_MAX_FREQUENCY,
        output_v: 2.0,
        dac_hz: 20e6,
        min_dwell_count: MIN_DWELL_COUNT,
        noise: true,
        prbs: true,
        arbitrary,
        continuous_run: true,
        trigger_sources: LEGACY_TRIGGERS,
    }
}

fn sig_gen_model(driver: &PS3000ADriver, handle: i16) -> PicoResult<LegacyModel> {
    let caps = driver
        .capabilities(handle)?
        .sig_gen
        .ok_or_else(|| PicoError::not_supported("signal generator"))?;
    Ok(of(caps.arbitrary))
}

legacy_calls_u32!(
    PS3000ADriver,
    sig_gen_model,
    ps3000aSetSigGenBuiltInV2,
    ps3000aSetSigGenArbitrary,
    ps3000aSigGenFrequencyToPhase,
    ps3000aSigGenArbitraryMinMaxValues,
    ps3000aSigGenSoftwareControl
);
legacy_sig_gen_driver!(PS3000ADriver);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_come_from_the_header() {
        assert_eq!(of(true).max_frequency_hz, 1e6);
        assert!(!of(false).arbitrary);
    }
}
