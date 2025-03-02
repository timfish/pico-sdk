#![forbid(unsafe_code)]

//! `PicoDevice` implementation for Pico Technology oscilloscope drivers.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! When a `PicoDevice` is created, it is opened, its channels and capabilities are
//! automatically detected and then it is closed.
//!
//! # Example
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use pico_common::Driver;
//! use pico_driver::LibraryResolution;
//! use pico_device::PicoDevice;
//!
//! // Load the required driver
//! let driver = LibraryResolution::Default.try_load(Driver::PS2000)?;
//!
//! // Try and open the first available ps2000 device
//! let device1 = PicoDevice::try_open(&driver, None)?;
//!
//! // Try and open devices by serial
//! let device2 = PicoDevice::try_open(&driver, Some("ABC/123"))?;
//! let device3 = PicoDevice::try_open(&driver, Some("ABC/987"))?;
//! # Ok(())
//! # }
//! ```

use pico_config::{DeviceConfig, DeviceInfo};
use pico_driver::{PicoDriver, PicoError, StreamingResult, StreamingState};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
enum DeviceState {
    Closed,
    Open { handle: i16 },
}

#[derive(Clone, Debug)]
pub struct PicoDevice {
    driver: Arc<dyn PicoDriver>,
    pub serial: String,
    state: DeviceState,
}

impl PicoDevice {
    pub fn new_closed(driver: &Arc<dyn PicoDriver>, serial: &str) -> Self {
        Self {
            driver: driver.clone(),
            serial: serial.to_string(),
            state: DeviceState::Closed,
        }
    }

    pub fn new_open(driver: &Arc<dyn PicoDriver>, serial: Option<&str>) -> Result<Self, PicoError> {
        let result = driver.open_device(serial)?;

        Ok(Self {
            driver: driver.clone(),
            serial: result.serial,
            state: DeviceState::Open {
                handle: result.handle,
            },
        })
    }

    pub fn device_config(&self) -> Result<Option<DeviceConfig>, PicoError> {
        match self.state {
            DeviceState::Open { handle } => Ok(Some(self.driver.get_device_config(handle)?)),
            DeviceState::Closed => Ok(None),
        }
    }

    pub fn device_info(&self) -> Result<Option<DeviceInfo>, PicoError> {
        match self.state {
            DeviceState::Open { handle } => Ok(Some(self.driver.get_device_info(handle)?)),
            DeviceState::Closed => Ok(None),
        }
    }

    pub fn configure_device(&self, config: &DeviceConfig) -> Result<(), PicoError> {
        match self.state {
            DeviceState::Open { handle } => self.driver.configure_device(handle, config),
            _ => todo!(),
        }
    }

    pub fn start_streaming(&self, config: &DeviceConfig) -> Result<StreamingState, PicoError> {
        match self.state {
            DeviceState::Open { handle } => self.driver.start_streaming(handle, config),
            _ => todo!(),
        }
    }

    pub fn stop(&self) -> Result<(), PicoError> {
        match self.state {
            DeviceState::Open { handle } => self.driver.stop(handle),
            _ => todo!(),
        }
    }

    pub fn get_streaming_values(
        &self,
        state: &mut StreamingState,
    ) -> Result<StreamingResult, PicoError> {
        match self.state {
            DeviceState::Open { handle } => self.driver.get_streaming_values(handle, state),
            _ => todo!(),
        }
    }

    pub fn consume(mut self) -> (Arc<dyn PicoDriver>, String, Option<i16>) {
        let handle = match self.state {
            DeviceState::Open { handle } => Some(handle),
            DeviceState::Closed => None,
        };
        self.state = DeviceState::Closed;
        (self.driver.clone(), self.serial.clone(), handle)
    }

    pub fn driver(&self) -> &Arc<dyn PicoDriver> {
        &self.driver
    }
}

impl Drop for PicoDevice {
    fn drop(&mut self) {
        if let DeviceState::Open { handle } = self.state {
            let _ = self.driver.close_device(handle);
        }
    }
}
