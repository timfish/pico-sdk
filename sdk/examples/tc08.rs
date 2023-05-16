use console::Term;
use pico_sdk::prelude::*;
use std::sync::Arc;

struct PrintData;

impl EventHandler<TC08StreamingEvent> for PrintData {
    fn new_data(&self, event: &TC08StreamingEvent) {
        println!("{:#?}", event);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a.contains("--debug")) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
            .init();
    }

    if std::env::args().any(|a| a.contains("--trace")) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
            .init();
    }

    download_drivers_to_cache(&[Driver::TC08])?;

    let driver = Arc::new(TC08Driver::load(&cache_resolution())?);
    let device = TC08Device::open(&driver, None)?;
    let device = device.into_streaming_device();

    let callback: Arc<dyn EventHandler<TC08StreamingEvent>> = Arc::new(PrintData);

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
