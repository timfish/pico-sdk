use pico_common::{Driver, PicoError};
use pico_driver::oscilloscope::DriverLoadError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum EnumerationError {
    #[error("Pico driver error: {error}")]
    DriverError {
        driver: Driver,
        #[source]
        error: PicoError,
    },

    #[error("The {driver} driver could not find any devices. The Pico Technology kernel driver appears to be missing.")]
    KernelDriverError { driver: Driver },

    #[error("The {driver} driver could not be found or failed to load")]
    DriverLoadError { driver: Driver, error: String },

    #[error("Invalid Driver Version: Requires >= {required}, Found: {found}")]
    VersionError {
        driver: Driver,
        found: String,
        required: String,
    },
}

impl EnumerationError {
    pub fn from(driver: Driver, error: DriverLoadError) -> Self {
        match error {
            DriverLoadError::DriverError(error) => EnumerationError::DriverError { driver, error },
            DriverLoadError::LibloadingError(error) => EnumerationError::DriverLoadError {
                driver,
                error: error.to_string(),
            },
            DriverLoadError::VersionError { found, required } => EnumerationError::VersionError {
                driver,
                found,
                required,
            },
        }
    }
}
