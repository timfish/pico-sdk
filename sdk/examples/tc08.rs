//! Logs temperatures from a USB TC-08 thermocouple data logger.
//!
//! ```text
//! cargo run --example tc08
//! ```

use console::Term;
use pico_sdk::prelude::*;
use std::sync::Arc;

fn get_level() -> tracing::Level {
    if std::env::args().any(|a| a.contains("--trace")) {
        tracing::Level::TRACE
    } else if std::env::args().any(|a| a.contains("--debug")) {
        tracing::Level::DEBUG
    } else if std::env::args().any(|a| a.contains("--info")) {
        tracing::Level::INFO
    } else {
        tracing::Level::WARN
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(get_level())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .init();

    // The TC-08 binary may not be published in the current driver bundle yet, in which case fall
    // back to whatever is already installed on the system.
    if let Err(error) = download_drivers_to_cache(&[Driver::TC08]) {
        println!("Could not download the TC-08 driver ({error}), trying the installed one");
    }

    let driver = Driver::TC08.load(&cache_resolution())?;

    let device = match driver.open_device(None)? {
        PicoDevice::TC08(device) => device,
        other => unreachable!("the TC-08 driver returned {:?}", other.get_serial()),
    };

    println!("Opened TC-08 {}", device.serial);

    let device = device.into_streaming_device();

    struct PrintData;

    impl EventHandler<TC08StreamingEvent> for PrintData {
        fn new_data(&self, event: &TC08StreamingEvent) {
            for (channel, values) in &event.channels {
                if let Some(latest) = values.last() {
                    println!("Channel {channel}: {latest:.2} °C");
                }
            }
        }
    }

    let callback: Arc<dyn EventHandler<_>> = Arc::new(PrintData);
    device.events.subscribe(&callback);

    let mut config = TC08Config {
        interval_ms: 100,
        mains_rejection: MainsRejectionFreq::_50Hz,
        ..Default::default()
    };
    config.enable_channel(TC08Channel::CHANNEL_CJC, TCType::K);
    config.enable_channel(TC08Channel::CHANNEL_1, TCType::K);

    device.start(config)?;

    println!("Press Enter to stop logging");
    Term::stdout().read_line()?;

    Ok(())
}
