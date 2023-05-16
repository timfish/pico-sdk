use pico_sdk::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .init();

    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());

    loop {
        println!("Enumerating Pico devices...");
        let results = enumerator.enumerate();

        println!("{:#?}", results);

        let missing_drivers = results.missing_drivers();

        if !missing_drivers.is_empty() {
            println!(
                "Downloading drivers that failed to load {:?}",
                &missing_drivers
            );
            download_drivers_to_cache(&missing_drivers)?;
            println!("Downloads complete");
        } else {
            return Ok(());
        }
    }
}
