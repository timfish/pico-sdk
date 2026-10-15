use super::PS4000Driver;
use crate::oscilloscope::{
    models::model,
    siggen::legacy::{
        legacy_sig_gen_driver, phase_status, wave_code, Arbitrary, AwgLimits, BuiltIn, LegacyCalls,
        LegacyModel, LEGACY_TRIGGERS,
    },
    OscilloscopeDriverInternal,
};
use pico_common::{PicoError, PicoInfo, PicoResult, PicoStatus};
use pico_sys_dynamic::ps4000::{
    MAX_SIG_GEN_FREQ, MAX_SIG_GEN_FREQ_4262, MIN_DWELL_COUNT, PS4262_MIN_DWELL_COUNT,
};

/// §5.52.2: the 4226 and 4227 have a 20 MHz DAC; the 4262 a 192 kHz
/// one and the only PRBS. The header gives the frequencies.
fn of(variant: &str, arbitrary: bool) -> LegacyModel {
    let is_4262 = model(variant) == "4262";
    LegacyModel {
        min_frequency_hz: 0.0,
        max_frequency_hz: if is_4262 {
            MAX_SIG_GEN_FREQ_4262
        } else {
            MAX_SIG_GEN_FREQ
        },
        output_v: 2.0,
        dac_hz: if is_4262 { 192e3 } else { 20e6 },
        min_dwell_count: if is_4262 {
            PS4262_MIN_DWELL_COUNT
        } else {
            MIN_DWELL_COUNT
        },
        noise: true,
        prbs: is_4262,
        arbitrary,
        continuous_run: false,
        trigger_sources: LEGACY_TRIGGERS,
    }
}

impl LegacyCalls for PS4000Driver {
    fn model(&self, handle: i16) -> PicoResult<LegacyModel> {
        let caps = self
            .capabilities(handle)?
            .sig_gen
            .ok_or_else(|| PicoError::not_supported("signal generator"))?;
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        Ok(of(&variant, caps.arbitrary))
    }

    // The family has only the f32 call.
    fn built_in(&self, handle: i16, args: &BuiltIn) -> PicoStatus {
        let c = &args.common;
        PicoStatus::from(unsafe {
            self.bindings.ps4000SetSigGenBuiltIn(
                handle,
                c.offset_uv,
                c.pk_to_pk_uv,
                wave_code(args.shape),
                args.start_hz as f32,
                args.stop_hz as f32,
                args.increment_hz as f32,
                args.dwell_s as f32,
                c.sweep_type,
                c.operation as i16,
                c.shots,
                c.sweeps,
                c.trigger_type,
                c.trigger_source,
                0,
            )
        })
    }

    fn arbitrary(&self, handle: i16, args: &Arbitrary, samples: &mut [i16]) -> PicoStatus {
        let phases = [
            args.start_phase,
            args.stop_phase,
            args.phase_increment,
            args.dwell_count,
        ];
        if let Some(status) = phase_status(&phases) {
            return status;
        }
        let c = &args.common;
        PicoStatus::from(unsafe {
            self.bindings.ps4000SetSigGenArbitrary(
                handle,
                c.offset_uv,
                c.pk_to_pk_uv,
                args.start_phase as u32,
                args.stop_phase as u32,
                args.phase_increment as u32,
                args.dwell_count as u32,
                samples.as_mut_ptr(),
                samples.len() as i32,
                c.sweep_type,
                c.operation as i16,
                0,
                c.shots,
                c.sweeps,
                c.trigger_type,
                c.trigger_source,
                0,
            )
        })
    }

    fn frequency_to_phase(&self, handle: i16, hz: f64, len: u32) -> PicoResult<u64> {
        let mut phase = 0u32;
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000SigGenFrequencyToPhase(handle, hz, 0, len, &mut phase)
        })
        .to_result(u64::from(phase), "sig_gen_frequency_to_phase")
    }

    fn awg_limits(&self, handle: i16) -> PicoResult<AwgLimits> {
        let (mut min_value, mut max_value, mut min_len, mut max_len) = (0, 0, 0, 0);
        PicoStatus::from(unsafe {
            self.bindings.ps4000SigGenArbitraryMinMaxValues(
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
                .ps4000SigGenSoftwareControl(handle, i16::from(state))
        })
    }

    fn off(&self, handle: i16) -> PicoStatus {
        PicoStatus::from(unsafe { self.bindings.ps4000SigGenOff(handle) })
    }
}

legacy_sig_gen_driver!(PS4000Driver);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_4262_has_its_own_generator() {
        let slow = of("4262", true);
        assert_eq!((slow.max_frequency_hz, slow.dac_hz), (20_000.0, 192e3));
        assert!(slow.prbs && slow.min_dwell_count == 3);
        let fast = of("4227", true);
        assert_eq!((fast.max_frequency_hz, fast.dac_hz), (100_000.0, 20e6));
        assert!(!fast.prbs && fast.min_dwell_count == 10);
    }
}
