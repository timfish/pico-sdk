//! The properties signal generator API of ps6000a (§3.65–3.80) and
//! psospa (§3.61–3.74): one call per property, then `SigGenApply`, which
//! reports the values the generator took. Both families implement
//! [`PropertyCalls`] and the plan is made here once.

use super::legacy::{sweep_code, trigger_type_code};
use super::scale_samples;
use pico_common::{
    PicoError, PicoResult, PicoStatus, SigGenConfig, SigGenInfo, SigGenShape, SigGenShapeKind,
    SigGenTriggerKind, SigGenTriggerSource, Sweep,
};

/// `PICO_WAVE_TYPE` of the headers.
pub(crate) mod wave {
    pub const SINE: u32 = 17;
    pub const SQUARE: u32 = 18;
    pub const TRIANGLE: u32 = 19;
    pub const RAMP_UP: u32 = 20;
    pub const RAMP_DOWN: u32 = 21;
    pub const SINC: u32 = 22;
    pub const GAUSSIAN: u32 = 23;
    pub const HALF_SINE: u32 = 24;
    pub const DC_VOLTAGE: u32 = 1024;
    pub const WHITENOISE: u32 = 8193;
    pub const PRBS: u32 = 8194;
    pub const ARBITRARY: u32 = 268_435_456;
}

/// `PICO_SIGGEN_PARAMETER`.
pub(crate) mod param {
    pub const OUTPUT_VOLTS: u32 = 0;
    pub const SAMPLE: u32 = 1;
    pub const BUFFER_LENGTH: u32 = 2;
}

/// `PICO_SIGGEN_FILTER_STATE`.
pub(crate) mod filter {
    pub const AUTO: u32 = 0;
    pub const OFF: u32 = 1;
    pub const ON: u32 = 2;
}

/// The sources §3.78 (ps6000a) and §3.72 (psospa) list.
pub(crate) const PROPERTY_TRIGGERS: &[SigGenTriggerSource] = &[
    SigGenTriggerSource::ScopeTrigger,
    SigGenTriggerSource::Aux,
    SigGenTriggerSource::Software,
];

pub(crate) fn wave_code(shape: SigGenShapeKind) -> u32 {
    match shape {
        SigGenShapeKind::Sine => wave::SINE,
        SigGenShapeKind::Square => wave::SQUARE,
        SigGenShapeKind::Triangle => wave::TRIANGLE,
        SigGenShapeKind::RampUp => wave::RAMP_UP,
        SigGenShapeKind::RampDown => wave::RAMP_DOWN,
        SigGenShapeKind::Sinc => wave::SINC,
        SigGenShapeKind::Gaussian => wave::GAUSSIAN,
        SigGenShapeKind::HalfSine => wave::HALF_SINE,
        SigGenShapeKind::Dc => wave::DC_VOLTAGE,
        SigGenShapeKind::WhiteNoise => wave::WHITENOISE,
        SigGenShapeKind::Prbs => wave::PRBS,
        SigGenShapeKind::Arbitrary => wave::ARBITRARY,
    }
}

fn source_code(source: SigGenTriggerSource) -> u32 {
    match source {
        SigGenTriggerSource::ScopeTrigger => 1,
        SigGenTriggerSource::Aux => 2,
        SigGenTriggerSource::External => 3,
        SigGenTriggerSource::Software => 4,
    }
}

/// What `SigGenApply` writes back.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Applied {
    pub frequency_hz: f64,
    pub stop_hz: f64,
    pub increment_hz: f64,
    pub dwell_s: f64,
}

/// One family's property calls, each a thin wrapper of the binding.
pub(crate) trait PropertyCalls {
    fn waveform(&self, handle: i16, wave: u32, buffer: &mut [i16]) -> PicoStatus;
    fn range(&self, handle: i16, pk_to_pk_v: f64, offset_v: f64) -> PicoStatus;
    fn duty_cycle(&self, handle: i16, percent: f64) -> PicoStatus;
    fn trigger(&self, handle: i16, kind: u32, source: u32, cycles: u64) -> PicoStatus;
    fn frequency(&self, handle: i16, hz: f64) -> PicoStatus;
    fn frequency_sweep(
        &self,
        handle: i16,
        stop_hz: f64,
        step_hz: f64,
        dwell_s: f64,
        kind: u32,
    ) -> PicoStatus;
    /// `None` where the family has no output filter.
    fn filter(&self, handle: i16, state: u32) -> Option<PicoStatus>;
    fn apply(&self, handle: i16, on: bool, sweep: bool, trigger: bool) -> PicoResult<Applied>;
    /// Least, most and step of a `PICO_SIGGEN_PARAMETER`.
    fn limits(&self, handle: i16, param: u32) -> PicoResult<(f64, f64, f64)>;
    /// The lowest and highest frequency of a wave type.
    fn frequency_limits(&self, handle: i16, wave: u32, samples: u64) -> PicoResult<(f64, f64)>;
    fn software_trigger(&self, handle: i16, state: u32) -> PicoStatus;
    fn pause(&self, handle: i16) -> PicoStatus;
    fn restart(&self, handle: i16) -> PicoStatus;
    fn has_filter(&self) -> bool;
    fn dac_hz(&self) -> Option<f64>;
}

