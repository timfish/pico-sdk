//! Shows the over-range flag of each channel while a scope streams.
//!
//! With no wiring (the default) every input stays open. Streams A on
//! its narrowest range and B on ±2 V, both DC at 100 kS/s, and steps:
//!
//! 1. A's offset 0 V: no block of A or B over range;
//! 2. A's offset at the largest the scope allows for that range, so a
//!    0 V input sits outside it: at least 90% of A's blocks over
//!    range, none of B's;
//! 3. A's offset 0 V again: A back in range, B still in range.
//!
//! A family with no analogue offset, or whose offset cannot leave the
//! narrowest range, skips step 2 and says so; that is not a failure.
//!
//! With `--wired`, wire the AWG output to channel A and leave B open.
//! A goes on ±200 mV, and the steps are the generator off, a 1 kHz sine
//! of 2 V pk-pk (A over range) and off again.
//!
//! Each step waits 300 ms to settle, then counts blocks for 1 s. It
//! prints a line each time a channel goes over range or comes back,
//! the counts and `PASS` or `FAIL` per step, and exits non-zero on a
//! failure. `RUST_LOG=pico_streaming=debug` shows the stream's own
//! lines for each change.
//!
//! `cargo run --release --example over_range -- ps2000a [serial]`
//! `cargo run --release --example over_range -- psospa 13474/0004`
//! `cargo run --release --example over_range -- psospa 13474/0004 --wired`

use pico_sdk::{driver::oscilloscope::OscilloscopeDriver, prelude::*};
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

/// How a step sets the stream up.
enum Setup {
    /// Channel A's analogue offset in volts.
    Offset(f64),
    Generator(Option<SigGenConfig>),
}

struct Step {
    name: String,
    setup: Setup,
    a_over: bool,
}

/// The offset furthest from 0 V the scope allows on `range`, when it
/// puts a 0 V input outside the range.
fn outside_offset(
    driver: &OscilloscopeDriver,
    handle: i16,
    range: PicoRange,
) -> Result<f64, String> {
    let full_scale = range.get_max_scaled_value();
    match driver.offset_limits(handle, range, PicoCoupling::DC) {
        Ok((low, high)) => {
            println!("A's offset limits on {range}: {low:+.3} V to {high:+.3} V");
            let offset = if high.abs() >= low.abs() { high } else { low };
            if offset.abs() > full_scale {
                Ok(offset)
            } else {
                Err(format!(
                    "the offset limit {offset:+.3} V does not leave the ±{full_scale} V range"
                ))
            }
        }
        Err(e) if e.is_not_supported() => Err("this family has no analogue offset".into()),
        Err(e) => Err(format!("offset limits: {e}")),
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let wired = env::args().any(|a| a == "--wired");
    let mut args = env::args().skip(1).filter(|a| !a.starts_with("--"));
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
    let info = device.info.clone().expect("an open device has info");
    println!("scope {} {}", device.variant, device.serial);

    let mut steps = Vec::new();
    let a_range = if wired {
        let sine = SigGenConfig::new(SigGenShape::Sine, 1000.0, 2.0);
        steps.push(Step {
            name: "generator off".into(),
            setup: Setup::Generator(None),
            a_over: false,
        });
        steps.push(Step {
            name: "1 kHz sine 2 V pk-pk".into(),
            setup: Setup::Generator(Some(sine)),
            a_over: true,
        });
        steps.push(Step {
            name: "generator off again".into(),
            setup: Setup::Generator(None),
            a_over: false,
        });
        PicoRange::X1_PROBE_200MV
    } else {
        let range = info.valid_channel_ranges[&PicoChannel::A]
            .iter()
            .copied()
            .min_by(|a, b| {
                a.get_max_scaled_value()
                    .total_cmp(&b.get_max_scaled_value())
            })
            .expect("channel A has a range");
        steps.push(Step {
            name: "offset 0 V".into(),
            setup: Setup::Offset(0.0),
            a_over: false,
        });
        match outside_offset(&driver, *info.handle, range) {
            Ok(offset) => steps.push(Step {
                name: format!("offset {offset:+.3} V"),
                setup: Setup::Offset(offset),
                a_over: true,
            }),
            Err(why) => println!("the over-range step is skipped: {why}"),
        }
        steps.push(Step {
            name: "offset 0 V again".into(),
            setup: Setup::Offset(0.0),
            a_over: false,
        });
        range
    };

    let runner = device.into_streaming_device();
    let watch = Arc::new(Watch::default());
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = watch.clone();
    runner.events.subscribe(&handler);
    let config = |offset: f64| {
        let mut config = OscilloscopeConfig::default();
        config.set_sample_rate(RATE);
        config.enable_channel(PicoChannel::A, a_range, PicoCoupling::DC);
        config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        if let Some(a) = config.channels.get_mut(&PicoChannel::A) {
            a.offset = offset;
        }
        config
    };
    runner.start(config(0.0)).expect("start");
    println!("streaming A on {a_range} and B on ±2 V");

    let mut failures = 0;
    for step in steps {
        let name = &step.name;
        println!("\n{name}");
        let set = match &step.setup {
            Setup::Offset(offset) => runner.start(config(*offset)),
            Setup::Generator(generator) => runner.set_sig_gen(generator.as_ref()).map(|_| ()),
        };
        if let Err(e) = set {
            println!("{name}: set up failed: {e}\n{name}: FAIL");
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
            let want_over = channel == PicoChannel::A && step.a_over;
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

    if wired {
        let _ = runner.set_sig_gen(None);
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
