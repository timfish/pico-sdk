//! Streams the MSO digital ports of a PicoScope 5000E or 3000E MSO.
//!
//! Wire AUX I/O to D0 for the edge check; with nothing wired the other
//! checks still run. Threshold 1.5 V on every port. Runs, at 1 MS/s:
//!
//! 1. Channel A (±5 V DC) and PORT0 for 4 s at the unit's resolution,
//!    with AUX I/O driven high and low by turns every 250 ms.
//! 2. PORT0 and PORT1 with no channel: the stream must pick 8 bits.
//! 3. PORT0 alone at 16 bits: refused before the driver with
//!    `USE_8BIT_RESOLUTION`.
//! 4. Channel A and PORT0 at 16 bits: prints what the driver says,
//!    which no guide states; not a check.
//!
//! Checks in runs 1 and 2: each event has as many bytes for each port
//! as samples, and only the ports asked for; the driver's sample index
//! moved within 1% of the rate times the time between the two returns
//! that bound the window; no start index gap (lost samples); and no
//! restart. The samples that arrived in the window are printed only,
//! as they are off by the samples in flight. In run 1, with AUX wired
//! to D0, bit 0 should change about 16 times, once at each AUX change;
//! the example prints the count and the share of time each bit was
//! high, and counts it a failure only when `--wired` is given. Exits 1
//! on a failure.
//!
//! `cargo run --release --example digital_ports -- [serial] [--wired]`

use pico_sdk::prelude::*;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

const RATE: u32 = 1_000_000;
const THRESHOLD_V: f64 = 1.5;

#[derive(Default)]
struct Tally {
    samples: u64,
    events: u64,
    /// Events whose port bytes did not match the sample count, or that
    /// held ports not asked for.
    bad_events: u64,
    /// Samples with each bit of each port high.
    high: HashMap<DigitalPort, [u64; 8]>,
    /// Changes of each bit of each port.
    edges: HashMap<DigitalPort, [u64; 8]>,
    last: HashMap<DigitalPort, u8>,
}

struct Counter {
    ports: Vec<DigitalPort>,
    tally: Mutex<Tally>,
}

impl EventHandler<OscilloscopeStreamEvent> for Counter {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        let mut t = self.tally.lock().unwrap();
        t.events += 1;
        t.samples += event.length as u64;
        let mut keys: Vec<_> = event.digital.keys().copied().collect();
        keys.sort();
        if keys != self.ports
            || event
                .digital
                .values()
                .any(|bytes| bytes.len() != event.length)
        {
            t.bad_events += 1;
        }
        for (port, bytes) in &event.digital {
            let mut last = t.last.get(port).copied();
            let mut high = [0u64; 8];
            let mut edges = [0u64; 8];
            for byte in bytes {
                for (bit, count) in high.iter_mut().enumerate() {
                    *count += u64::from(byte >> bit & 1);
                }
                if let Some(prev) = last {
                    let changed = prev ^ byte;
                    for (bit, count) in edges.iter_mut().enumerate() {
                        *count += u64::from(changed >> bit & 1);
                    }
                }
                last = Some(*byte);
            }
            if let Some(last) = last {
                t.last.insert(*port, last);
            }
            let h = t.high.entry(*port).or_default();
            h.iter_mut().zip(high).for_each(|(a, b)| *a += b);
            let e = t.edges.entry(*port).or_default();
            e.iter_mut().zip(edges).for_each(|(a, b)| *a += b);
        }
    }
}

fn config(
    channel_a: bool,
    ports: &[DigitalPort],
    resolution: Option<PicoResolution>,
) -> OscilloscopeConfig {
    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(RATE);
    config.resolution = resolution;
    if channel_a {
        config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_5V, PicoCoupling::DC);
    }
    for port in ports {
        config.enable_digital_port(*port, DigitalPortConfig::volts(THRESHOLD_V));
    }
    config
}

struct Checks(u32);

impl Checks {
    fn check(&mut self, ok: bool, what: &str) {
        println!("  {} {what}", if ok { "ok  " } else { "FAIL" });
        if !ok {
            self.0 += 1;
        }
    }
}

