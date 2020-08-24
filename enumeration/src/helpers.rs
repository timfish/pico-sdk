use crate::EnumerationError;
use pico_common::Driver;
use pico_device::PicoDevice;

pub trait EnumResultHelpers {
    fn missing_drivers(&self) -> Vec<Driver>;
}

impl EnumResultHelpers for Vec<Result<PicoDevice, EnumerationError>> {
    fn missing_drivers(&self) -> Vec<Driver> {
        let mut failed_results = self
            .iter()
            .flat_map(|r| match r {
                Err(EnumerationError::DriverLoadError { driver }) => Some(*driver),
                _ => None,
            })
            .collect::<Vec<Driver>>();

        failed_results.sort();
        failed_results.dedup();

        failed_results
    }
}
