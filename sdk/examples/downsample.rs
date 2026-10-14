//! Streams channel A with a downsample: the driver joins each `ratio`
//! samples into one, so the stream delivers the device's rate divided
//! by `ratio`. Prints the rates, the sample type and the mean of each
//! second.
//!
//! `cargo run --release --example downsample -- [rate] [ratio] [average|sum]`
//!
//! `PICO_SERIAL` picks the scope. Sum needs a ps6000a or psospa scope.

use pico_sdk::prelude::*;
use std::{
    env,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Default)]
struct Mean {
    /// Sum of the scaled values and their count.
    total: Mutex<(f64, usize, Option<SampleType>)>,
}

impl EventHandler<OscilloscopeStreamEvent> for Mean {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        let Some(block) = event.channels.get(&PicoChannel::A) else {
            return;
        };
        let mut total = self.total.lock().unwrap();
        total.0 += block.scale_samples().iter().sum::<f64>();
        total.1 += block.samples.len();
        total.2 = Some(block.samples.sample_type());
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let mut args = env::args().skip(1);
    let rate: u32 = args.next().map_or(1_000_000, |a| a.parse().unwrap());
    let ratio: u32 = args.next().map_or(16, |a| a.parse().unwrap());
    let mode = match args.next().as_deref() {
        None | Some("average") => DownsampleKind::Average,
        Some("sum") => DownsampleKind::Sum,
        Some(other) => panic!("unknown mode {other}: use average or sum"),
    };

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
    config.set_sample_rate(rate);
    config.set_downsample(Some(Downsample { ratio, mode }));
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_5V, PicoCoupling::DC);

    let runner = device.into_streaming_device();
    let mean = Arc::new(Mean::default());
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = mean.clone();
    runner.events.subscribe(&handler);
    if let Err(error) = runner.start(config) {
        println!("refused: {error}");
        return;
    }
    let stream = runner.get_stream().expect("streaming");
    println!(
        "device {:.0} S/s, delivered {:.0} S/s ({:?}), multiplier {:e}",
        stream.device_samples_per_second(),
        stream.samples_per_second(),
        stream.sample_type(),
        stream.multiplier(PicoChannel::A).unwrap_or(f64::NAN),
    );

    for _ in 0..5 {
        thread::sleep(Duration::from_secs(1));
        let (sum, count, sample_type) = std::mem::take(&mut *mean.total.lock().unwrap());
        println!(
            "{count} samples of {sample_type:?}, mean {:.4} V",
            sum / count.max(1) as f64
        );
    }
    runner.stop();
    runner.close();
}
