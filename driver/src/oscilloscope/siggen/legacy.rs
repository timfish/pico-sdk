//! The legacy signal generator API: one call sets the whole output
//! (ps2000a §3.51–3.55, ps3000a §4.54–4.58, ps4000 §5.52–5.53, ps4000a §4.60–4.63,
//! ps5000a §4.61–4.65, ps6000 §3.45–3.47). The families differ only in
//! type names, so each family implements [`LegacyCalls`] and the plan
//! is made here once.

use pico_common::{
    PicoError, PicoResult, PicoStatus, SigGenConfig, SigGenInfo, SigGenShape, SigGenShapeKind,
    SigGenTriggerKind, SigGenTriggerSource, SweepKind,
};

/// Header values shared by ps2000a, ps3000a, ps4000, ps4000a, ps5000a
/// and ps6000 (ps2000 numbers its wave types otherwise).
pub(crate) mod codes {
    pub const SINE: i16 = 0;
    pub const SQUARE: i16 = 1;
    pub const TRIANGLE: i16 = 2;
    pub const RAMP_UP: i16 = 3;
    pub const RAMP_DOWN: i16 = 4;
    pub const SINC: i16 = 5;
    pub const GAUSSIAN: i16 = 6;
    pub const HALF_SINE: i16 = 7;
    pub const DC_VOLTAGE: i16 = 8;

    pub const ES_OFF: u32 = 0;
    pub const WHITENOISE: u32 = 1;
    pub const PRBS: u32 = 2;

    pub const TRIGGER_NONE: u32 = 0;
    pub const TRIGGER_SCOPE: u32 = 1;
    pub const TRIGGER_AUX_IN: u32 = 2;
    pub const TRIGGER_EXT_IN: u32 = 3;
    pub const TRIGGER_SOFT: u32 = 4;

    /// ps2000a §3.51: run on with no end after the trigger.
    pub const CONTINUOUS_RUN: u32 = 0xFFFF_FFFF;
}

/// What a family and model offer, from its guide and header.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LegacyModel {
    pub min_frequency_hz: f64,
    pub max_frequency_hz: f64,
    /// The span of the output, so ±2 V on every legacy family
    /// (ps2000a §3.51, ps6000 §3.45).
    pub output_v: f64,
    /// The AWG's DAC rate; a sweep's dwell count is in its periods.
    pub dac_hz: f64,
    pub min_dwell_count: u32,
    pub noise: bool,
    pub prbs: bool,
    pub arbitrary: bool,
    /// The header has `SHOT_SWEEP_TRIGGER_CONTINUOUS_RUN`.
    pub continuous_run: bool,
    pub trigger_sources: &'static [SigGenTriggerSource],
}

pub(crate) const LEGACY_TRIGGERS: &[SigGenTriggerSource] = &[
    SigGenTriggerSource::ScopeTrigger,
    SigGenTriggerSource::External,
    SigGenTriggerSource::Aux,
    SigGenTriggerSource::Software,
];

impl LegacyModel {
    pub fn info(&self, awg_samples: Option<(u64, u64)>) -> SigGenInfo {
        use SigGenShapeKind::*;
        let mut shapes = vec![
            Sine, Square, Triangle, RampUp, RampDown, Sinc, Gaussian, HalfSine, Dc,
        ];
        if self.noise {
            shapes.push(WhiteNoise);
        }
        if self.prbs {
            shapes.push(Prbs);
        }
        if self.arbitrary && awg_samples.is_some() {
            shapes.push(Arbitrary);
        }
        SigGenInfo {
            shapes,
            frequency_hz: (self.min_frequency_hz, self.max_frequency_hz),
            amplitude_v: (0.0, 2.0 * self.output_v),
            offset_v: (-self.output_v, self.output_v),
            output_v: (-self.output_v, self.output_v),
            awg_samples: awg_samples.filter(|_| self.arbitrary),
            awg_dac_hz: self.arbitrary.then_some(self.dac_hz),
            duty_cycle: false,
            filter: false,
            trigger_sources: self.trigger_sources.to_vec(),
        }
    }
}