/// Streams `config` for `seconds`, toggling AUX I/O if `toggle_aux`,
/// and checks the events. Returns the tally.
fn run(
    runner: &ScopeRunner,
    checks: &mut Checks,
    name: &str,
    config: OscilloscopeConfig,
    seconds: u64,
    toggle_aux: bool,
) -> Option<Tally> {
    println!("{name}");
    let mut ports: Vec<_> = config.digital.keys().copied().collect();
    ports.sort();
    let counter = Arc::new(Counter {
        ports: ports.clone(),
        tally: Mutex::default(),
    });
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = counter.clone();
    runner.events.subscribe(&handler);
    if let Err(e) = runner.start(config) {
        checks.check(false, &format!("start: {e}"));
        return None;
    }
    let stream = runner.get_stream().expect("streaming");
    println!(
        "  streaming at {} S/s, resolution {:?}, ports {:?}",
        stream.samples_per_second(),
        runner
            .with_open(|d, i| d.driver.get_resolution(*i.handle))
            .flatten(),
        stream.digital_ports(),
    );
    // Let the first second settle before counting.
    thread::sleep(Duration::from_secs(1));
    *counter.tally.lock().unwrap() = Tally::default();
    let counts_before = stream.counts();
    let started = Instant::now();
    let mut high = false;
    while started.elapsed() < Duration::from_secs(seconds) {
        if toggle_aux {
            high = !high;
            let mode = if high {
                AuxIoMode::High
            } else {
                AuxIoMode::Low
            };
            if let Err(e) = runner.set_aux_io_mode(mode) {
                checks.check(false, &format!("AUX I/O {mode:?}: {e}"));
                break;
            }
        }
        thread::sleep(Duration::from_millis(250));
    }
    let elapsed = started.elapsed().as_secs_f64();
    let counts_after = stream.counts();
    let restarted = runner
        .get_stream()
        .is_none_or(|now| !now.same_stream(&stream));
    if toggle_aux {
        let _ = runner.set_aux_io_mode(AuxIoMode::Input);
    }
    runner.stop();
    let tally = std::mem::take(&mut *counter.tally.lock().unwrap());

    let rate = stream.samples_per_second();
    println!(
        "  {} samples arrived in {elapsed:.2} s, {:.1}% of the rate (information only: \
         off by the samples in flight)",
        tally.samples,
        tally.samples as f64 / (elapsed * rate) * 100.0
    );
    checks.check(
        tally.events > 0 && tally.bad_events == 0,
        &format!(
            "{} events, {} with port bytes not matching",
            tally.events, tally.bad_events
        ),
    );
    // The driver's sample index at two returns against the rate over
    // the time between those returns.
    let samples = counts_after.samples - counts_before.samples;
    match (counts_before.last_return, counts_after.last_return) {
        (Some(t0), Some(t1)) if t1 > t0 => {
            let span = (t1 - t0).as_secs_f64();
            let share = samples as f64 / (span * rate);
            checks.check(
                (0.99..=1.01).contains(&share),
                &format!(
                    "driver index moved {samples} in {span:.3} s between returns, {:.2}% of the rate",
                    share * 100.0
                ),
            );
        }
        _ => checks.check(false, "the driver returned samples in the window"),
    }
    let gaps = counts_after.index_gaps - counts_before.index_gaps;
    checks.check(
        gaps == 0,
        &format!(
            "{gaps} start index gaps skipping {} samples ({} returns, {} buffer refills)",
            counts_after.skipped - counts_before.skipped,
            counts_after.returns - counts_before.returns,
            counts_after.refills - counts_before.refills,
        ),
    );
    checks.check(!restarted, "the stream ran without a restart");
    for port in &ports {
        let high = tally.high.get(port).copied().unwrap_or_default();
        let edges = tally.edges.get(port).copied().unwrap_or_default();
        let shares: Vec<_> = high
            .iter()
            .map(|h| format!("{:.0}%", *h as f64 / tally.samples.max(1) as f64 * 100.0))
            .collect();
        println!("  {port} high per bit (bit 0 first): {}", shares.join(" "));
        println!("  {port} changes per bit: {edges:?}");
    }
    Some(tally)
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
    let info = device.info.clone().expect("open device has info");
    println!(
        "scope {} {}: {} digital ports, resolutions {:?}",
        device.variant,
        device.serial,
        info.capabilities.digital_ports,
        info.resolutions.iter().map(|(r, _)| r).collect::<Vec<_>>()
    );
    let mut checks = Checks(0);
    checks.check(
        info.capabilities.digital_ports == 2,
        "the unit lists 2 digital ports",
    );

    let runner = device.into_streaming_device();
    use DigitalPort::*;

    if let Some(tally) = run(
        &runner,
        &mut checks,
        "1. channel A and PORT0, AUX I/O toggling",
        config(true, &[Port0], None),
        4,
        true,
    ) {
        let d0 = tally.edges.get(&Port0).map_or(0, |e| e[0]);
        let what = format!("D0 changed {d0} times for 16 AUX changes");
        if wired {
            checks.check((12..=20).contains(&d0), &what);
        } else {
            println!("  {what} (not checked; pass --wired with AUX on D0)");
        }
    }

    let alone = config(false, &[Port0, Port1], None);
    checks.check(
        info.effective_resolution(&alone)
            .is_none_or(|r| r == PicoResolution::DR_8BIT),
        "ports alone pick 8 bits",
    );
    run(
        &runner,
        &mut checks,
        "2. PORT0 and PORT1 alone",
        alone,
        3,
        false,
    );

    println!("3. PORT0 alone at 16 bits");
    let refused = runner.start(config(false, &[Port0], Some(PicoResolution::DR_16BIT)));
    checks.check(
        matches!(&refused, Err(e) if e.status == PicoStatus::USE_8BIT_RESOLUTION),
        &format!("refused: {refused:?}"),
    );

    println!("4. channel A and PORT0 at 16 bits (driver's answer, not checked)");
    println!(
        "  resolution before: {:?}",
        runner
            .with_open(|d, i| d.driver.get_resolution(*i.handle))
            .flatten()
    );
    match runner.start(config(true, &[Port0], Some(PicoResolution::DR_16BIT))) {
        Ok(()) => {
            thread::sleep(Duration::from_secs(1));
            println!("  streamed");
            runner.stop();
        }
        Err(e) => println!("  refused: {e}"),
    }

    runner.close();
    let waited = Instant::now();
    while runner.get_info().is_some() && waited.elapsed() < Duration::from_secs(5) {
        thread::sleep(Duration::from_millis(10));
    }
    println!("{} checks failed", checks.0);
    if checks.0 > 0 {
        std::process::exit(1);
    }
}
