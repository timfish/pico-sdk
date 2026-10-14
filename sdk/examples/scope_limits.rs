//! Opens a scope and prints what it allows: its power, the channels
//! that power runs, its resolutions with any channel limits, and the
//! fastest rate for each count of channels on. Then waits a few
//! seconds for power notices, so plugging the supply in or out shows.
//!
//! `cargo run --example scope_limits -- psospa [serial] [driver dir]`

use pico_sdk::prelude::*;
use std::{env, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

struct PrintNotices;

impl EventHandler<DeviceNotice> for PrintNotices {
    fn new_data(&self, notice: Arc<DeviceNotice>) {
        println!("notice: {notice:?}");
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let driver_type = Driver::from_str(&args.next().expect("driver type, such as psospa"))
        .expect("a driver type such as 5000a or psospa");
    let serial = args.next();
    let resolution = args.next().map_or(LibraryResolution::Default, |dir| {
        LibraryResolution::Custom(PathBuf::from(dir))
    });

    let driver = driver_type
        .load(&resolution)
        .unwrap_or_else(|e| panic!("could not load the {driver_type} driver: {e}"));
    let PicoDriver::Oscilloscope(driver) = driver else {
        panic!("{driver_type} is not a scope driver");
    };
    let device = driver
        .open_device(serial.as_deref())
        .unwrap_or_else(|e| panic!("could not open a {driver_type} unit: {e}"));
    let info: OscilloscopeInfo = device.info.clone().expect("an open device has info");

    println!(
        "{} {} on USB {}",
        device.variant, device.serial, info.usb_version
    );
    match info.power {
        Some(power) => println!("power: {power}"),
        None => println!("power: no choice of power"),
    }
    println!("channels on this power: {:?}", info.available_channels());
    if !info.supply_channels.is_empty() {
        println!("channels only on the supply: {:?}", info.supply_channels);
    }
    for (resolution, max) in &info.resolutions {
        let most = info
            .channel_limits
            .iter()
            .find(|(r, _)| r == resolution)
            .map_or("every".to_string(), |(_, n)| n.to_string());
        println!("{resolution}: max ADC {max}, {most} channels");
    }
    for limit in &info.rate_limits {
        let resolution = limit
            .resolution
            .map_or("fixed".to_string(), |r| r.to_string());
        println!(
            "{resolution}, {} channels: {:.4} GS/s",
            limit.channels,
            limit.max_samples_per_second / 1e9
        );
    }

    let streaming = device.into_streaming_device();
    let handler: Arc<dyn EventHandler<DeviceNotice>> = Arc::new(PrintNotices);
    streaming.notices.subscribe(&handler);
    std::thread::sleep(Duration::from_secs(3));
    streaming.close();
}