/// The arguments the built-in and arbitrary calls share.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Common {
    pub offset_uv: i32,
    pub pk_to_pk_uv: u32,
    pub sweep_type: u32,
    pub operation: u32,
    pub shots: u32,
    pub sweeps: u32,
    pub trigger_type: u32,
    pub trigger_source: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct BuiltIn {
    pub common: Common,
    pub shape: SigGenShapeKind,
    pub start_hz: f64,
    pub stop_hz: f64,
    pub increment_hz: f64,
    pub dwell_s: f64,
}

/// Phases from the family's `SigGenFrequencyToPhase`; index mode single.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Arbitrary {
    pub common: Common,
    pub start_phase: u64,
    pub stop_phase: u64,
    pub phase_increment: u64,
    pub dwell_count: u64,
}

/// What `SigGenArbitraryMinMaxValues` reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AwgLimits {
    pub min_value: i16,
    pub max_value: i16,
    pub min_len: u32,
    pub max_len: u32,
}

/// One family's legacy calls, each a thin wrapper of the binding.
pub(crate) trait LegacyCalls {
    fn model(&self, handle: i16) -> PicoResult<LegacyModel>;
    fn built_in(&self, handle: i16, args: &BuiltIn) -> PicoStatus;
    fn arbitrary(&self, handle: i16, args: &Arbitrary, samples: &mut [i16]) -> PicoStatus;
    fn frequency_to_phase(&self, handle: i16, hz: f64, len: u32) -> PicoResult<u64>;
    fn awg_limits(&self, handle: i16) -> PicoResult<AwgLimits>;
    fn software_control(&self, handle: i16, state: bool) -> PicoStatus;

    /// No family but ps4000 has an off call; a DC output of 0 V stands
    /// in for it.
    fn off(&self, handle: i16) -> PicoStatus {
        let model = match self.model(handle) {
            Ok(model) => model,
            Err(e) => return e.status,
        };
        let hz = model.min_frequency_hz.max(1.0);
        self.built_in(
            handle,
            &BuiltIn {
                common: Common {
                    offset_uv: 0,
                    pk_to_pk_uv: 0,
                    sweep_type: 0,
                    operation: codes::ES_OFF,
                    shots: 0,
                    sweeps: 0,
                    trigger_type: 0,
                    trigger_source: codes::TRIGGER_NONE,
                },
                shape: SigGenShapeKind::Dc,
                start_hz: hz,
                stop_hz: hz,
                increment_hz: 0.0,
                dwell_s: 0.0,
            },
        )
    }
}

pub(crate) fn wave_code(shape: SigGenShapeKind) -> i16 {
    match shape {
        SigGenShapeKind::Square => codes::SQUARE,
        SigGenShapeKind::Triangle => codes::TRIANGLE,
        SigGenShapeKind::RampUp => codes::RAMP_UP,
        SigGenShapeKind::RampDown => codes::RAMP_DOWN,
        SigGenShapeKind::Sinc => codes::SINC,
        SigGenShapeKind::Gaussian => codes::GAUSSIAN,
        SigGenShapeKind::HalfSine => codes::HALF_SINE,
        SigGenShapeKind::Dc => codes::DC_VOLTAGE,
        // Noise and PRBS go by the operation argument.
        _ => codes::SINE,
    }
}

pub(crate) fn sweep_code(kind: SweepKind) -> u32 {
    match kind {
        SweepKind::Up => 0,
        SweepKind::Down => 1,
        SweepKind::UpDown => 2,
        SweepKind::DownUp => 3,
    }
}

pub(crate) fn trigger_type_code(kind: SigGenTriggerKind) -> u32 {
    match kind {
        SigGenTriggerKind::Rising => 0,
        SigGenTriggerKind::Falling => 1,
        SigGenTriggerKind::GateHigh => 2,
        SigGenTriggerKind::GateLow => 3,
    }
}

fn trigger_source_code(source: SigGenTriggerSource) -> u32 {
    match source {
        SigGenTriggerSource::ScopeTrigger => codes::TRIGGER_SCOPE,
        SigGenTriggerSource::Aux => codes::TRIGGER_AUX_IN,
        SigGenTriggerSource::External => codes::TRIGGER_EXT_IN,
        SigGenTriggerSource::Software => codes::TRIGGER_SOFT,
    }
}

