use pico_driver::tc08::{TC08Channel, TC08Driver, TCType};
use pico_sdk::prelude::*;

fn main() {
    download_drivers_to_cache(&[Driver::TC08]).unwrap();
    let path = cache_resolution().get_path(Driver::TC08);

    let driver = TC08Driver::new(path).unwrap();

    let version = driver.get_driver_version().unwrap();

    println!("Driver version: {}", version);

    let handle = driver.open_unit().unwrap();

    if let Some(handle) = handle {
        let info = driver.get_unit_info(handle).unwrap();

        println!("{:?}", info);

        driver
            .configure_channel(handle, TC08Channel::CHANNEL_CJC, Some(TCType::K))
            .unwrap();

        driver.start(handle, 100).unwrap();

        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));

            let (values, _) = driver
                .get_values(handle, TC08Channel::CHANNEL_CJC, 100)
                .unwrap();

            if values.len() > 0 {
                println!("{}", values[0]);
                return;
            }
        }
    }
}
