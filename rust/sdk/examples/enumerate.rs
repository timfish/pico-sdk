use pico_sdk::{
    download::{cache_resolution, download_drivers_to_cache},
    enumeration::{DeviceEnumerator, EnumResultHelpers},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
