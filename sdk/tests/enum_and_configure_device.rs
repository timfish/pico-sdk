use pico_device::PicoDevice;
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
        let mut device = result.expect("Enumeration should not error");

        assert!(
            !device.get_serial().is_empty(),
            "Enumerated device has no serial"
        );

        // Oscilloscopes enumerate closed, so opening is a separate step. The TC-08 has to be
        // opened to be discovered at all and so is already open.
        if let PicoDevice::Oscilloscope(scope) = &mut device {
            scope.ensure_open().unwrap();
            assert!(scope.info.is_some(), "Opened scope reported no info");
        }
    }
}