fn wave_refused(e: &PicoError) -> bool {
    matches!(
        e.status,
        PicoStatus::SIGGEN_WAVETYPE_NOT_SUPPORTED | PicoStatus::SIG_GEN_WAVETYPE_NOT_SUPPORTED
    )
}

/// Reads the unit's limits: the output span, the AWG buffer, and which
/// shapes the driver takes.
pub(crate) fn info<C: PropertyCalls + ?Sized>(calls: &C, handle: i16) -> PicoResult<SigGenInfo> {
    let (low_v, high_v, _) = calls.limits(handle, param::OUTPUT_VOLTS)?;
    let awg = calls
        .limits(handle, param::BUFFER_LENGTH)
        .ok()
        .map(|(fewest, most, _)| (fewest.max(1.0) as u64, most as u64));
    let frequency_hz = calls.frequency_limits(handle, wave::SINE, 0)?;

    use SigGenShapeKind::*;
    let mut shapes = Vec::new();
    for kind in [
        Sine, Square, Triangle, RampUp, RampDown, Sinc, Gaussian, HalfSine, Dc, WhiteNoise, Prbs,
        Arbitrary,
    ] {
        let samples = match kind {
            Arbitrary => match awg {
                Some((fewest, _)) => fewest,
                None => continue,
            },
            _ => 0,
        };
        match calls.frequency_limits(handle, wave_code(kind), samples) {
            Err(e) if wave_refused(&e) => {}
            _ => shapes.push(kind),
        }
    }

    Ok(SigGenInfo {
        shapes,
        frequency_hz,
        amplitude_v: (0.0, high_v - low_v),
        offset_v: (low_v, high_v),
        output_v: (low_v, high_v),
        awg_samples: awg,
        awg_dac_hz: calls.dac_hz(),
        duty_cycle: true,
        filter: calls.has_filter(),
        trigger_sources: PROPERTY_TRIGGERS.to_vec(),
    })
}

fn call(status: PicoStatus, context: &str) -> PicoResult<()> {
    status.to_result((), context)
}

/// Checks `config`, sets each property, then applies them. Every
/// property is set each time, as the driver keeps the last value of
/// each.
pub(crate) fn apply<C: PropertyCalls + ?Sized>(
    calls: &C,
    handle: i16,
    config: Option<&SigGenConfig>,
) -> PicoResult<Option<SigGenConfig>> {
    let Some(config) = config else {
        calls.apply(handle, false, false, false)?;
        return Ok(None);
    };
    info(calls, handle)?.check(config)?;
    let kind = config.shape.kind();

    let mut buffer = match &config.shape {
        SigGenShape::Arbitrary(samples) => {
            let (low, high, _) = calls.limits(handle, param::SAMPLE)?;
            scale_samples(samples, (low, high))
        }
        _ => Vec::new(),
    };
    call(
        calls.waveform(handle, wave_code(kind), &mut buffer),
        "sig_gen_waveform",
    )?;
    call(
        calls.range(handle, config.amplitude_v, config.offset_v),
        "sig_gen_range",
    )?;
    if matches!(kind, SigGenShapeKind::Square | SigGenShapeKind::Triangle) {
        call(
            calls.duty_cycle(handle, config.duty_percent.unwrap_or(50.0)),
            "sig_gen_duty_cycle",
        )?;
    }
    if kind.has_frequency() {
        call(
            calls.frequency(handle, config.frequency_hz),
            "sig_gen_frequency",
        )?;
    }
    if let Some(sweep) = &config.sweep {
        call(
            calls.frequency_sweep(
                handle,
                sweep.stop_hz,
                sweep.increment_hz,
                sweep.dwell_s,
                sweep_code(sweep.kind),
            ),
            "sig_gen_frequency_sweep",
        )?;
    }
    if let Some(trigger) = &config.trigger {
        call(
            calls.trigger(
                handle,
                trigger_type_code(trigger.kind),
                source_code(trigger.source),
                trigger.cycles_or_shots,
            ),
            "sig_gen_trigger",
        )?;
    }
    let state = match config.filter {
        None => filter::AUTO,
        Some(true) => filter::ON,
        Some(false) => filter::OFF,
    };
    if let Some(status) = calls.filter(handle, state) {
        call(status, "sig_gen_filter")?;
    }

    let applied = calls.apply(
        handle,
        true,
        config.sweep.is_some(),
        config.trigger.is_some(),
    )?;
    let mut set = config.clone();
    if kind.has_frequency() {
        set.frequency_hz = applied.frequency_hz;
    }
    if let Some(sweep) = &config.sweep {
        set.sweep = Some(Sweep {
            stop_hz: applied.stop_hz,
            increment_hz: applied.increment_hz,
            dwell_s: applied.dwell_s,
            kind: sweep.kind,
        });
    }
    Ok(Some(set))
}

