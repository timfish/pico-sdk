//! Prints each block's over-range flag and sample codes for A and B on
//! open inputs, over setups a desktop app may stream with: 16 or 8 bit,
//! with and without MSO ports, A and B on ±20 V (A also through a x10
//! probe range). With nothing on A or B no block should be over range.
//!
//! For each setup it streams 2 s at 100 kS/s and prints the first 5
//! blocks, each later block that is over range (at most 20), and the
//! counts. `RUST_LOG=pico_driver=trace` also shows each poll's
//! `overflow_` per `PICO_STREAMING_DATA_INFO` entry.
//!
//! `cargo run --release --example over_range_check -- psospa 13474/0004`

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
struct Seen {
    blocks: usize,
    over: [usize; 2],
    printed: usize,
}

#[derive(Default)]
struct Watch(Mutex<Seen>);

fn span(samples: &Samples) -> (i64, i64) {
    let (lo, hi) = match samples {
        Samples::I8(v) => (
            v.iter().copied().min().map(i64::from),
            v.iter().copied().max().map(i64::from),
        ),
        Samples::I16(v) => (
            v.iter().copied().min().map(i64::from),
            v.iter().copied().max().map(i64::from),
        ),
        Samples::I32(v) => (
            v.iter().copied().min().map(i64::from),
            v.iter().copied().max().map(i64::from),
        ),
    };
    (lo.unwrap_or(0), hi.unwrap_or(0))
}

impl EventHandler<OscilloscopeStreamEvent> for Watch {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        let mut seen = self.0.lock().unwrap();
        seen.blocks += 1;
        let mut line = format!("  block {:>4} len {:>6}", seen.blocks, event.length);
        let mut any = false;
        for (i, ch) in [PicoChannel::A, PicoChannel::B].iter().enumerate() {
            if let Some(block) = event.channels.get(ch) {
                let (lo, hi) = span(&block.samples);
                any |= block.over_range;
                seen.over[i] += usize::from(block.over_range);
                line += &format!(
                    "  {ch}: over {:<5} codes {lo:>6}..{hi:<6} x{:.3e}",
                    block.over_range, block.multiplier
                );
            }
        }
        if !event.digital.is_empty() {
            line += &format!("  ports {}", event.digital.len());
        }
        if seen.blocks <= 5 || (any && seen.printed < 20) {
            if seen.blocks > 5 {
                seen.printed += 1;
            }
            println!("{line}");
        }
    }
}

struct Setup {
    name: &'static str,
    resolution: PicoResolution,
    a_range: PicoRange,
    ports: bool,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let mut args = env::args().skip(1);
    let driver_type = Driver::from_str(&args.next().expect("driver type, such as psospa"))
        .expect("a driver type such as psospa");
    let serial = args.next();
    let PicoDriver::Oscilloscope(driver) = driver_type
        .load(&cache_resolution())
        .unwrap_or_else(|e| panic!("could not load the {driver_type} driver: {e}"))
    else {
        panic!("{driver_type} is not a scope driver");
    };
    let device = driver.open_device(serial.as_deref()).expect("open");
    println!("scope {} {}", device.variant, device.serial);

    let setups = [
        Setup {
            name: "16 bit, A x10 ±20 V, B ±20 V, ports 0 and 1",
            resolution: PicoResolution::DR_16BIT,
            a_range: PicoRange::X10_PROBE_20V,
            ports: true,
        },
        Setup {
            name: "16 bit, A x10 ±20 V, B ±20 V, no ports",
            resolution: PicoResolution::DR_16BIT,
            a_range: PicoRange::X10_PROBE_20V,
            ports: false,
        },
        Setup {
            name: "16 bit, A ±20 V, B ±20 V, no ports",
            resolution: PicoResolution::DR_16BIT,
            a_range: PicoRange::X1_PROBE_20V,
            ports: false,
        },
        Setup {
            name: "8 bit, A ±20 V, B ±20 V, ports 0 and 1",
            resolution: PicoResolution::DR_8BIT,
            a_range: PicoRange::X1_PROBE_20V,
            ports: true,
        },
        Setup {
            name: "8 bit, A ±20 V, B ±20 V, no ports",
            resolution: PicoResolution::DR_8BIT,
            a_range: PicoRange::X1_PROBE_20V,
            ports: false,
        },
    ];

    let runner = device.into_streaming_device();
    let watch = Arc::new(Watch::default());
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = watch.clone();
    runner.events.subscribe(&handler);

    for setup in setups {
        println!("\n{}", setup.name);
        let mut config = OscilloscopeConfig::default();
        config.set_sample_rate(RATE);
        config.resolution = Some(setup.resolution);
        config.enable_channel(PicoChannel::A, setup.a_range, PicoCoupling::DC);
        config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_20V, PicoCoupling::DC);
        if setup.ports {
            for port in [DigitalPort::Port0, DigitalPort::Port1] {
                config.enable_digital_port(port, DigitalPortConfig::volts(1.5));
            }
        }
        *watch.0.lock().unwrap() = Seen::default();
        if let Err(e) = runner.start(config) {
            println!("  start failed: {e}");
            continue;
        }
        let started = Instant::now();
        thread::sleep(Duration::from_secs(2));
        let seen = std::mem::take(&mut *watch.0.lock().unwrap());
        println!(
            "  {} blocks in {:.1} s: A over range in {}, B in {}",
            seen.blocks,
            started.elapsed().as_secs_f64(),
            seen.over[0],
            seen.over[1]
        );
    }

    runner.stop();
    runner.close();
    let waited = Instant::now();
    while runner.get_info().is_some() && waited.elapsed() < Duration::from_secs(5) {
        thread::sleep(Duration::from_millis(10));
    }
}