fn volts_to_uv(volts: f64) -> i64 {
    (volts * 1e6).round() as i64
}

fn invalid(why: &str) -> PicoError {
    PicoError::from_status(PicoStatus::SIG_GEN_PARAM, why)
}

/// The arguments of `config` that do not depend on the shape.
pub(crate) fn common(config: &SigGenConfig, model: &LegacyModel) -> PicoResult<Common> {
    let offset_uv =
        i32::try_from(volts_to_uv(config.offset_v)).map_err(|_| invalid("offset too large"))?;
    let pk_to_pk_uv =
        u32::try_from(volts_to_uv(config.amplitude_v)).map_err(|_| invalid("amplitude"))?;
    let operation = match config.shape.kind() {
        SigGenShapeKind::WhiteNoise => codes::WHITENOISE,
        SigGenShapeKind::Prbs => codes::PRBS,
        _ => codes::ES_OFF,
    };
    let (shots, sweeps, trigger_type, trigger_source) = match &config.trigger {
        None => (0, 0, 0, codes::TRIGGER_NONE),
        Some(trigger) => {
            let count = match trigger.cycles_or_shots {
                0 if model.continuous_run => codes::CONTINUOUS_RUN,
                0 => return Err(invalid("this generator cannot run on after a trigger")),
                // ps2000a §3.51: PS2000A_MAX_SWEEPS_SHOTS, the same on
                // every legacy family.
                n if n <= 0x3FFF_FFFF => n as u32,
                _ => return Err(invalid("more cycles or sweeps than the generator counts")),
            };
            let (shots, sweeps) = if config.sweep.is_some() {
                (0, count)
            } else {
                (count, 0)
            };
            (
                shots,
                sweeps,
                trigger_type_code(trigger.kind),
                trigger_source_code(trigger.source),
            )
        }
    };
    Ok(Common {
        offset_uv,
        pk_to_pk_uv,
        sweep_type: config.sweep.map(|s| sweep_code(s.kind)).unwrap_or(0),
        operation,
        shots,
        sweeps,
        trigger_type,
        trigger_source,
    })
}

/// The built-in call for `config`. DC and noise take a frequency in
/// range though they do not use it.
pub(crate) fn built_in(config: &SigGenConfig, model: &LegacyModel) -> PicoResult<BuiltIn> {
    let kind = config.shape.kind();
    let start_hz = if kind.has_frequency() {
        config.frequency_hz
    } else {
        model.min_frequency_hz.max(1.0)
    };
    let (stop_hz, increment_hz, dwell_s) = match &config.sweep {
        Some(sweep) => (sweep.stop_hz, sweep.increment_hz, sweep.dwell_s),
        None => (start_hz, 0.0, 0.0),
    };
    Ok(BuiltIn {
        common: common(config, model)?,
        shape: kind,
        start_hz,
        stop_hz,
        increment_hz,
        dwell_s,
    })
}

/// Checks `config` against the unit and makes the one call that sets
/// it, or turns the output off.
pub(crate) fn apply<C: LegacyCalls + ?Sized>(
    calls: &C,
    handle: i16,
    config: Option<&SigGenConfig>,
) -> PicoResult<Option<SigGenConfig>> {
    let Some(config) = config else {
        calls.off(handle).to_result((), "sig_gen_off")?;
        return Ok(None);
    };
    let model = calls.model(handle)?;
    let awg = if model.arbitrary {
        calls.awg_limits(handle).ok()
    } else {
        None
    };
    let info = model.info(awg.map(|a| (u64::from(a.min_len), u64::from(a.max_len))));
    info.check(config)?;

    match &config.shape {
        SigGenShape::Arbitrary(samples) => {
            let limits = awg.ok_or_else(|| PicoError::not_supported("arbitrary waveform"))?;
            let len = samples.len() as u32;
            let phase = |hz: f64| calls.frequency_to_phase(handle, hz, len);
            let start_phase = phase(config.frequency_hz)?;
            let (stop_phase, phase_increment, dwell_count) = match &config.sweep {
                Some(sweep) => {
                    let dwell = (sweep.dwell_s * model.dac_hz).round();
                    if dwell < f64::from(model.min_dwell_count) || dwell > u32::MAX as f64 {
                        return Err(PicoError::from_status(
                            PicoStatus::SIGGEN_INVALID_SWEEP_PARAMETERS,
                            &format!("dwell {} s is out of range", sweep.dwell_s),
                        ));
                    }
                    (
                        phase(sweep.stop_hz)?,
                        phase(sweep.increment_hz)?.max(1),
                        dwell as u64,
                    )
                }
                None => (start_phase, 0, u64::from(model.min_dwell_count)),
            };
            let args = Arbitrary {
                common: common(config, &model)?,
                start_phase,
                stop_phase,
                phase_increment,
                dwell_count,
            };
            let span = (f64::from(limits.min_value), f64::from(limits.max_value));
            let mut scaled = super::scale_samples(samples, span);
            calls
                .arbitrary(handle, &args, &mut scaled)
                .to_result((), "set_sig_gen_arbitrary")?;
        }
        _ => {
            let args = built_in(config, &model)?;
            calls
                .built_in(handle, &args)
                .to_result((), "set_sig_gen_built_in")?;
        }
    }
    Ok(Some(config.clone()))
}

