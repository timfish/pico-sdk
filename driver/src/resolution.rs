use pico_common::Driver;
use std::{env::current_exe, path::PathBuf};

/// Instructs the loader where to load drivers from
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resolution {
    /// Searches for drivers using the OS default path resolution
    Default,
    /// Searches for drivers in the application root directory
    AppRoot,
    /// Searches for drivers at a specific path
    Custom(PathBuf),
}

impl Resolution {
    /// Get the expected path for a driver for this resolution
    pub fn get_path(&self, driver: Driver) -> PathBuf {
        let binary_name = driver.get_binary_name();

        match self {
            Resolution::Default => PathBuf::from(binary_name),
            Resolution::AppRoot => current_exe()
                .expect("current_exe path could not be found")
                .parent()
                .expect("current_exe path does not have parent")
                .join(binary_name),
            Resolution::Custom(path) => path.join(binary_name),
        }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::Default
    }
}