/// §3.77 (ps6000a), §3.71 (psospa): with a gate trigger, `true` runs
/// the generator and `false` holds it; an edge trigger fires either way.
pub(crate) fn software_trigger<C: PropertyCalls + ?Sized>(
    calls: &C,
    handle: i16,
    state: bool,
) -> PicoResult<()> {
    let kind = if state {
        SigGenTriggerKind::GateHigh
    } else {
        SigGenTriggerKind::GateLow
    };
    call(
        calls.software_trigger(handle, trigger_type_code(kind)),
        "sig_gen_software_trigger",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::{SigGenTrigger, SweepKind};
    use std::sync::Mutex;

    #[derive(Debug, Clone, PartialEq)]
    enum Call {
        Waveform(u32, Vec<i16>),
        Range(f64, f64),
        Duty(f64),
        Trigger(u32, u32, u64),
        Frequency(f64),
        Sweep(f64, f64, f64, u32),
        Filter(u32),
        Apply(bool, bool, bool),
    }

    struct Fake {
        calls: Mutex<Vec<Call>>,
        refuses: Vec<u32>,
        filter: bool,
    }

    impl Fake {
        fn new() -> Self {
            Fake {
                calls: Mutex::new(Vec::new()),
                refuses: vec![wave::PRBS],
                filter: false,
            }
        }
        fn push(&self, call: Call) -> PicoStatus {
            self.calls.lock().unwrap().push(call);
            PicoStatus::OK
        }
        fn take(&self) -> Vec<Call> {
            std::mem::take(&mut self.calls.lock().unwrap())
        }
    }

    impl PropertyCalls for Fake {
        fn waveform(&self, _: i16, wave: u32, buffer: &mut [i16]) -> PicoStatus {
            self.push(Call::Waveform(wave, buffer.to_vec()))
        }
        fn range(&self, _: i16, pk: f64, offset: f64) -> PicoStatus {
            self.push(Call::Range(pk, offset))
        }
        fn duty_cycle(&self, _: i16, percent: f64) -> PicoStatus {
            self.push(Call::Duty(percent))
        }
        fn trigger(&self, _: i16, kind: u32, source: u32, cycles: u64) -> PicoStatus {
            self.push(Call::Trigger(kind, source, cycles))
        }
        fn frequency(&self, _: i16, hz: f64) -> PicoStatus {
            self.push(Call::Frequency(hz))
        }
        fn frequency_sweep(
            &self,
            _: i16,
            stop: f64,
            step: f64,
            dwell: f64,
            kind: u32,
        ) -> PicoStatus {
            self.push(Call::Sweep(stop, step, dwell, kind))
        }
        fn filter(&self, _: i16, state: u32) -> Option<PicoStatus> {
            self.filter.then(|| self.push(Call::Filter(state)))
        }
        fn apply(&self, _: i16, on: bool, sweep: bool, trigger: bool) -> PicoResult<Applied> {
            self.push(Call::Apply(on, sweep, trigger));
            Ok(Applied {
                frequency_hz: 999.9,
                stop_hz: 2000.1,
                increment_hz: 10.0,
                dwell_s: 0.001,
            })
        }
        fn limits(&self, _: i16, param: u32) -> PicoResult<(f64, f64, f64)> {
            Ok(match param {
                param::OUTPUT_VOLTS => (-2.5, 2.5, 0.001),
                param::SAMPLE => (-32767.0, 32767.0, 1.0),
                _ => (1.0, 32768.0, 1.0),
            })
        }
        fn frequency_limits(&self, _: i16, wave: u32, _: u64) -> PicoResult<(f64, f64)> {
            if self.refuses.contains(&wave) {
                Err(PicoError::from_status(
                    PicoStatus::SIGGEN_WAVETYPE_NOT_SUPPORTED,
                    "test",
                ))
            } else {
                Ok((0.1, 50e6))
            }
        }
        fn software_trigger(&self, _: i16, _: u32) -> PicoStatus {
            PicoStatus::OK
        }
        fn pause(&self, _: i16) -> PicoStatus {
            PicoStatus::OK
        }
        fn restart(&self, _: i16) -> PicoStatus {
            PicoStatus::OK
        }
        fn has_filter(&self) -> bool {
            self.filter
        }
        fn dac_hz(&self) -> Option<f64> {
            Some(200e6)
        }
    }

    #[test]
    fn info_reads_the_driver_limits() {
        let info = info(&Fake::new(), 1).unwrap();
        assert_eq!(info.output_v, (-2.5, 2.5));
        assert_eq!(info.amplitude_v, (0.0, 5.0));
        assert_eq!(info.awg_samples, Some((1, 32768)));
        assert_eq!(info.frequency_hz, (0.1, 50e6));
        assert!(!info.shapes.contains(&SigGenShapeKind::Prbs));
        assert!(info.shapes.contains(&SigGenShapeKind::Arbitrary));
        assert!(!info.filter);
    }

    #[test]
    fn a_square_sets_each_property_then_applies() {
        let fake = Fake::new();
        let mut config = SigGenConfig::new(SigGenShape::Square, 1000.0, 2.0);
        config.duty_percent = Some(25.0);
        config.offset_v = 0.5;
        let set = apply(&fake, 1, Some(&config)).unwrap().unwrap();
        assert_eq!(set.frequency_hz, 999.9);
        assert_eq!(
            fake.take(),
            vec![
                Call::Waveform(wave::SQUARE, vec![]),
                Call::Range(2.0, 0.5),
                Call::Duty(25.0),
                Call::Frequency(1000.0),
                Call::Apply(true, false, false),
            ]
        );
    }

    #[test]
    fn a_triangle_resets_the_duty_cycle_and_dc_sets_no_frequency() {
        let fake = Fake::new();
        apply(
            &fake,
            1,
            Some(&SigGenConfig::new(SigGenShape::Triangle, 10.0, 1.0)),
        )
        .unwrap();
        assert!(fake.take().contains(&Call::Duty(50.0)));
        let mut dc = SigGenConfig::new(SigGenShape::Dc, 0.0, 0.0);
        dc.offset_v = 1.0;
        apply(&fake, 1, Some(&dc)).unwrap();
        let calls = fake.take();
        assert!(!calls.iter().any(|c| matches!(c, Call::Frequency(_))));
        assert_eq!(calls[0], Call::Waveform(wave::DC_VOLTAGE, vec![]));
    }

    #[test]
    fn a_triggered_arbitrary_sweep() {
        let fake = Fake {
            filter: true,
            ..Fake::new()
        };
        let mut config =
            SigGenConfig::new(SigGenShape::Arbitrary(vec![-32767, 0, 32767]), 1000.0, 1.0);
        config.sweep = Some(Sweep {
            stop_hz: 2000.0,
            increment_hz: 10.0,
            dwell_s: 0.001,
            kind: SweepKind::Down,
        });
        config.trigger = Some(SigGenTrigger {
            kind: SigGenTriggerKind::Rising,
            source: SigGenTriggerSource::Software,
            cycles_or_shots: 5,
            auto_trigger: None,
        });
        config.filter = Some(true);
        let set = apply(&fake, 1, Some(&config)).unwrap().unwrap();
        assert_eq!(set.sweep.unwrap().stop_hz, 2000.1);
        assert_eq!(
            fake.take(),
            vec![
                Call::Waveform(wave::ARBITRARY, vec![-32767, 0, 32767]),
                Call::Range(1.0, 0.0),
                Call::Frequency(1000.0),
                Call::Sweep(2000.0, 10.0, 0.001, 1),
                Call::Trigger(0, 4, 5),
                Call::Filter(filter::ON),
                Call::Apply(true, true, true),
            ]
        );
    }

    #[test]
    fn refusals_come_before_any_call() {
        let fake = Fake::new();
        let mut config = SigGenConfig::new(SigGenShape::Sine, 1000.0, 2.0);
        config.trigger = Some(SigGenTrigger {
            kind: SigGenTriggerKind::Rising,
            source: SigGenTriggerSource::External,
            cycles_or_shots: 1,
            auto_trigger: None,
        });
        let e = apply(&fake, 1, Some(&config)).unwrap_err();
        assert_eq!(e.status, PicoStatus::SIGGEN_TRIGGERSOURCE_NOT_SUPPORTED);
        let prbs = SigGenConfig::new(SigGenShape::Prbs, 1000.0, 2.0);
        let e = apply(&fake, 1, Some(&prbs)).unwrap_err();
        assert_eq!(e.status, PicoStatus::SIG_GEN_WAVETYPE_NOT_SUPPORTED);
        let mut filtered = SigGenConfig::new(SigGenShape::Sine, 1000.0, 2.0);
        filtered.filter = Some(false);
        assert!(apply(&fake, 1, Some(&filtered)).is_err());
        assert!(fake.take().is_empty());
    }

    #[test]
    fn off_applies_with_the_generator_off() {
        let fake = Fake::new();
        assert_eq!(apply(&fake, 1, None).unwrap(), None);
        assert_eq!(fake.take(), vec![Call::Apply(false, false, false)]);
    }
}
