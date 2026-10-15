use super::PS2000ADriver;
use crate::oscilloscope::{
    models::model,
    siggen::legacy::{legacy_calls_u32, legacy_sig_gen_driver, LegacyModel, LEGACY_TRIGGERS},
    OscilloscopeDriverInternal,
};
use pico_common::{PicoError, PicoInfo, PicoResult};
use pico_sys_dynamic::ps2000a::{
    PS2000A_MIN_DWELL_COUNT, PS2000A_MIN_FREQUENCY, PS2000A_SINE_MAX_FREQUENCY,
};

/// §3.51.2: the 2205 MSO's DDS runs at 48 MHz and has no PRBS; every
/// other model's at 20 MHz.
fn of(variant: &str, arbitrary: bool) -> LegacyModel {
    let first_mso = model(variant) == "2205" && variant.contains("MSO");
    LegacyModel {
        min_frequency_hz: PS2000A_MIN_FREQUENCY,
        max_frequency_hz: PS2000A_SINE_MAX_FREQUENCY,
        output_v: 2.0,
        dac_hz: if first_mso { 48e6 } else { 20e6 },
        min_dwell_count: PS2000A_MIN_DWELL_COUNT,
        noise: true,
        prbs: !first_mso,
        arbitrary,
        continuous_run: true,
        trigger_sources: LEGACY_TRIGGERS,
    }
}

fn sig_gen_model(driver: &PS2000ADriver, handle: i16) -> PicoResult<LegacyModel> {
    let caps = driver
        .capabilities(handle)?
        .sig_gen
        .ok_or_else(|| PicoError::not_supported("signal generator"))?;
    let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
    Ok(of(&variant, caps.arbitrary))
}

legacy_calls_u32!(
    PS2000ADriver,
    sig_gen_model,
    ps2000aSetSigGenBuiltInV2,
    ps2000aSetSigGenArbitrary,
    ps2000aSigGenFrequencyToPhase,
    ps2000aSigGenArbitraryMinMaxValues,
    ps2000aSigGenSoftwareControl
);
legacy_sig_gen_driver!(PS2000ADriver);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_mso_has_its_own_dds() {
        assert_eq!(of("2205 MSO", true).dac_hz, 48e6);
        assert!(!of("2205 MSO", true).prbs);
        assert_eq!(of("2205A MSO", true).dac_hz, 20e6);
        assert_eq!(of("2208A", true).max_frequency_hz, 1e6);
        assert_eq!(of("2208A", true).min_dwell_count, 3);
    }
}
