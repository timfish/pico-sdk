use pico_common::Driver;
use pico_download::{cache_resolution, download_drivers_to_cache};
use pico_driver::DriverLoader;

#[test]
fn load_all_drivers() {
    let drivers = [
        Driver::PS2000,
        Driver::PS2000A,
        Driver::PS3000A,
        Driver::PS4000,
        Driver::PS4000A,
        Driver::PS5000A,
        Driver::PS6000,
        Driver::TC08,
    ];

    let cache_resolution = cache_resolution();

    drivers.into_iter().for_each(|driver_type| {
        let mut loaded = driver_type.load(&cache_resolution);

        if loaded.is_err() {
            download_drivers_to_cache(&[driver_type]).unwrap();
            loaded = driver_type.load(&cache_resolution);
        }

        assert!(loaded.is_ok());
        let loaded = loaded.unwrap();
        match loaded {
            pico_driver::PicoDriver::Oscilloscope(loaded) => {
                assert!(loaded.get_version().is_ok());
            }
            pico_driver::PicoDriver::TC08(loaded) => {
                assert!(loaded.get_unit_info(0).is_ok());
            }
        }
    });
}
