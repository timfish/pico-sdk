//! Block captures of an MSO scope triggered by its digital lines, with
//! nothing on the pod. An open input reads low against a threshold
//! above 0 V and high against one below it, so each case moves port 0's
//! threshold to make its condition true or false, and checks that the
//! block fires or waits out its 1 s timeout:
//!
//! 1. D0 high at -1 V: fires.
//! 2. D0 high at 1.5 V: waits.
//! 3. D0 low at 1.5 V: fires.
//! 4. D0 high and D1 low at -1 V: waits if the unit joins the lines of
//!    a port by AND (psospa-3 §3.59.1: each line "must be" so), fires
//!    if by OR (the wording of ps5000apg r4 §4.74). It prints which.
//! 5. A above -1 V and D0 high at -1 V (an analog and a digital term
//!    in one AND, A at ±5 V DC and open, so near 0 V): fires.
//! 6. A above 1 V and D0 high at -1 V: waits.
//!
//! An edge on a line needs a signal on the pod and is not checked
//! here. A port the config leaves off is refused before any driver
//! call. Exits 1 on a failure.
//!
//! `cargo run --release --example block_digital_trigger -- [serial]`

use pico_sdk::{common::*, prelude::*};
use std::{env, process::exit, time::Duration};

const RATE: f64 = 1e6;
const SAMPLES: u64 = 1_000;

struct Case {
    name: &'static str,
    threshold_v: f64,
    lines: Vec<DigitalBit>,
    /// A's threshold in volts, for a mixed case.
    a_above_v: Option<f64>,
    fires: Option<bool>,
}

fn trigger(case: &Case, max_adc: i16) -> TriggerConfig {
    let mut channels = Vec::new();
    let mut term = vec![(TriggerSource::Digital, TriggerState::True)];
    if let Some(v) = case.a_above_v {
        let adc = (v / 5.0 * f64::from(max_adc)) as i16;
        channels.push(ChannelTrigger::level(
            TriggerSource::Channel(PicoChannel::A),
            ThresholdDirection::Above,
            adc,
            256,
        ));
        term.push((TriggerSource::Channel(PicoChannel::A), TriggerState::True));
    }
    TriggerConfig::Advanced {
        channels,
        conditions: vec![term],
        digital: case.lines.clone(),
        pulse_width: None,
        delay: 0,
        holdoff: None,
        auto_trigger: None,
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let wanted = env::args().nth(1).or_else(|| env::var("PICO_SERIAL").ok());
    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());
    let (devices, _) = enumerator.enumerate_oscilloscopes().devices_and_errors();
    let Some(mut device) = devices
        .into_iter()
        .find(|d| wanted.as_deref().is_none_or(|w| d.serial == w))
    else {
        println!("no scope with that serial");
        exit(2);
    };
    device.ensure_open().expect("open");
    let info = device.info.clone().expect("open");
    println!("scope {} {}", device.variant, device.serial);
    if info.capabilities.digital_ports == 0 || !info.capabilities.triggers.digital {
        println!("this scope has no digital trigger");
        exit(2);
    }
    use DigitalDirection::{High, Low};
    let cases = [
        Case {
            name: "D0 high at -1 V",
            threshold_v: -1.0,
            lines: vec![DigitalBit::new(0, High)],
            a_above_v: None,
            fires: Some(true),
        },
        Case {
            name: "D0 high at 1.5 V",
            threshold_v: 1.5,
            lines: vec![DigitalBit::new(0, High)],
            a_above_v: None,
            fires: Some(false),
        },
        Case {
            name: "D0 low at 1.5 V",
            threshold_v: 1.5,
            lines: vec![DigitalBit::new(0, Low)],
            a_above_v: None,
            fires: Some(true),
        },
        Case {
            name: "D0 high and D1 low at -1 V",
            threshold_v: -1.0,
            lines: vec![DigitalBit::new(0, High), DigitalBit::new(1, Low)],
            a_above_v: None,
            fires: None,
        },
        Case {
            name: "A above -1 V and D0 high at -1 V",
            threshold_v: -1.0,
            lines: vec![DigitalBit::new(0, High)],
            a_above_v: Some(-1.0),
            fires: Some(true),
        },
        Case {
            name: "A above 1 V and D0 high at -1 V",
            threshold_v: -1.0,
            lines: vec![DigitalBit::new(0, High)],
            a_above_v: Some(1.0),
            fires: Some(false),
        },
    ];
    let mut failed = false;
    let mut device = Some(device);
    for case in &cases {
        let mut config = OscilloscopeConfig::default();
        if case.a_above_v.is_some() {
            config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_5V, PicoCoupling::DC);
        }
        config.digital.insert(
            DigitalPort::Port0,
            DigitalPortConfig::volts(case.threshold_v),
        );
        let mut settings = BlockSettings::new(RATE, SAMPLES);
        settings.overview_columns = 0;
        settings.trigger = trigger(case, info.max_adc_value_for(&config));
        let mut runner = match BlockRunner::new(device.take().unwrap(), &config, settings) {
            Ok(runner) => runner,
            Err(error) => {
                println!("{}: refused: {error}", case.name);
                exit(1);
            }
        };
        let fired = match runner.capture(Duration::from_secs(1)) {
            Ok(capture) => {
                let raw = runner.read(0, 0, 1, None).expect("raw read");
                let byte = raw.digital.get(&DigitalPort::Port0).and_then(|b| b.first());
                println!(
                    "{}: fired after {:.1} ms, port 0 reads {:08b}",
                    case.name,
                    capture.waited.as_secs_f64() * 1e3,
                    byte.copied().unwrap_or(0)
                );
                true
            }
            Err(error) if error.status == PicoStatus::TIMEOUT => {
                println!("{}: waited out 1 s", case.name);
                false
            }
            Err(error) => {
                println!("{}: failed: {error}", case.name);
                exit(1);
            }
        };
        match case.fires {
            Some(want) => {
                let ok = want == fired;
                println!("check {}: {}", case.name, if ok { "ok" } else { "FAIL" });
                failed |= !ok;
            }
            None => println!(
                "lines of a port join by {}",
                if fired { "OR" } else { "AND" }
            ),
        }
        runner.stop().ok();
        device = Some(runner.into_device());
    }

    // A trigger on a port that is off.
    let mut config = OscilloscopeConfig::default();
    config
        .digital
        .insert(DigitalPort::Port0, DigitalPortConfig::volts(1.5));
    let mut settings = BlockSettings::new(RATE, SAMPLES);
    settings.trigger = trigger(
        &Case {
            name: "D9",
            threshold_v: 1.5,
            lines: vec![DigitalBit::new(9, High)],
            a_above_v: None,
            fires: None,
        },
        info.max_adc_value_for(&config),
    );
    let refused = BlockRunner::new(device.take().unwrap(), &config, settings).err();
    let ok = refused
        .as_ref()
        .is_some_and(|e| e.status == PicoStatus::INVALID_DIGITAL_PORT);
    println!(
        "check a line on a port that is off is refused: {} ({})",
        if ok { "ok" } else { "FAIL" },
        refused.map_or("not refused".into(), |e| e.to_string())
    );
    failed |= !ok;
    exit(i32::from(failed));
}