/// The unit's limits, with the AWG's from the driver.
pub(crate) fn info<C: LegacyCalls + ?Sized>(calls: &C, handle: i16) -> PicoResult<SigGenInfo> {
    let model = calls.model(handle)?;
    let awg = if model.arbitrary {
        let limits = calls.awg_limits(handle)?;
        Some((u64::from(limits.min_len), u64::from(limits.max_len)))
    } else {
        None
    };
    Ok(model.info(awg))
}

pub(crate) fn software_trigger<C: LegacyCalls + ?Sized>(
    calls: &C,
    handle: i16,
    state: bool,
) -> PicoResult<()> {
    calls.model(handle)?;
    calls
        .software_control(handle, state)
        .to_result((), "sig_gen_software_control")
}

/// The status a phase that does not fit 32 bits gives.
pub(crate) fn phase_status(phases: &[u64]) -> Option<PicoStatus> {
    phases
        .iter()
        .any(|&p| u32::try_from(p).is_err())
        .then_some(PicoStatus::SIG_GEN_PARAM)
}

/// The legacy calls of a family whose arguments are 32 bits wide, as
/// in ps2000a, ps3000a, ps5000a and ps6000: `SetSigGenBuiltInV2`,
/// `SetSigGenArbitrary`, `SigGenFrequencyToPhase`,
/// `SigGenArbitraryMinMaxValues` and `SigGenSoftwareControl`.
macro_rules! legacy_calls_u32 {
    ($driver:ty, $model:path, $built_in:ident, $arbitrary:ident, $to_phase:ident,
     $min_max:ident, $soft:ident) => {
        impl $crate::oscilloscope::siggen::legacy::LegacyCalls for $driver {
            fn model(
                &self,
                handle: i16,
            ) -> pico_common::PicoResult<$crate::oscilloscope::siggen::legacy::LegacyModel> {
                $model(self, handle)
            }

            fn built_in(
                &self,
                handle: i16,
                args: &$crate::oscilloscope::siggen::legacy::BuiltIn,
            ) -> pico_common::PicoStatus {
                let c = &args.common;
                pico_common::PicoStatus::from(unsafe {
                    self.bindings.$built_in(
                        handle,
                        c.offset_uv,
                        c.pk_to_pk_uv,
                        $crate::oscilloscope::siggen::legacy::wave_code(args.shape) as _,
                        args.start_hz,
                        args.stop_hz,
                        args.increment_hz,
                        args.dwell_s,
                        c.sweep_type,
                        c.operation,
                        c.shots,
                        c.sweeps,
                        c.trigger_type,
                        c.trigger_source,
                        0,
                    )
                })
            }

            fn arbitrary(
                &self,
                handle: i16,
                args: &$crate::oscilloscope::siggen::legacy::Arbitrary,
                samples: &mut [i16],
            ) -> pico_common::PicoStatus {
                let phases = [
                    args.start_phase,
                    args.stop_phase,
                    args.phase_increment,
                    args.dwell_count,
                ];
                if let Some(status) = $crate::oscilloscope::siggen::legacy::phase_status(&phases) {
                    return status;
                }
                let c = &args.common;
                pico_common::PicoStatus::from(unsafe {
                    self.bindings.$arbitrary(
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
                        c.operation,
                        0,
                        c.shots,
                        c.sweeps,
                        c.trigger_type,
                        c.trigger_source,
                        0,
                    )
                })
            }

            fn frequency_to_phase(
                &self,
                handle: i16,
                hz: f64,
                len: u32,
            ) -> pico_common::PicoResult<u64> {
                let mut phase = 0u32;
                pico_common::PicoStatus::from(unsafe {
                    self.bindings.$to_phase(handle, hz, 0, len, &mut phase)
                })
                .to_result(u64::from(phase), "sig_gen_frequency_to_phase")
            }

            fn awg_limits(
                &self,
                handle: i16,
            ) -> pico_common::PicoResult<$crate::oscilloscope::siggen::legacy::AwgLimits> {
                let (mut min_value, mut max_value, mut min_len, mut max_len) = (0, 0, 0, 0);
                pico_common::PicoStatus::from(unsafe {
                    self.bindings.$min_max(
                        handle,
                        &mut min_value,
                        &mut max_value,
                        &mut min_len,
                        &mut max_len,
                    )
                })
                .to_result(
                    $crate::oscilloscope::siggen::legacy::AwgLimits {
                        min_value,
                        max_value,
                        min_len,
                        max_len,
                    },
                    "sig_gen_arbitrary_min_max_values",
                )
            }

            fn software_control(&self, handle: i16, state: bool) -> pico_common::PicoStatus {
                pico_common::PicoStatus::from(unsafe {
                    self.bindings.$soft(handle, i16::from(state))
                })
            }
        }
    };
}
pub(crate) use legacy_calls_u32;

