//! Drives a scope's signal generator and checks each output from the
//! samples of channel A. Wire the AWG output to channel A first.
//!
//! Streams A at 100 kS/s on ±2 V DC. The ps2000a and psospa drivers
//! answer `BUSY` to most generator changes while the scope streams
//! (ps2000a guide §3.52: set up the generator before starting data
//! acquisition), so each step stops the stream, sets the generator and
//! starts the stream again. In turn:
//!
//! 1. a 1 kHz sine of 2 V pk-pk: frequency within 1%, pk-pk within 10%;
//! 2. a 2 kHz square of 1 V pk-pk on a 0.5 V offset: frequency within
//!    1%, low near 0 V and high near 1 V (±0.1 V);
//! 3. an arbitrary 8-step staircase at 500 Hz, 1.5 V pk-pk, when the
//!    unit has an AWG: frequency within 1%, pk-pk within 15%;
//! 4. off: pk-pk under 0.1 V and mean within 0.1 V of 0 V.
//!
//! Each step waits 300 ms for the output to settle, then measures 1 s.
//! It prints the generator's limits, what each set returned and each
//! measurement, then `PASS` or `FAIL` per step, and exits non-zero on a
//! failure.
//!
//! `cargo run --release --example signal_generator -- ps2000a [serial]`
//! `cargo run --release --example signal_generator -- psospa 13474/0004`

use pico_sdk::prelude::*;
use std::{
    env,
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

const RATE: u32 = 100_000;

#[derive(Default)]
struct Collect(Mutex<Vec<f64>>);

impl EventHandler<OscilloscopeStreamEvent> for Collect {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        if let Some(block) = event.channels.get(&PicoChannel::A) {
            self.0.lock().unwrap().extend(block.scale_samples());
        }
    }
}

struct Measured {
    low: f64,
    high: f64,
    mean: f64,
    hz: f64,
}

/// Rising crossings of the middle, with 10% of the swing as hysteresis.
fn measure(samples: &[f64], rate: f64) -> Measured {
    let low = samples.iter().copied().fold(f64::INFINITY, f64::min);
    let high = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean = samples.iter().sum::<f64>() / samples.len().max(1) as f64;
    let middle = (low + high) / 2.0;
    let band = (high - low) * 0.1;
    let mut armed = false;
    let mut rises = Vec::new();
    for (i, &v) in samples.iter().enumerate() {
        if v < middle - band {
            armed = true;
        } else if armed && v > middle + band {
            armed = false;
            rises.push(i);
        }
    }
    let hz = match (rises.first(), rises.last()) {
        (Some(&first), Some(&last)) if rises.len() > 1 => {
            (rises.len() - 1) as f64 * rate / (last - first) as f64
        }
        _ => 0.0,
    };
    Measured {
        low,
        high,
        mean,
        hz,
    }
}

fn near(value: f64, want: f64, part: f64) -> bool {
    (value - want).abs() <= want.abs() * part
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let mut args = env::args().skip(1);
    let driver_type = Driver::from_str(&args.next().expect("driver type, such as ps2000a"))
        .expect("a driver type such as ps2000a or psospa");
    let serial = args.next();

    let PicoDriver::Oscilloscope(driver) = driver_type
        .load(&cache_resolution())
        .unwrap_or_else(|e| panic!("could not load the {driver_type} driver: {e}"))
    else {
        panic!("{driver_type} is not a scope driver");
    };
    let device = driver.open_device(serial.as_deref()).expect("open");
    let caps = device
        .info
        .as_ref()
        .expect("an open device has info")
        .capabilities
        .sig_gen;
    println!(
        "scope {} {}, generator {caps:?}",
        device.variant, device.serial
    );

    let runner = device.into_streaming_device();
    let info = runner.sig_gen_info().expect("sig_gen_info");
    println!("{info:#?}");

    let collect = Arc::new(Collect::default());
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = collect.clone();
    runner.events.subscribe(&handler);
    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(RATE);
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    runner.start(config.clone()).expect("start");
    let rate = runner.get_stream().expect("streaming").samples_per_second();
    println!("streaming A at {rate} S/s");

    let sine = SigGenConfig::new(SigGenShape::Sine, 1000.0, 2.0);
    let mut square = SigGenConfig::new(SigGenShape::Square, 2000.0, 1.0);
    square.offset_v = 0.5;
    let stairs: Vec<i16> = (0..8).map(|i| (-32767 + i * 65534 / 7) as i16).collect();
    let arbitrary = SigGenConfig::new(SigGenShape::Arbitrary(stairs), 500.0, 1.5);
    let has_awg = info.shapes.contains(&SigGenShapeKind::Arbitrary);

    let mut steps: Vec<(&str, Option<SigGenConfig>)> = vec![
        ("1 kHz sine 2 V pk-pk", Some(sine)),
        ("2 kHz square 0 to 1 V", Some(square)),
    ];
    if has_awg {
        steps.push(("500 Hz staircase 1.5 V pk-pk", Some(arbitrary)));
    } else {
        println!("no AWG on this unit: the staircase is skipped");
    }
    steps.push(("off", None));

    let mut failures = 0;
    let stream = config;
    for (name, config) in steps {
        // `stop` returns before the stream has ended.
        runner.stop();
        while runner.get_stream().is_some() {
            thread::sleep(Duration::from_millis(10));
        }
        let set = runner.set_sig_gen(config.as_ref());
        println!("\n{name}: set returned {set:?}");
        runner.start(stream.clone()).expect("start");
        if set.is_err() {
            failures += 1;
            println!("{name}: FAIL");
            continue;
        }
        thread::sleep(Duration::from_millis(300));
        collect.0.lock().unwrap().clear();
        let started = Instant::now();
        thread::sleep(Duration::from_secs(1));
        let samples = std::mem::take(&mut *collect.0.lock().unwrap());
        let m = measure(&samples, rate);
        println!(
            "{} samples in {:.2} s: low {:.3} V, high {:.3} V, pk-pk {:.3} V, mean {:.3} V, {:.2} Hz",
            samples.len(),
            started.elapsed().as_secs_f64(),
            m.low,
            m.high,
            m.high - m.low,
            m.mean,
            m.hz
        );
        let pk = m.high - m.low;
        let ok = !samples.is_empty()
            && match &config {
                Some(c) => match c.shape {
                    SigGenShape::Square => {
                        near(m.hz, c.frequency_hz, 0.01)
                            && (m.low - (c.offset_v - c.amplitude_v / 2.0)).abs() <= 0.1
                            && (m.high - (c.offset_v + c.amplitude_v / 2.0)).abs() <= 0.1
                    }
                    SigGenShape::Arbitrary(_) => {
                        near(m.hz, c.frequency_hz, 0.01) && near(pk, c.amplitude_v, 0.15)
                    }
                    _ => near(m.hz, c.frequency_hz, 0.01) && near(pk, c.amplitude_v, 0.1),
                },
                None => pk < 0.1 && m.mean.abs() < 0.1,
            };
        println!("{name}: {}", if ok { "PASS" } else { "FAIL" });
        if !ok {
            failures += 1;
        }
    }

    runner.stop();
    runner.close();
    let waited = Instant::now();
    while runner.get_info().is_some() && waited.elapsed() < Duration::from_secs(5) {
        thread::sleep(Duration::from_millis(10));
    }
    println!("\n{failures} steps failed");
    if failures > 0 {
        std::process::exit(1);
    }
}
