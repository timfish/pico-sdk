//! Drives the AUX I/O connector of a PicoScope 3000E, 5000E or 6000E
//! through the `Outputs` API, first with the scope open, then while it
//! streams channels A and B at 1 MS/s.
//!
//! Sets AUX high, low, trigger out and back to input (released), and
//! prints what each call returned and the mean of channel B after each,
//! for a rig with AUX wired to channel B. With `--wired` it checks B:
//! above 1 V when high, below 0.5 V when low. While streaming it counts
//! the samples against the rate, to show the calls did not stop the
//! stream. Exits 1 on a failed call or check.
//!
//! `cargo run --release --example aux_io -- [serial] [--wired]`
//! (default serial `13474/0004`, the 5464E).

use pico_sdk::prelude::*;
use std::{
    env,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

const RATE: u32 = 1_000_000;
const SETTLE: Duration = Duration::from_millis(500);

#[derive(Default)]
struct Watch {
    samples: AtomicU64,
    /// Channel B's mean over the newest event, in volts.
    b_mean: Mutex<Option<f64>>,
}

impl EventHandler<OscilloscopeStreamEvent> for Watch {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        self.samples
            .fetch_add(event.length as u64, Ordering::Relaxed);
        if let Some(block) = event.channels.get(&PicoChannel::B) {
            let values = block.scale_samples();
            if !values.is_empty() {
                *self.b_mean.lock().unwrap() =
                    Some(values.iter().sum::<f64>() / values.len() as f64);
            }
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let args: Vec<String> = env::args().skip(1).collect();
    let wired = args.iter().any(|a| a == "--wired");
    let serial = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "13474/0004".into());

    // Only the psospa driver, so no other scope is touched.
    let driver = match Driver::PSOSPA
        .load(&cache_resolution())
        .expect("load psospa")
    {
        PicoDriver::Oscilloscope(driver) => driver,
        _ => unreachable!("psospa is a scope driver"),
    };
    let device = driver.open_device(Some(&serial)).expect("open");
    println!("scope {} {}", device.variant, device.serial);
    let mut failures = 0;

    let specs = device.outputs().expect("outputs");
    for spec in &specs {
        println!(
            "  {} ({}): {:?}, safe {:?}",
            spec.id, spec.label, spec.kind, spec.safe
        );
    }
    if !specs.iter().any(|s| s.id == "aux") {
        println!("FAIL no AUX output listed");
        std::process::exit(1);
    }

    println!("open:");
    let values = [
        OutputValue::On,
        OutputValue::Off,
        OutputValue::TriggerOut,
        OutputValue::Released,
    ];
    for value in &values {
        match device.set_output("aux", value) {
            Ok(set) => println!("  aux {value:?}: {set:?}"),
            Err(e) => {
                failures += 1;
                println!("  aux {value:?}: FAIL {e}");
            }
        }
    }

    let runner = device.into_streaming_device();
    let watch = Arc::new(Watch::default());
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = watch.clone();
    runner.events.subscribe(&handler);
    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(RATE);
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_5V, PicoCoupling::DC);
    config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_5V, PicoCoupling::DC);
    runner.start(config).expect("start");
    let hz = runner.get_stream().expect("streaming").samples_per_second();
    println!("streaming A and B at {hz} S/s:");
    thread::sleep(Duration::from_secs(1));

    let started = Instant::now();
    let counted = watch.samples.load(Ordering::Relaxed);
    for value in &values {
        let result = runner.set_output("aux", value);
        thread::sleep(SETTLE);
        let b = *watch.b_mean.lock().unwrap();
        let b_text = b.map_or("none".to_string(), |v| format!("{v:+.3} V"));
        match result {
            Ok(set) => println!("  aux {value:?}: {set:?}, B mean {b_text}"),
            Err(e) => {
                failures += 1;
                println!("  aux {value:?}: FAIL {e}");
                continue;
            }
        }
        if wired {
            let ok = match (value, b) {
                (OutputValue::On, Some(v)) => v > 1.0,
                (OutputValue::Off, Some(v)) => v < 0.5,
                (OutputValue::On | OutputValue::Off, None) => false,
                _ => true,
            };
            if !ok {
                failures += 1;
                println!("  FAIL B does not follow AUX {value:?}");
            }
        }
    }
    let elapsed = started.elapsed().as_secs_f64();
    let got = watch.samples.load(Ordering::Relaxed) - counted;
    let expected = elapsed * hz;
    println!(
        "samples in {elapsed:.2} s: expected {expected:.0}, got {got}, short {:.0} ({:.3} s)",
        expected - got as f64,
        (expected - got as f64) / hz,
    );

    if let Err(e) = runner.set_outputs_safe() {
        failures += 1;
        println!("safe: FAIL {e}");
    }
    runner.stop();
    runner.close();
    let waited = Instant::now();
    while runner.get_info().is_some() && waited.elapsed() < Duration::from_secs(5) {
        thread::sleep(Duration::from_millis(10));
    }
    println!("{failures} failures");
    if failures > 0 {
        std::process::exit(1);
    }
}
