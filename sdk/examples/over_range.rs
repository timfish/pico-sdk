//! Shows the over-range flag of each channel while a scope streams.
//! Wire the AWG output to channel A first; leave channel B open.
//!
//! Streams A on ±200 mV and B on ±2 V, both DC at 100 kS/s, and sets in
//! turn, without stopping the stream:
//!
//! 1. generator off: no block of A or B over range;
//! 2. a 1 kHz sine of 2 V pk-pk: at least 90% of A's blocks over range,
//!    none of B's;
//! 3. off again: A back in range, B still in range.
//!
//! Each step waits 300 ms to settle, then counts blocks for 1 s. It
//! prints a line each time a channel goes over range or comes back,
//! the counts and `PASS` or `FAIL` per step, and exits non-zero on a
//! failure. `RUST_LOG=pico_streaming=debug` shows the stream's own
//! lines for each change.
//!
//! `cargo run --release --example over_range -- ps2000a [serial]`
//! `cargo run --release --example over_range -- psospa 13474/0004`

use pico_sdk::prelude::*;
use std::{
    collections::BTreeMap,
    env,
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

const RATE: u32 = 100_000;
const CHANNELS: [PicoChannel; 2] = [PicoChannel::A, PicoChannel::B];

/// Blocks seen and blocks over range, per channel.
#[derive(Default)]
struct Counts {
    blocks: BTreeMap<PicoChannel, (usize, usize)>,
    last: BTreeMap<PicoChannel, bool>,
}

#[derive(Default)]
struct Watch(Mutex<Counts>);

impl EventHandler<OscilloscopeStreamEvent> for Watch {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        let mut counts = self.0.lock().unwrap();
        for (channel, block) in &event.channels {
            let entry = counts.blocks.entry(*channel).or_default();
            entry.0 += 1;
            entry.1 += usize::from(block.over_range);
            let before = counts.last.insert(*channel, block.over_range);
            if before != Some(block.over_range) {
                let state = if block.over_range {
                    "over range"
                } else {
                    "in range"
                };
                println!("  channel {channel}: {state}");
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
    println!("scope {} {}", device.variant, device.serial);

    let runner = device.into_streaming_device();
    let watch = Arc::new(Watch::default());
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = watch.clone();
    runner.events.subscribe(&handler);
    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(RATE);
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_200MV, PicoCoupling::DC);
    config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    runner.start(config).expect("start");
    println!("streaming A on ±200 mV and B on ±2 V");

    let sine = SigGenConfig::new(SigGenShape::Sine, 1000.0, 2.0);
    let steps: [(&str, Option<SigGenConfig>, bool); 3] = [
        ("generator off", None, false),
        ("1 kHz sine 2 V pk-pk", Some(sine), true),
        ("generator off again", None, false),
    ];

    let mut failures = 0;
    for (name, generator, a_over) in steps {
        println!("\n{name}");
        if let Err(e) = runner.set_sig_gen(generator.as_ref()) {
            println!("{name}: set_sig_gen failed: {e}\n{name}: FAIL");
            failures += 1;
            continue;
        }
        thread::sleep(Duration::from_millis(300));
        watch.0.lock().unwrap().blocks.clear();
        let started = Instant::now();
        thread::sleep(Duration::from_secs(1));
        let blocks = std::mem::take(&mut watch.0.lock().unwrap().blocks);
        let mut ok = true;
        for channel in CHANNELS {
            let (seen, over) = blocks.get(&channel).copied().unwrap_or_default();
            println!(
                "  {channel}: {over} of {seen} blocks over range in {:.2} s",
                started.elapsed().as_secs_f64()
            );
            let want_over = channel == PicoChannel::A && a_over;
            ok &= seen > 0
                && if want_over {
                    over * 10 >= seen * 9
                } else {
                    over == 0
                };
        }
        println!("{name}: {}", if ok { "PASS" } else { "FAIL" });
        if !ok {
            failures += 1;
        }
    }

    let _ = runner.set_sig_gen(None);
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
