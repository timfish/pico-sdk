//! Opens a scope that detects probes and prints what its driver
//! reports: each channel's probe, its ranges with their units, and the
//! range and coupling the probe is set to. Then streams every channel
//! that has a range at 1 kS/s and prints each channel's unit, scale and
//! mean, and prints each probe change while it watches, so plugging a
//! probe in, out or swapping it shows.
//!
//! `cargo run --example probes -- [driver] [serial] [seconds]`
//!
//! The driver defaults to `4000a`, the seconds to 20. Drivers come
//! from the download cache, fetched first when missing.

use pico_sdk::prelude::*;
use std::{
    collections::HashMap,
    env,
    str::FromStr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

struct PrintNotices;

impl EventHandler<DeviceNotice> for PrintNotices {
    fn new_data(&self, notice: Arc<DeviceNotice>) {
        match notice.as_ref() {
            DeviceNotice::ProbeChanged { probes, streaming } => {
                for probe in probes {
                    println!("probe changed (streaming: {streaming}):");
                    print_probe(probe);
                }
            }
            other => println!("notice: {other:?}"),
        }
    }
}

/// Sums of each channel's values since the last print.
#[derive(Default)]
struct Means(Mutex<HashMap<PicoChannel, (f64, usize, f64)>>);

impl EventHandler<OscilloscopeStreamEvent> for Means {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        let mut sums = self.0.lock().unwrap();
        for (channel, block) in &event.channels {
            let values = block.scale_samples();
            let entry = sums.entry(*channel).or_insert((0.0, 0, block.multiplier));
            entry.0 += values.iter().sum::<f64>();
            entry.1 += values.len();
            entry.2 = block.multiplier;
        }
    }
}

fn print_probe(p: &ProbeState) {
    println!(
        "  {}: {} (code {}), connected {}, enabled {}, needs power {}, powered {}, status {:?}",
        p.channel,
        p.probe,
        p.probe.code(),
        p.connected,
        p.enabled,
        p.requires_power,
        p.powered,
        p.status
    );
    println!(
        "     set to {:?} {:?}; couplings {:?}",
        p.range, p.coupling, p.couplings
    );
    if !p.unknown_ranges.is_empty() {
        println!("     range values not named: {:?}", p.unknown_ranges);
    }
    if let Some(problem) = p.problem() {
        println!("     cannot run: {problem}");
    }
}

fn print_ranges(info: &OscilloscopeInfo, channel: PicoChannel) {
    let ranges = info
        .valid_channel_ranges
        .get(&channel)
        .cloned()
        .unwrap_or_default();
    let shown: Vec<String> = ranges
        .iter()
        .map(|r| match r.full_scale() {
            Some(_) => format!("{r} [{}]", r.get_units().short),
            None => format!("{r:?} [{}, no scale]", r.get_units().short),
        })
        .collect();
    println!("  {channel} ranges: {}", shown.join(", "));
}

/// The range a channel streams at: the probe's own where this crate
/// scales it, else the widest scaled range.
fn pick_range(info: &OscilloscopeInfo, channel: PicoChannel) -> Option<PicoRange> {
    let ranges = info.valid_channel_ranges.get(&channel)?;
    let scaled: Vec<PicoRange> = ranges
        .iter()
        .copied()
        .filter(|r| r.full_scale().is_some())
        .collect();
    info.probe(channel)
        .and_then(|p| p.range)
        .filter(|r| scaled.contains(r))
        .or_else(|| {
            scaled
                .iter()
                .copied()
                .max_by(|a, b| a.full_scale().unwrap().total_cmp(&b.full_scale().unwrap()))
        })
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let mut args = env::args().skip(1);
    let driver_type = Driver::from_str(&args.next().unwrap_or_else(|| "4000a".into()))
        .expect("a driver type such as 4000a");
    let serial = args.next().filter(|s| !s.is_empty() && s != "-");
    let seconds: u64 = args.next().map_or(20, |s| s.parse().expect("seconds"));

    let resolution = cache_resolution();
    let driver = match driver_type.load(&resolution) {
        Ok(driver) => driver,
        Err(_) => {
            download_drivers_to_cache(&[driver_type]).expect("driver download");
            driver_type.load(&resolution).expect("driver load")
        }
    };
    let PicoDriver::Oscilloscope(driver) = driver else {
        panic!("{driver_type} is not a scope driver");
    };
    let device = match driver.open_device(serial.as_deref()) {
        Ok(device) => device,
        Err(e) => {
            println!("no {driver_type} unit opened: {e}");
            return;
        }
    };
    let info: OscilloscopeInfo = device.info.clone().expect("an open device has info");
    println!("{} {}", device.variant, device.serial);
    let mut channels: Vec<PicoChannel> = info.valid_channel_ranges.keys().copied().collect();
    channels.sort();
    for channel in &channels {
        match info.probes.iter().find(|p| p.channel == *channel) {
            Some(p) => print_probe(p),
            None => println!("  {channel}: no probe reported"),
        }
        print_ranges(&info, *channel);
    }

    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(1_000);
    for channel in &channels {
        if info.probe(*channel).is_some_and(|p| p.problem().is_some()) {
            continue;
        }
        if let Some(range) = pick_range(&info, *channel) {
            config.enable_channel(*channel, range, PicoCoupling::DC);
        }
    }

    let streaming = device.into_streaming_device();
    let notices: Arc<dyn EventHandler<DeviceNotice>> = Arc::new(PrintNotices);
    streaming.notices.subscribe(&notices);
    let means = Arc::new(Means::default());
    let events: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = means.clone();
    streaming.events.subscribe(&events);

    let mut on: Vec<_> = config.channels.iter().map(|(c, v)| (*c, v.range)).collect();
    on.sort();
    println!("streaming at 1 kS/s: {on:?}");
    if let Err(e) = streaming.start(config) {
        println!("stream start failed: {e}");
    }
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(seconds) {
        std::thread::sleep(Duration::from_secs(1));
        let mut sums = means.0.lock().unwrap();
        let mut line: Vec<_> = sums
            .iter()
            .map(|(c, (sum, n, m))| (*c, *sum / (*n).max(1) as f64, *n, *m))
            .collect();
        line.sort_by_key(|l| l.0);
        sums.clear();
        drop(sums);
        for (channel, mean, n, multiplier) in line {
            let unit = streaming
                .get_info()
                .and_then(|i| i.probe(channel).and_then(|p| p.range))
                .map(|r| r.get_units().short)
                .unwrap_or_else(|| "?".into());
            println!(
                "  {channel}: mean {mean:.6} over {n} samples, scale {multiplier:e} (probe unit {unit})"
            );
        }
    }
    streaming.stop();
    streaming.close();
}
