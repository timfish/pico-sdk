//! Sets the outputs of the first PicoLog 1000, USB DrDAQ and ADC-24
//! attached, first with the unit open, then while it streams.
//!
//! For each output it lists, a digital output goes on then off, a PWM
//! output runs at 25% then 75% then off, a signal generator gives a
//! 1 kHz sine then 0.5 V then off, and an LED goes red, green, then
//! back to the unit. While streaming it also counts the samples that
//! came. Each call prints what it returned; at the end every output
//! goes to its safe value. A family whose driver or unit is missing is
//! skipped, so with no logger attached the program prints that and
//! exits 0. Exits 1 if any call failed.
//!
//! `cargo run --example logger_outputs`

use pico_sdk::common::{DrDAQChannel, HRDLChannel, HRDLRange, PL1000Channel, SigGenShapeKind};
use pico_sdk::prelude::*;
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

const STEP: Duration = Duration::from_millis(300);

fn report(failures: &mut u32, what: &str, result: Result<OutputValue, PicoError>) {
    match result {
        Ok(set) => println!("  {what}: {set:?}"),
        Err(e) => {
            *failures += 1;
            println!("  {what}: FAIL {e}");
        }
    }
}

/// The values each kind of output goes through, safe value last.
fn steps(spec: &OutputSpec) -> Vec<OutputValue> {
    let mut steps = match &spec.kind {
        OutputKind::Digital { .. } => vec![OutputValue::On, OutputValue::Off],
        OutputKind::Pwm { period_us } => {
            let period_us = 1000.clamp(period_us.0, period_us.1);
            [25.0, 75.0]
                .map(|duty_percent| OutputValue::Pwm {
                    period_us,
                    duty_percent,
                })
                .into()
        }
        OutputKind::SigGen(info) => {
            let mut steps = Vec::new();
            if info.shapes.contains(&SigGenShapeKind::Sine) {
                steps.push(OutputValue::SigGen(Some(SigGenConfig {
                    shape: SigGenShape::Sine,
                    frequency_hz: 1000.0,
                    amplitude_v: 1.0,
                    offset_v: 0.0,
                    duty_percent: None,
                    phase_deg: None,
                    sweep: None,
                    trigger: None,
                    filter: None,
                })));
            }
            steps.push(OutputValue::Volts(0.5));
            steps
        }
        OutputKind::Analog { volts } => vec![OutputValue::Volts((volts.0 + volts.1) / 2.0)],
        OutputKind::Indicator => vec![OutputValue::Rgb(255, 0, 0), OutputValue::Rgb(0, 255, 0)],
    };
    steps.push(spec.safe.clone());
    steps
}

fn exercise(outputs: &impl Outputs, failures: &mut u32) {
    let specs = match outputs.outputs() {
        Ok(specs) => specs,
        Err(e) => {
            *failures += 1;
            println!("  outputs: FAIL {e}");
            return;
        }
    };
    if specs.is_empty() {
        println!("  no outputs");
    }
    for spec in specs {
        println!("  {} ({}): {:?}", spec.id, spec.label, spec.kind);
        for value in steps(&spec) {
            let what = format!("{} {value:?}", spec.id);
            report(failures, &what, outputs.set_output(&spec.id, &value));
            thread::sleep(STEP);
        }
    }
}

struct Counter(AtomicU64);

macro_rules! counts {
    ($event:ty, $count:expr) => {
        impl EventHandler<$event> for Counter {
            fn new_data(&self, event: Arc<$event>) {
                let n: usize = $count(&*event);
                self.0.fetch_add(n as u64, Ordering::Relaxed);
            }
        }
    };
}

counts!(PL1000StreamingEvent, |e: &PL1000StreamingEvent| e
    .channels
    .values()
    .map(Vec::len)
    .sum());
counts!(DrDAQStreamingEvent, |e: &DrDAQStreamingEvent| e
    .channels
    .values()
    .map(Vec::len)
    .sum());
counts!(HRDLStreamingEvent, |e: &HRDLStreamingEvent| e
    .channels
    .values()
    .map(Vec::len)
    .sum());

fn open(driver: Driver) -> Option<PicoDevice> {
    let loaded = match driver.load(&LibraryResolution::Default) {
        Ok(loaded) => loaded,
        Err(e) => {
            let reason: String = e.to_string().chars().take(80).collect();
            println!("{driver:?}: driver not loaded ({reason}...); skipped");
            return None;
        }
    };
    match loaded.open_device(None) {
        Ok(device) => Some(device),
        Err(e) => {
            println!("{driver:?}: no unit ({e}); skipped");
            None
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let mut failures = 0;
    let mut found = 0;
    let counter = Arc::new(Counter(AtomicU64::new(0)));

    for driver in [Driver::PL1000, Driver::DrDAQ, Driver::PicoHRDL] {
        let Some(device) = open(driver) else {
            continue;
        };
        found += 1;
        println!("{driver:?} {}: open", device.get_serial());
        counter.0.store(0, Ordering::Relaxed);
        match device {
            PicoDevice::PL1000(device) => {
                exercise(&device, &mut failures);
                let runner = device.into_streaming_device();
                let handler: Arc<dyn EventHandler<PL1000StreamingEvent>> = counter.clone();
                runner.events.subscribe(&handler);
                let mut config = PL1000Config {
                    sample_interval_us: 10_000,
                    ..Default::default()
                };
                config.enable_channel(PL1000Channel::CHANNEL_1);
                streaming(runner.start(config), &runner, &counter, &mut failures);
                runner.close();
            }
            PicoDevice::DrDAQ(device) => {
                exercise(&device, &mut failures);
                let runner = device.into_streaming_device();
                let handler: Arc<dyn EventHandler<DrDAQStreamingEvent>> = counter.clone();
                runner.events.subscribe(&handler);
                let mut config = DrDAQConfig {
                    sample_interval_us: 10_000,
                    ..Default::default()
                };
                config.enable_channel(DrDAQChannel::CHANNEL_EXT1);
                streaming(runner.start(config), &runner, &counter, &mut failures);
                runner.close();
            }
            PicoDevice::PicoHRDL(device) => {
                exercise(&device, &mut failures);
                let runner = device.into_streaming_device();
                let handler: Arc<dyn EventHandler<HRDLStreamingEvent>> = counter.clone();
                runner.events.subscribe(&handler);
                let mut config = HRDLConfig {
                    sample_interval_ms: 100,
                    ..Default::default()
                };
                config.enable_channel(HRDLChannel::CHANNEL_1, HRDLRange::Range2500mV, true);
                streaming(runner.start(config), &runner, &counter, &mut failures);
                runner.close();
            }
            _ => unreachable!("a logger driver opened another family"),
        }
    }

    println!("{found} loggers found, {failures} calls failed");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn streaming(
    started: Result<(), PicoError>,
    runner: &impl Outputs,
    counter: &Counter,
    failures: &mut u32,
) {
    if let Err(e) = started {
        *failures += 1;
        println!("  start: FAIL {e}");
        return;
    }
    println!("  streaming");
    let before = counter.0.load(Ordering::Relaxed);
    exercise(runner, failures);
    if let Err(e) = runner.set_outputs_safe() {
        *failures += 1;
        println!("  safe: FAIL {e}");
    }
    let got = counter.0.load(Ordering::Relaxed) - before;
    println!("  samples while outputs changed: {got}");
}
