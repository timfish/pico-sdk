//! Block and rapid block captures of the MSO digital ports of a
//! PicoScope 5000E, 3000E, 6000E or a 2000A, 3000A or 5000A MSO.
//! One threshold on every port, 1.5 V by default; no trigger, so each
//! block starts at once. Runs:
//!
//! 1. Block mode: channel A (±5 V DC) and every port, 3 blocks of
//!    100 000 samples at 10 MS/s, each read whole and raw.
//! 2. Rapid block: every port and no channel, so the unit runs at
//!    8 bits; 8 segments of 10 000 samples at 10 MS/s, read in one
//!    bulk read.
//!
//! Checks: every raw read holds each port asked for and no other, with
//! a byte for each sample; the downsampled overview holds no ports;
//! rapid block fills every segment. It prints the share of time each
//! line was high and its edges in the first block, and the speed of
//! each read. With nothing on the pod the lines read low, and high
//! with a threshold below 0 V, such as -1. Exits 1 on a failure.
//!
//! `cargo run --release --example block_ports -- [serial] [threshold_v]`

use pico_sdk::prelude::*;
use std::{collections::HashMap, env, process::exit, time::Duration};

const RATE: f64 = 10e6;

fn check(name: &str, ok: bool, failed: &mut bool) {
    println!("check {name}: {}", if ok { "ok" } else { "FAIL" });
    *failed |= !ok;
}

/// Prints the share of samples each line is high and its edges.
fn lines(digital: &HashMap<DigitalPort, Vec<u8>>) {
    let mut ports: Vec<_> = digital.keys().copied().collect();
    ports.sort();
    for port in ports {
        let bytes = &digital[&port];
        let mut row = String::new();
        for bit in 0..8 {
            let line = port.index() as usize * 8 + bit;
            let high = bytes.iter().filter(|b| (*b >> bit) & 1 == 1).count();
            let edges = bytes
                .windows(2)
                .filter(|w| ((w[0] ^ w[1]) >> bit) & 1 == 1)
                .count();
            let share = 100.0 * high as f64 / bytes.len().max(1) as f64;
            row += &format!(" D{line}={share:.0}%/{edges}");
        }
        println!("  {port}:{row}");
    }
}

/// Every port asked for and no other, a byte a sample.
fn ports_whole(data: &BlockData, ports: &[DigitalPort], samples: u64) -> bool {
    data.digital.len() == ports.len()
        && ports
            .iter()
            .all(|p| data.digital.get(p).map(Vec::len) == Some(samples as usize))
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let mut args = env::args().skip(1);
    let wanted = args.next().or_else(|| env::var("PICO_SERIAL").ok());
    let threshold: f64 = args
        .next()
        .map_or(1.5, |a| a.parse().expect("threshold in V"));
    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());
    let (devices, _) = enumerator.enumerate_oscilloscopes().devices_and_errors();
    let mut device = devices
        .into_iter()
        .find(|d| wanted.as_deref().is_none_or(|w| d.serial == w))
        .expect("no scope with that serial");
    device.ensure_open().expect("open");
    let info = device.info.clone().expect("open");
    let ports = DigitalPort::first(info.capabilities.digital_ports);
    println!(
        "scope {} {}, {} ports",
        device.variant,
        device.serial,
        ports.len()
    );
    if ports.is_empty() {
        println!("this scope has no digital ports");
        exit(2);
    }
    let mut failed = false;

    // 1. Block mode with channel A.
    let mut config = OscilloscopeConfig::default();
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_5V, PicoCoupling::DC);
    for port in &ports {
        config
            .digital
            .insert(*port, DigitalPortConfig::volts(threshold));
    }
    let samples = 100_000;
    let mut settings = BlockSettings::new(RATE, samples);
    settings.overview_columns = 1000;
    let mut runner = match BlockRunner::new(device, &config, settings) {
        Ok(runner) => runner,
        Err(error) => {
            println!("block mode refused: {error}");
            exit(1);
        }
    };
    println!(
        "block: {:.0} S/s, {samples} samples",
        runner.plan().timebase.samples_per_second()
    );
    for block in 0..3 {
        let capture = match runner.capture(Duration::from_secs(5)) {
            Ok(capture) => capture,
            Err(error) => {
                println!("block {block} failed: {error}");
                exit(1);
            }
        };
        if block == 0 {
            let overview = &capture.overview[0];
            check(
                "overview holds no ports",
                overview.digital.is_empty() && overview.channels.contains_key(&PicoChannel::A),
                &mut failed,
            );
        }
        let raw = runner.read(0, 0, samples, None).expect("raw read");
        println!(
            "block {block}: waited {:.1} ms, read {} samples in {:.1} ms ({:.1} MB/s)",
            capture.waited.as_secs_f64() * 1e3,
            raw.len(),
            raw.transfer.as_secs_f64() * 1e3,
            raw.bytes() as f64 / raw.transfer.as_secs_f64().max(1e-9) / 1e6,
        );
        check(
            "block holds every port",
            ports_whole(&raw, &ports, samples) && raw.len() == samples as usize,
            &mut failed,
        );
        if block == 0 {
            lines(&raw.digital);
        }
    }
    let device = runner.into_device();

    // 2. Rapid block, ports alone.
    let mut config = OscilloscopeConfig::default();
    for port in &ports {
        config
            .digital
            .insert(*port, DigitalPortConfig::volts(threshold));
    }
    let samples = 10_000;
    let mut settings = BlockSettings::new(RATE, samples);
    settings.segments = 8;
    settings.overview_columns = 0;
    let mut runner = match BlockRunner::new(device, &config, settings) {
        Ok(runner) => runner,
        Err(error) => {
            println!("rapid block refused: {error}");
            exit(1);
        }
    };
    println!(
        "rapid block, ports alone: {:.0} S/s, 8 segments of {samples}, resolution {:?}",
        runner.plan().timebase.samples_per_second(),
        runner
            .device()
            .driver
            .get_resolution(*runner.device().info.as_ref().unwrap().handle),
    );
    let capture = match runner.capture(Duration::from_secs(5)) {
        Ok(capture) => capture,
        Err(error) => {
            println!("rapid block failed: {error}");
            exit(1);
        }
    };
    check("every segment", capture.segments.len() == 8, &mut failed);
    let all = runner.read_segments(0, 7).expect("bulk read");
    let secs = all.first().map_or(0.0, |s| s.transfer.as_secs_f64());
    let bytes: usize = all.iter().map(BlockData::bytes).sum();
    println!(
        "bulk read: {} segments in {:.1} ms ({:.1} MB/s)",
        all.len(),
        secs * 1e3,
        bytes as f64 / secs.max(1e-9) / 1e6
    );
    check(
        "each segment holds every port and no channel",
        all.len() == 8
            && all
                .iter()
                .all(|s| s.channels.is_empty() && ports_whole(s, &ports, samples)),
        &mut failed,
    );
    if let Some(first) = all.first() {
        lines(&first.digital);
    }
    for segment in &capture.segments {
        match segment.since_first_s {
            Some(t) => print!(
                " {}{:.1}µs",
                if segment.estimated { "~" } else { "" },
                t * 1e6
            ),
            None => print!(" -"),
        }
    }
    println!();
    drop(runner.into_device());
    if failed {
        exit(1);
    }
}
