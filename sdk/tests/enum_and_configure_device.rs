use pico_download::{cache_resolution, download_drivers_to_cache};
use pico_enumeration::{DeviceEnumerator, EnumResultHelpers};

#[test]
#[ignore]
fn enum_and_configure_device() {
    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());

    let mut results = enumerator.enumerate();

    let missing_drivers = results.missing_drivers();

    if !missing_drivers.is_empty() {
        download_drivers_to_cache(&missing_drivers).unwrap();
        results = enumerator.enumerate();
    }

    assert!(!results.is_empty(), "No devices were found");

    for result in results {
        let _device = result.expect("Enumeration should not error");
    }
}