/// The `SigGenDriver` impl of a legacy family.
macro_rules! legacy_sig_gen_driver {
    ($driver:ty) => {
        impl $crate::oscilloscope::SigGenDriver for $driver {
            #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
            fn sig_gen_info(
                &self,
                handle: i16,
            ) -> pico_common::PicoResult<pico_common::SigGenInfo> {
                $crate::oscilloscope::siggen::legacy::info(self, handle)
            }

            #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
            fn sig_gen_apply(
                &self,
                handle: i16,
                config: Option<&pico_common::SigGenConfig>,
            ) -> pico_common::PicoResult<Option<pico_common::SigGenConfig>> {
                $crate::oscilloscope::siggen::legacy::apply(self, handle, config)
            }

            #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
            fn sig_gen_software_trigger(
                &self,
                handle: i16,
                state: bool,
            ) -> pico_common::PicoResult<()> {
                $crate::oscilloscope::siggen::legacy::software_trigger(self, handle, state)
            }
        }
    };
}
pub(crate) use legacy_sig_gen_driver;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use pico_common::{SigGenTrigger, Sweep};
    use std::sync::Mutex;

    pub(crate) const MODEL: LegacyModel = LegacyModel {
        min_frequency_hz: 0.03,
        max_frequency_hz: 1_000_000.0,
        output_v: 2.0,
        dac_hz: 20_000_000.0,
        min_dwell_count: 3,
        noise: true,
        prbs: true,
        arbitrary: true,
        continuous_run: true,
        trigger_sources: LEGACY_TRIGGERS,
    };

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) enum Call {
        BuiltIn(BuiltIn),
        Arbitrary(Arbitrary, Vec<i16>),
        Soft(bool),
    }

    pub(crate) struct Fake {
        pub model: LegacyModel,
        pub calls: Mutex<Vec<Call>>,
    }

    impl Fake {
        pub fn new(model: LegacyModel) -> Self {
            Fake {
                model,
                calls: Mutex::new(Vec::new()),
            }
        }
        fn take(&self) -> Vec<Call> {
            std::mem::take(&mut self.calls.lock().unwrap())
        }
    }

    impl LegacyCalls for Fake {
        fn model(&self, _handle: i16) -> PicoResult<LegacyModel> {
            Ok(self.model)
        }
        fn built_in(&self, _handle: i16, args: &BuiltIn) -> PicoStatus {
            self.calls.lock().unwrap().push(Call::BuiltIn(*args));
            PicoStatus::OK
        }
        fn arbitrary(&self, _handle: i16, args: &Arbitrary, samples: &mut [i16]) -> PicoStatus {
            self.calls
                .lock()
                .unwrap()
                .push(Call::Arbitrary(*args, samples.to_vec()));
            PicoStatus::OK
        }
        // 2^32 / 20 MHz per hertz of a 1-sample waveform, as the guide's
        // formula with an AWG buffer of one sample.
        fn frequency_to_phase(&self, _handle: i16, hz: f64, len: u32) -> PicoResult<u64> {
            Ok((hz * 4_294_967_296.0 * f64::from(len) / self.model.dac_hz) as u64)
        }
        fn awg_limits(&self, _handle: i16) -> PicoResult<AwgLimits> {
            Ok(AwgLimits {
                min_value: -32767,
                max_value: 32767,
                min_len: 1,
                max_len: 8192,
            })
        }
        fn software_control(&self, _handle: i16, state: bool) -> PicoStatus {
            self.calls.lock().unwrap().push(Call::Soft(state));
            PicoStatus::OK
        }
    }

    fn sine() -> SigGenConfig {
        SigGenConfig::new(SigGenShape::Sine, 1000.0, 2.0)
    }

    #[test]
    fn a_sine_is_one_built_in_call_in_microvolts() {
        let fake = Fake::new(MODEL);
        let mut config = sine();
        config.offset_v = -0.25;
        assert_eq!(apply(&fake, 1, Some(&config)).unwrap(), Some(config));
        assert_eq!(
            fake.take(),
            vec![Call::BuiltIn(BuiltIn {
                common: Common {
                    offset_uv: -250_000,
                    pk_to_pk_uv: 2_000_000,
                    sweep_type: 0,
                    operation: codes::ES_OFF,
                    shots: 0,
                    sweeps: 0,
                    trigger_type: 0,
                    trigger_source: codes::TRIGGER_NONE,
                },
                shape: SigGenShapeKind::Sine,
                start_hz: 1000.0,
                stop_hz: 1000.0,
                increment_hz: 0.0,
                dwell_s: 0.0,
            })]
        );
    }

    #[test]
    fn off_is_dc_at_zero_volts() {
        let fake = Fake::new(MODEL);
        assert_eq!(apply(&fake, 1, None).unwrap(), None);
        let Call::BuiltIn(args) = &fake.take()[0] else {
            panic!("not a built-in call");
        };
        assert_eq!(args.shape, SigGenShapeKind::Dc);
        assert_eq!((args.common.offset_uv, args.common.pk_to_pk_uv), (0, 0));
    }

    #[test]
    fn noise_and_prbs_go_by_the_operation() {
        let fake = Fake::new(MODEL);
        let noise = SigGenConfig::new(SigGenShape::WhiteNoise, 0.0, 1.0);
        apply(&fake, 1, Some(&noise)).unwrap();
        let prbs = SigGenConfig::new(SigGenShape::Prbs, 10_000.0, 1.0);
        apply(&fake, 1, Some(&prbs)).unwrap();
        let ops: Vec<u32> = fake
            .take()
            .iter()
            .map(|c| match c {
                Call::BuiltIn(args) => args.common.operation,
                _ => panic!("not a built-in call"),
            })
            .collect();
        assert_eq!(ops, vec![codes::WHITENOISE, codes::PRBS]);
    }

    #[test]
    fn a_triggered_sweep_counts_sweeps_not_shots() {
        let fake = Fake::new(MODEL);
        let mut config = sine();
        config.sweep = Some(Sweep {
            stop_hz: 10_000.0,
            increment_hz: 100.0,
            dwell_s: 0.01,
            kind: SweepKind::UpDown,
        });
        config.trigger = Some(SigGenTrigger {
            kind: SigGenTriggerKind::GateHigh,
            source: SigGenTriggerSource::Software,
            cycles_or_shots: 3,
            auto_trigger: None,
        });
        apply(&fake, 1, Some(&config)).unwrap();
        let Call::BuiltIn(args) = &fake.take()[0] else {
            panic!("not a built-in call");
        };
        assert_eq!(args.common.sweep_type, 2);
        assert_eq!((args.common.shots, args.common.sweeps), (0, 3));
        assert_eq!(args.common.trigger_type, 2);
        assert_eq!(args.common.trigger_source, codes::TRIGGER_SOFT);
        assert_eq!(
            (args.stop_hz, args.increment_hz, args.dwell_s),
            (10_000.0, 100.0, 0.01)
        );

        config.sweep = None;
        config.trigger.as_mut().unwrap().cycles_or_shots = 0;
        apply(&fake, 1, Some(&config)).unwrap();
        let Call::BuiltIn(args) = &fake.take()[0] else {
            panic!("not a built-in call");
        };
        assert_eq!(
            (args.common.shots, args.common.sweeps),
            (codes::CONTINUOUS_RUN, 0)
        );
    }

    #[test]
    fn run_on_after_a_trigger_needs_the_header_constant() {
        let fake = Fake::new(LegacyModel {
            continuous_run: false,
            ..MODEL
        });
        let mut config = sine();
        config.trigger = Some(SigGenTrigger {
            kind: SigGenTriggerKind::Rising,
            source: SigGenTriggerSource::ScopeTrigger,
            cycles_or_shots: 0,
            auto_trigger: None,
        });
        let e = apply(&fake, 1, Some(&config)).unwrap_err();
        assert_eq!(e.status, PicoStatus::SIG_GEN_PARAM);
        assert!(fake.take().is_empty());
    }

    #[test]
    fn an_arbitrary_waveform_takes_phases_from_the_driver() {
        let fake = Fake::new(MODEL);
        let samples = vec![0, 32767, 0, -32767];
        let config = SigGenConfig::new(SigGenShape::Arbitrary(samples.clone()), 1000.0, 1.0);
        apply(&fake, 1, Some(&config)).unwrap();
        let phase = (1000.0 * 4_294_967_296.0 * 4.0 / 20e6) as u64;
        assert_eq!(
            fake.take(),
            vec![Call::Arbitrary(
                Arbitrary {
                    common: common(&config, &MODEL).unwrap(),
                    start_phase: phase,
                    stop_phase: phase,
                    phase_increment: 0,
                    dwell_count: 3,
                },
                samples
            )]
        );
    }

    #[test]
    fn an_arbitrary_sweep_dwells_in_dac_periods() {
        let fake = Fake::new(MODEL);
        let mut config = SigGenConfig::new(SigGenShape::Arbitrary(vec![1, 2]), 100.0, 1.0);
        config.sweep = Some(Sweep {
            stop_hz: 200.0,
            increment_hz: 10.0,
            dwell_s: 1e-3,
            kind: SweepKind::Up,
        });
        apply(&fake, 1, Some(&config)).unwrap();
        let Call::Arbitrary(args, _) = &fake.take()[0] else {
            panic!("not an arbitrary call");
        };
        assert_eq!(args.dwell_count, 20_000);
        assert!(args.stop_phase > args.start_phase && args.phase_increment > 0);

        config.sweep.as_mut().unwrap().dwell_s = 1e-9;
        let e = apply(&fake, 1, Some(&config)).unwrap_err();
        assert_eq!(e.status, PicoStatus::SIGGEN_INVALID_SWEEP_PARAMETERS);
    }

    #[test]
    fn a_function_generator_refuses_arbitrary_and_unknown_settings() {
        let fake = Fake::new(LegacyModel {
            arbitrary: false,
            ..MODEL
        });
        let config = SigGenConfig::new(SigGenShape::Arbitrary(vec![1, 2]), 100.0, 1.0);
        let e = apply(&fake, 1, Some(&config)).unwrap_err();
        assert_eq!(e.status, PicoStatus::AWG_NOT_SUPPORTED);

        let mut config = sine();
        config.duty_percent = Some(25.0);
        let e = apply(&fake, 1, Some(&config)).unwrap_err();
        assert!(e.is_not_supported());

        let mut config = sine();
        config.amplitude_v = 3.0;
        config.offset_v = 1.0;
        let e = apply(&fake, 1, Some(&config)).unwrap_err();
        assert_eq!(e.status, PicoStatus::SIGGEN_OUTPUT_OVER_VOLTAGE);
        assert!(fake.take().is_empty());
    }
}
