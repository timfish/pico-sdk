use super::PS4000ADriver;
use crate::oscilloscope::{
    siggen::legacy::{
        legacy_sig_gen_driver, wave_code, Arbitrary, AwgLimits, BuiltIn, LegacyCalls, LegacyModel,
        LEGACY_TRIGGERS,
    },
    OscilloscopeDriverInternal,
};
use pico_common::{PicoError, PicoResult, PicoStatus};
use pico_sys_dynamic::ps4000a::{
    PS4000A_AWG_DAC_FREQUENCY, PS4000A_MIN_DWELL_COUNT, PS4000A_MIN_FREQUENCY,
    PS4000A_SINE_MAX_FREQUENCY,
};

/// §4.60.2: an 80 MHz DAC on the 4824 and
/// 4000A models. The header has no continuous run after a trigger.
fn of(arbitrary: bool) -> LegacyModel {
    LegacyModel {
        min_frequency_hz: PS4000A_MIN_FREQUENCY,
        max_frequency_hz: PS4000A_SINE_MAX_FREQUENCY,
        output_v: 2.0,
        dac_hz: PS4000A_AWG_DAC_FREQUENCY,
        min_dwell_count: PS4000A_MIN_DWELL_COUNT,
        noise: true,
        prbs: true,
        arbitrary,
        continuous_run: false,
        trigger_sources: LEGACY_TRIGGERS,
    }
}

// The V2 calls take 64-bit phases, shots and sweeps.
impl LegacyCalls for PS4000ADriver {
    fn model(&self, handle: i16) -> PicoResult<LegacyModel> {
        let caps = self
            .capabilities(handle)?
            .sig_gen
            .ok_or_else(|| PicoError::not_supported("signal generator"))?;
        Ok(of(caps.arbitrary))
    }

    fn built_in(&self, handle: i16, args: &BuiltIn) -> PicoStatus {
        let c = &args.common;
        PicoStatus::from(unsafe {
            self.bindings.ps4000aSetSigGenBuiltInV2(
                handle,
                c.offset_uv,
                c.pk_to_pk_uv,
                wave_code(args.shape) as u32,
                args.start_hz,
                args.stop_hz,
                args.increment_hz,
                args.dwell_s,
                c.sweep_type,
                c.operation,
                u64::from(c.shots),
                u64::from(c.sweeps),
                c.trigger_type,
                c.trigger_source,
                0,
            )
        })
    }

    fn arbitrary(&self, handle: i16, args: &Arbitrary, samples: &mut [i16]) -> PicoStatus {
        let c = &args.common;
        PicoStatus::from(unsafe {
            self.bindings.ps4000aSetSigGenArbitraryV2(
                handle,
                c.offset_uv,
                c.pk_to_pk_uv,
                args.start_phase,
                args.stop_phase,
                args.phase_increment,
                args.dwell_count,
                samples.as_mut_ptr(),
                samples.len() as i32,
                c.sweep_type,
                c.operation,
                0,
                u64::from(c.shots),
                u64::from(c.sweeps),
                c.trigger_type,
                c.trigger_source,
                0,
            )
        })
    }

    fn frequency_to_phase(&self, handle: i16, hz: f64, len: u32) -> PicoResult<u64> {
        let mut phase = 0u64;
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000aSigGenFrequencyToPhaseV2(handle, hz, 0, len, &mut phase)
        })
        .to_result(phase, "sig_gen_frequency_to_phase")
    }

    fn awg_limits(&self, handle: i16) -> PicoResult<AwgLimits> {
        let (mut min_value, mut max_value, mut min_len, mut max_len) = (0, 0, 0, 0);
        PicoStatus::from(unsafe {
            self.bindings.ps4000aSigGenArbitraryMinMaxValues(
                handle,
                &mut min_value,
                &mut max_value,
                &mut min_len,
                &mut max_len,
            )
        })
        .to_result(
            AwgLimits {
                min_value,
                max_value,
                min_len,
                max_len,
            },
            "sig_gen_arbitrary_min_max_values",
        )
    }

    fn software_control(&self, handle: i16, state: bool) -> PicoStatus {
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000aSigGenSoftwareControl(handle, i16::from(state))
        })
    }
}

legacy_sig_gen_driver!(PS4000ADriver);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_come_from_the_header() {
        assert_eq!(of(true).dac_hz, 80e6);
        assert_eq!(of(true).min_dwell_count, 10);
        assert!(!of(true).continuous_run);
    }
}
