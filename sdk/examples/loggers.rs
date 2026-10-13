//! Logs scaled readings from a USB PT-104, PicoLog CM3, PicoLog 1000 or ADC-20/ADC-24.
//!
//! ```text
//! cargo run --example loggers -- pt104
//! cargo run --example loggers -- cm3
//! cargo run --example loggers -- pl1000
//! cargo run --example loggers -- hrdl
//! ```
//!
//! The drivers for these loggers are not in the published driver bundle, so this loads the
//! installed PicoSDK driver.

use console::Term;
use pico_sdk::common::{
    HRDLChannel, HRDLRange, PL1000Channel, PLCM3Channel, PLCM3DataType, PT104Channel,
    PT104DataType, PT104Wires,
};
use pico_sdk::prelude::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let family = std::env::args().nth(1).unwrap_or_else(|| "pt104".to_string());
    let driver = match family.as_str() {
        "pt104" => Driver::PT104,
        "cm3" => Driver::PLCM3,
        "pl1000" => Driver::PL1000,
        "hrdl" => Driver::PicoHRDL,
        other => return Err(format!("unknown family '{other}'").into()),
    };
    let device = driver.load(&LibraryResolution::Default)?.open_device(None)?;
    println!("Opened {}", device.get_serial());

    match device {
        PicoDevice::PT104(device) => pt104(device)?,
        PicoDevice::PLCM3(device) => cm3(device)?,
        PicoDevice::PL1000(device) => pl1000(device)?,
        PicoDevice::PicoHRDL(device) => hrdl(device)?,
        _ => unreachable!("the driver opened another family"),
    }

    Ok(())
}

/// Logs until Enter; the stream stops when its runner drops.
fn wait_for_enter() -> Result<(), Box<dyn std::error::Error>> {
    println!("Press Enter to stop logging");
    Term::stdout().read_line()?;
    Ok(())
}

fn pt104(device: PT104Device) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = PT104Config::default();
    config.enable_channel(PT104Channel::CHANNEL_1, PT104DataType::PT100, PT104Wires::FourWire);
    let types: Vec<_> = config.enabled_channels().map(|(c, t, _)| (c, t)).collect();
    let device = device.into_streaming_device();

    struct Print(Vec<(PT104Channel, PT104DataType)>);
    impl EventHandler<PT104StreamingEvent> for Print {
        fn new_data(&self, event: Arc<PT104StreamingEvent>) {
            for (channel, data_type) in &self.0 {
                let raw = event.channels.get(channel).copied().flatten();
                match (raw.and_then(|raw| data_type.to_unit(raw)), data_type.scale()) {
                    (Some(value), Some((_, unit))) => println!("Channel {channel}: {value} {unit}"),
                    _ => println!("Channel {channel}: no new conversion"),
                }
            }
        }
    }
    let callback: Arc<dyn EventHandler<_>> = Arc::new(Print(types));
    device.events.subscribe(&callback);
    device.start(config)?;
    wait_for_enter()
}

fn cm3(device: PLCM3Device) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = PLCM3Config::default();
    config.enable_channel(PLCM3Channel::CHANNEL_1, PLCM3DataType::TenMillivoltsPerAmp);
    let types: Vec<_> = config.enabled_channels().collect();
    let device = device.into_streaming_device();

    struct Print(Vec<(PLCM3Channel, PLCM3DataType)>);
    impl EventHandler<PLCM3StreamingEvent> for Print {
        fn new_data(&self, event: Arc<PLCM3StreamingEvent>) {
            for (channel, data_type) in &self.0 {
                let raw = event.channels.get(channel).copied().flatten();
                match (raw.and_then(|raw| data_type.to_unit(raw)), data_type.scale()) {
                    (Some(value), Some((_, unit))) => println!("Channel {channel}: {value} {unit}"),
                    _ => println!("Channel {channel}: no new conversion"),
                }
            }
        }
    }
    let callback: Arc<dyn EventHandler<_>> = Arc::new(Print(types));
    device.events.subscribe(&callback);
    device.start(config)?;
    wait_for_enter()
}

fn pl1000(device: PL1000Device) -> Result<(), Box<dyn std::error::Error>> {
    let info = device.info.clone().ok_or("the unit is not open")?;
    println!("{} with channels 1 to {}", info.model(), info.channels().len());
    let mut config = PL1000Config {
        sample_interval_us: 100_000,
        ..Default::default()
    };
    config.enable_channel(PL1000Channel::CHANNEL_1);
    let device = device.into_streaming_device();

    struct Print(f64);
    impl EventHandler<PL1000StreamingEvent> for Print {
        fn new_data(&self, event: Arc<PL1000StreamingEvent>) {
            for (channel, counts) in &event.channels {
                if let Some(&last) = counts.last() {
                    println!("Channel {channel}: {:.4} V", last as f64 * self.0);
                }
            }
        }
    }
    let callback: Arc<dyn EventHandler<_>> = Arc::new(Print(info.volts_per_count()));
    device.events.subscribe(&callback);
    device.start(config)?;
    wait_for_enter()
}

fn hrdl(device: HRDLDevice) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HRDLConfig {
        sample_interval_ms: 1_000,
        ..Default::default()
    };
    config.enable_channel(HRDLChannel::CHANNEL_1, HRDLRange::Range2500mV, true);
    let device = device.into_streaming_device();
    device.start(config)?;
    let state = device.get_stream().ok_or("the unit is not streaming")?;
    println!("Streaming every {} ms: {:?}", state.interval_ms, state.channels);

    struct Print(HRDLStreamState);
    impl EventHandler<HRDLStreamingEvent> for Print {
        fn new_data(&self, event: Arc<HRDLStreamingEvent>) {
            for (channel, counts) in &event.channels {
                let scale = &self.0.channels[channel];
                if let Some(&last) = counts.last() {
                    println!("Channel {channel}: {:.6} V", last as f64 * scale.volts_per_count());
                }
            }
        }
    }
    let callback: Arc<dyn EventHandler<_>> = Arc::new(Print(state));
    device.events.subscribe(&callback);
    wait_for_enter()
}
