//! Rapid block on channel A: one run fills N memory segments, then
//! reads them all in one call and prints each segment's timing. No
//! trigger is set, so each segment starts as soon as the unit has
//! rearmed, and the spacing of segments shows the rearm time.
//!
//! `cargo run --release --example rapid_block -- [segments] [rate] [samples]`
//!
//! Defaults: 32 segments, 10 MS/s, 1000 samples. `PICO_SERIAL` picks
//! the scope. It prints for each segment its time from the first (from
//! the unit's counter, or `~` when estimated by the host clock), the
//! trigger time offset, missed triggers and counter resets, then the
//! bulk read's time and speed, and checks that:
//! - every segment was captured,
//! - the bulk read gave each segment's samples, and
//! - on a unit with a counter (5000A/B/D, 6000E, 3000E, 5000E, the
//!   3207A/B) segments are at least one segment's length apart.
//!
//! Each check prints `ok` or `FAIL`; the program exits 1 on a FAIL.

use pico_sdk::prelude::*;
use std::{env, process::exit, time::Duration};

fn check(name: &str, ok: bool, failed: &mut bool) {
    println!("check {name}: {}", if ok { "ok" } else { "FAIL" });
    *failed |= !ok;
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let mut args = env::args().skip(1);
    let segments: u64 = args.next().map_or(32, |a| a.parse().unwrap());
    let rate: f64 = args.next().map_or(10e6, |a| a.parse().unwrap());
    let samples: u64 = args.next().map_or(1000, |a| a.parse().unwrap());

    let wanted = env::var("PICO_SERIAL").ok();
    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());
    let (devices, _) = enumerator.enumerate_oscilloscopes().devices_and_errors();
    let mut device = devices
        .into_iter()
        .find(|d| wanted.as_deref().is_none_or(|w| d.serial == w))
        .expect("no scope with that serial");
    device.ensure_open().expect("open");
    println!("scope {} {}", device.variant, device.serial);

    let mut config = OscilloscopeConfig::default();
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_5V, PicoCoupling::DC);
    let mut settings = BlockSettings::new(rate, samples);
    settings.segments = segments;
    settings.overview_columns = 0;

    let mut runner = match BlockRunner::new(device, &config, settings) {
        Ok(runner) => runner,
        Err(error) => {
            println!("refused: {error}");
            exit(1);
        }
    };
    let plan = *runner.plan();
    let interval = plan.timebase.interval_s;
    println!(
        "{segments} segments of {samples} at {:.0} S/s (timebase {}), {:?} samples a segment, counter {:?}",
        plan.timebase.samples_per_second(),
        plan.timebase.index,
        plan.segment_samples,
        plan.timestamps,
    );

    let capture = runner.capture(Duration::from_secs(10)).unwrap_or_else(|e| {
        println!("capture failed: {e}");
        exit(1);
    });
    println!(
        "run took {:.2} ms from arm to ready",
        capture.waited.as_secs_f64() * 1e3
    );
    let mut previous: Option<f64> = None;
    let mut gaps = Vec::new();
    for segment in &capture.segments {
        let t = segment.since_first_s;
        let step = t.zip(previous).map(|(t, p)| t - p);
        if let (Some(step), false) = (step, segment.estimated) {
            gaps.push(step);
        }
        println!(
            "segment {:>3}: {}{} s{}, offset {}, missed {}, reset {}",
            segment.segment,
            if segment.estimated { "~" } else { "" },
            t.map_or("-".to_string(), |t| format!("{t:.9}")),
            step.map_or(String::new(), |s| format!(" (+{:.3} us)", s * 1e6)),
            segment
                .trigger
                .offset_s
                .map_or("-".to_string(), |o| format!("{:.3} ns", o * 1e9)),
            segment
                .trigger
                .missed_triggers
                .map_or("-".to_string(), |m| m.to_string()),
            segment.trigger.timestamp_reset,
        );
        previous = t.or(previous);
    }

    let mut failed = false;
    check(
        "all segments captured",
        capture.segments.len() as u64 == segments,
        &mut failed,
    );
    let n = capture.segments.len() as u64;
    match runner.read_segments(0, n - 1) {
        Ok(data) => {
            let secs = data[0].transfer.as_secs_f64();
            let bytes: usize = data.iter().map(BlockData::bytes).sum();
            println!(
                "bulk read of {n} segments: {:.2} ms, {:.1} MB/s",
                secs * 1e3,
                bytes as f64 / secs / 1e6
            );
            check(
                "each segment read whole",
                data.iter().all(|d| d.len() as u64 == samples),
                &mut failed,
            );
        }
        Err(e) => {
            println!("bulk read failed: {e}");
            failed = true;
        }
    }
    if plan.timestamps.is_some() {
        let length = samples as f64 * interval;
        if let Some(least) = gaps.iter().copied().reduce(f64::min) {
            println!(
                "least spacing {:.3} us, segment length {:.3} us, rearm about {:.3} us",
                least * 1e6,
                length * 1e6,
                (least - length) * 1e6
            );
        }
        check(
            "segments at least a segment apart",
            gaps.iter().all(|g| *g >= length * 0.999),
            &mut failed,
        );
    }
    if failed {
        exit(1);
    }
}
