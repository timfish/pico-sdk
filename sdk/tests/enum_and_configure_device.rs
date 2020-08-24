use pico_common::{PicoChannel, PicoCoupling, PicoRange};
use pico_download::{cache_resolution, download_drivers_to_cache};
use pico_enumeration::{DeviceEnumerator, EnumResultHelpers};
use rayon::prelude::*;

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

    results
        .into_par_iter()
        .map(|d| d.expect("Enumeration should not error"))
        .for_each(|d| {
            d.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_1V, PicoCoupling::DC);
            d.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::AC);
        });
}
