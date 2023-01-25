use std::sync::Arc;

use console::Term;
use pico_driver::tc08::{TC08Driver, TCType};
use pico_sdk::prelude::*;
use pico_streaming::tc08::{TC08Config, TC08Device, TC08StreamingEvent};

struct PrintData;

impl EventHandler<TC08StreamingEvent> for PrintData {
    #[tracing::instrument(level = "trace", skip(self, event))]
    fn handle_event(&self, event: &TC08StreamingEvent) {
        println!("{:?}", event);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a.contains("--trace")) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
            .init();
    }

    download_drivers_to_cache(&[Driver::TC08])?;

    let path = cache_resolution().get_path(Driver::TC08);
    let driver = TC08Driver::new(path)?;

    let device = TC08Device::try_open(driver, None)?;

    let callback = Arc::new(PrintData);

    device.new_data.subscribe(callback.clone());

    device.start(TC08Config {
        sample_interval_ms: 100,
        cold_junction: true,
        channel_1: Some(TCType::K),
        ..Default::default()
    })?;

    Term::stdout().read_line().unwrap();

    Ok(())
}
