//! Block mode on channel A: arms a block at a high rate, reads a min and
//! max overview of it, then raw spans, and arms again. No trigger is
//! set, so each block starts at once.
//!
//! `cargo run --release --example block_capture -- [rate] [samples] [blocks]`
//!
//! Defaults: 100 MS/s, 1 M samples, 10 blocks. `PICO_SERIAL` picks the
//! scope. It prints the timebase the unit took, each block's wait and
//! overview time, the blocks a second, the time and speed of a whole
//! raw read, and checks that:
//! - the overview has the columns asked for, and
//! - every raw sample of a span lies within its overview column's low
//!   and high.
//!
//! Each check prints `ok` or `FAIL`; the program exits 1 on a FAIL.

use pico_sdk::prelude::*;
use std::{env, ops::ControlFlow, process::exit, time::Duration, time::Instant};

const COLUMNS: usize = 2500;

fn i16s(samples: &Samples) -> &[i16] {
    match samples {
        Samples::I16(v) => v,
        _ => panic!("block reads give i16"),
    }
}

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
    let rate: f64 = args.next().map_or(100e6, |a| a.parse().unwrap());
    let samples: u64 = args.next().map_or(1_000_000, |a| a.parse().unwrap());
    let blocks: u64 = args.next().map_or(10, |a| a.parse().unwrap());

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
    settings.pre_trigger = samples / 10;
    settings.post_trigger = samples - settings.pre_trigger;
    settings.overview_columns = COLUMNS;

    let mut runner = match BlockRunner::new(device, &config, settings) {
        Ok(runner) => runner,
        Err(error) => {
            println!("refused: {error}");
            exit(1);
        }
    };
    let plan = *runner.plan();
    println!(
        "asked {rate:.0} S/s, timebase {} at {:.3} ns ({:.0} S/s), {} samples, unit holds {}",
        plan.timebase.index,
        plan.timebase.interval_s * 1e9,
        plan.timebase.samples_per_second(),
        samples,
        plan.timebase.max_samples,
    );

    let mut failed = false;
    let started = Instant::now();
    let mut count = 0;
    runner
        .run(Duration::from_secs(10), |runner, capture| {
            count += 1;
            let overview = &capture.overview[0];
            let ov_ms: f64 = overview.transfer.as_secs_f64() * 1e3;
            println!(
                "block {}: waited {:.1} ms (driver said {:.1} ms), overview {} columns of ratio {} in {ov_ms:.1} ms",
                capture.sequence,
                capture.waited.as_secs_f64() * 1e3,
                capture.indisposed_ms,
                overview.len(),
                overview.ratio,
            );
            if capture.sequence == 0 {
                check_first(runner, &capture, &mut failed)?;
            }
            Ok(if count >= blocks {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            })
        })
        .unwrap_or_else(|e| {
            println!("capture failed: {e}");
            exit(1);
        });
    let elapsed = started.elapsed().as_secs_f64();
    println!(
        "{count} blocks in {elapsed:.2} s: {:.2} blocks/s with overviews and the first block's reads",
        count as f64 / elapsed
    );
    if failed {
        exit(1);
    }
}

fn check_first(
    runner: &mut BlockRunner,
    capture: &BlockCapture,
    failed: &mut bool,
) -> Result<(), PicoError> {
    let overview = &capture.overview[0];
    let expected = (capture.samples as usize).div_ceil(overview.ratio as usize);
    check("overview columns", overview.len() == expected, failed);

    let a = &overview.channels[&PicoChannel::A];
    let (high, low) = (i16s(&a.values), i16s(a.min.as_ref().unwrap()));
    let volts = |v: i16| v as f64 * a.multiplier;
    println!(
        "overview of A: {:.3} V to {:.3} V",
        volts(*low.iter().min().unwrap_or(&0)),
        volts(*high.iter().max().unwrap_or(&0)),
    );

    let start = capture.samples / 2;
    let span = runner.read(0, start, 1000, None)?;
    let raw = i16s(&span.channels[&PicoChannel::A].values);
    let ratio = overview.ratio as u64;
    let within = raw.iter().enumerate().all(|(i, v)| {
        let column = ((start + i as u64) / ratio) as usize;
        low[column] <= *v && *v <= high[column]
    });
    check("raw span within the overview", within, failed);
    check("raw span length", raw.len() == 1000, failed);

    let whole = runner.read(0, 0, capture.samples, None)?;
    let secs = whole.transfer.as_secs_f64();
    println!(
        "whole raw read: {} samples, {:.1} ms, {:.1} MB/s",
        whole.len(),
        secs * 1e3,
        whole.bytes() as f64 / secs / 1e6,
    );
    Ok(())
}
