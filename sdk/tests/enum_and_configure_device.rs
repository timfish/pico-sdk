use pico_download::{cache_resolution, download_drivers_to_cache};
use pico_enumeration::{DeviceEnumerator, EnumResultHelpers};

#[test]
#[ignore]
fn enum_and_configure_device() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .init();

    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());

    let mut results = enumerator.enumerate();

    let missing_drivers = results.missing_drivers();

    if !missing_drivers.is_empty() {
        download_drivers_to_cache(&missing_drivers).unwrap();
        results = enumerator.enumerate();
    }

    assert!(!results.is_empty(), "No devices were found");

    for result in results {
        let enum_result = result.expect("Enumeration should not error");
        let _device = enum_result.open().unwrap();
    }
}
