use console::Term;
use pico_sdk::prelude::*;
use std::sync::Arc;

fn get_level() -> tracing::Level {
    if std::env::args().any(|a| a.contains("--info")) {
        tracing::Level::INFO
    } else if std::env::args().any(|a| a.contains("--debug")) {
        tracing::Level::DEBUG
    } else if std::env::args().any(|a| a.contains("--trace")) {
        tracing::Level::TRACE
    } else {
        tracing::Level::WARN
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(get_level())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .init();

    download_drivers_to_cache(&[Driver::TC08])?;

    let driver = Driver::TC08.load(&cache_resolution())?;
    let device = match driver.open_device(None)? {
        PicoDevice::TC08(device) => device,
        _ => unreachable!(),
    };

    let device = device.into_streaming_device();

    struct PrintData;

    impl EventHandler<TC08StreamingEvent> for PrintData {
        fn new_data(&self, event: &TC08StreamingEvent) {
            println!("{:#?}", event);
        }
    }

    let callback: Arc<dyn EventHandler<_>> = Arc::new(PrintData);

    device.events.subscribe(&callback);

    device.start(TC08Config {
        interval_ms: 100,
        cold_junction: true,
        channel_1: Some(TCType::K),
        ..Default::default()
    })?;

    Term::stdout().read_line().unwrap();

    Ok(())
}
