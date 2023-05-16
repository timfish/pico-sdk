use std::fmt;

use super::EnumerationError;
use pico_common::Driver;

pub trait EnumResultHelpers<D> {
    fn missing_drivers(&self) -> Vec<Driver>;
    fn devices_and_errors(self) -> (Vec<D>, Vec<EnumerationError>);
}

impl<D> EnumResultHelpers<D> for Vec<Result<D, EnumerationError>>
where
    D: fmt::Debug,
{
    fn missing_drivers(&self) -> Vec<Driver> {
        let mut failed_results = self
            .iter()
            .flat_map(|r| match r {
                Err(EnumerationError::DriverLoadError { driver, .. }) => Some(*driver),
                Err(EnumerationError::VersionError { driver, .. }) => Some(*driver),
                _ => None,
            })
            .collect::<Vec<Driver>>();

        failed_results.sort();
        failed_results.dedup();
        failed_results
    }

    fn devices_and_errors(self) -> (Vec<D>, Vec<EnumerationError>) {
        let (devices, errors): (Vec<_>, Vec<_>) = self.into_iter().partition(|e| e.is_ok());

        (
            devices.into_iter().map(|r| r.unwrap()).collect(),
            errors.into_iter().map(|r| r.unwrap_err()).collect(),
        )
    }
}
