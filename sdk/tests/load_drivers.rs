use pico_sdk::{
    common::Driver,
    download::{available_drivers, cache_resolution, download_drivers_to_cache},
    driver::{DriverLoad, PicoDriver},
};

/// Downloads every driver Pico publish for this platform from the release the crate was built
/// against, then loads each one and asks it for its version. Needs network access and the
/// matching driver release to have been published, so it's `#[ignore]`d like the hardware tests.
///
/// Run with: `cargo test -p pico-sdk --test load_drivers -- --ignored --nocapture`
#[test]
#[ignore]
fn download_and_load_drivers() {
    let resolution = cache_resolution();

    for driver in available_drivers() {
        // PicoIPP is a shared library ps4000/ps6000 load internally, not a driver we load directly
        if driver == Driver::PicoIPP {
            continue;
        }

        if driver.load(&resolution).is_err() {
            download_drivers_to_cache(&[driver]).unwrap();
        }

        match driver.load(&resolution).unwrap() {
            PicoDriver::Oscilloscope(loaded) => assert!(
                loaded.get_version().is_ok(),
                "{driver} loaded but has no version"
            ),
            // The TC-08 reports a version per unit rather than per driver, so loading the
            // library is all we can check without hardware attached.
            PicoDriver::TC08(_) => {}
        }
    }
}
