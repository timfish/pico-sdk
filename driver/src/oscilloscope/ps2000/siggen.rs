use super::PS2000Driver;
use crate::oscilloscope::{
    siggen::legacy::{
        legacy_sig_gen_driver, phase_status, Arbitrary, AwgLimits, BuiltIn, LegacyCalls,
        LegacyModel,
    },
    OscilloscopeDriverInternal,
};
use pico_common::{PicoError, PicoResult, PicoStatus, SigGenShapeKind};
use pico_sys_dynamic::ps2000::{
    enPS2000WaveType_PS2000_DC_VOLTAGE, enPS2000WaveType_PS2000_GAUSSIAN,
    enPS2000WaveType_PS2000_HALF_SINE, enPS2000WaveType_PS2000_RAMPDOWN,
    enPS2000WaveType_PS2000_RAMPUP, enPS2000WaveType_PS2000_SINC, enPS2000WaveType_PS2000_SINE,
    enPS2000WaveType_PS2000_SQUARE, enPS2000WaveType_PS2000_TRIANGLE, PS2000_MAX_SIGGEN_FREQ,
    PS2000_MIN_SIGGEN_FREQ, PS2000_WAVE_TYPE,
};

/// §5.29: a 48 MHz DDS clock over an AWG buffer of 4096 bytes.
const DDS_HZ: f64 = 48e6;
const AWG_BUFFER: u32 = 4096;

/// §2.5: no noise, PRBS or trigger; only the 2203, 2204(A) and 2205(A)
/// have a generator (`capabilities`).
const MODEL: LegacyModel = LegacyModel {
    min_frequency_hz: PS2000_MIN_SIGGEN_FREQ,
    max_frequency_hz: PS2000_MAX_SIGGEN_FREQ,
    output_v: 2.0,
    dac_hz: DDS_HZ,
    min_dwell_count: 1,
    noise: false,
    prbs: false,
    arbitrary: true,
    continuous_run: false,
    trigger_sources: &[],
};

fn wave_type(shape: SigGenShapeKind) -> PS2000_WAVE_TYPE {
    match shape {
        SigGenShapeKind::Square => enPS2000WaveType_PS2000_SQUARE,
        SigGenShapeKind::Triangle => enPS2000WaveType_PS2000_TRIANGLE,
        SigGenShapeKind::RampUp => enPS2000WaveType_PS2000_RAMPUP,
        SigGenShapeKind::RampDown => enPS2000WaveType_PS2000_RAMPDOWN,
        SigGenShapeKind::Dc => enPS2000WaveType_PS2000_DC_VOLTAGE,
        SigGenShapeKind::Gaussian => enPS2000WaveType_PS2000_GAUSSIAN,
        SigGenShapeKind::Sinc => enPS2000WaveType_PS2000_SINC,
        SigGenShapeKind::HalfSine => enPS2000WaveType_PS2000_HALF_SINE,
        _ => enPS2000WaveType_PS2000_SINE,
    }
}

/// The family takes bytes: 0 is the bottom of the span, 255 the top.
fn to_bytes(samples: &[i16]) -> Vec<u8> {
    samples
        .iter()
        .map(|&s| ((f64::from(s) + 32767.0) * 255.0 / 65534.0).round() as u8)
        .collect()
}

/// §5.29: frequency = DDS × (phase / 2^32) × (buffer / length).
fn phase(hz: f64, len: u32) -> u64 {
    (hz * 4_294_967_296.0 * f64::from(len) / (DDS_HZ * f64::from(AWG_BUFFER))).round() as u64
}

/// The calls return 0 when a value is out of range.
fn status(ok: i16) -> PicoStatus {
    if ok == 0 {
        PicoStatus::SIG_GEN_PARAM
    } else {
        PicoStatus::OK
    }
}

impl LegacyCalls for PS2000Driver {
    fn model(&self, handle: i16) -> PicoResult<LegacyModel> {
        self.capabilities(handle)?
            .sig_gen
            .ok_or_else(|| PicoError::not_supported("signal generator"))?;
        Ok(MODEL)
    }

    fn built_in(&self, handle: i16, args: &BuiltIn) -> PicoStatus {
        let c = &args.common;
        status(unsafe {
            self.bindings.ps2000_set_sig_gen_built_in(
                handle,
                c.offset_uv,
                c.pk_to_pk_uv,
                wave_type(args.shape),
                args.start_hz as f32,
                args.stop_hz as f32,
                args.increment_hz as f32,
                args.dwell_s as f32,
                c.sweep_type,
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
        let mut bytes = to_bytes(samples);
        status(unsafe {
            self.bindings.ps2000_set_sig_gen_arbitrary(
                handle,
                c.offset_uv,
                c.pk_to_pk_uv,
                args.start_phase as u32,
                args.stop_phase as u32,
                args.phase_increment as u32,
                args.dwell_count as u32,
                bytes.as_mut_ptr(),
                bytes.len() as i32,
                c.sweep_type,
                0,
            )
        })
    }

    fn frequency_to_phase(&self, _handle: i16, hz: f64, len: u32) -> PicoResult<u64> {
        Ok(phase(hz, len))
    }

    fn awg_limits(&self, _handle: i16) -> PicoResult<AwgLimits> {
        Ok(AwgLimits {
            min_value: -32767,
            max_value: 32767,
            min_len: 1,
            max_len: AWG_BUFFER,
        })
    }

    fn software_control(&self, _handle: i16, _state: bool) -> PicoStatus {
        PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE
    }
}

legacy_sig_gen_driver!(PS2000Driver);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wave_types_follow_this_header() {
        assert_eq!(wave_type(SigGenShapeKind::Dc), 5);
        assert_eq!(wave_type(SigGenShapeKind::Sinc), 7);
        assert_eq!(wave_type(SigGenShapeKind::HalfSine), 8);
    }

    #[test]
    fn samples_become_bytes_and_phase_follows_the_guide() {
        assert_eq!(to_bytes(&[-32767, 0, 32767]), vec![0, 128, 255]);
        // A 4096-sample waveform at the DDS rate / 2^32 repeats once.
        assert_eq!(phase(DDS_HZ / 4_294_967_296.0, AWG_BUFFER), 1);
        assert_eq!(phase(1000.0, AWG_BUFFER), 89_478);
    }

    #[test]
    fn no_trigger_or_noise_is_offered() {
        let info = MODEL.info(Some((1, 4096)));
        assert!(info.trigger_sources.is_empty());
        assert!(!info.shapes.contains(&SigGenShapeKind::WhiteNoise));
        assert_eq!(info.frequency_hz, (0.0, 100_000.0));
    }
}
