use pico_driver::tc08::{TC08Channel, TC08Driver, TCType};
use pico_sdk::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    download_drivers_to_cache(&[Driver::TC08])?;

    let path = cache_resolution().get_path(Driver::TC08);
    let driver = TC08Driver::new(path)?;
    let version = driver.get_driver_version()?;

    println!("Driver version: {}", version);

    if let Some(handle) = driver.open_unit()? {
        println!("{:?}", driver.get_unit_info(handle)?);

        driver.configure_channel(handle, TC08Channel::CHANNEL_CJC, Some(TCType::K))?;

        driver.start(handle, 100)?;

        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));

            let (values, _) = driver.get_values(handle, TC08Channel::CHANNEL_CJC, 100)?;

            if !values.is_empty() {
                println!("{}", values[0]);
                break;
            }
        }
    }

    Ok(())
}
