use crate::DeviceOpen;
use pico_common::PicoResult;
use pico_driver::drdaq::DrDAQDriver;

/// Configuration for a USB DrDAQ device
///
/// Stub: channel configuration lands with the DrDAQ driver implementation.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct DrDAQConfig {}

#[derive(Clone, Debug)]
pub struct DrDAQDevice {
    pub driver: DrDAQDriver,
    pub serial: String,
}

impl DeviceOpen<DrDAQDevice> for DrDAQDriver {
    fn open_device(&self, _serial: Option<&str>) -> PicoResult<DrDAQDevice> {
        todo!("DrDAQDriver::open_device")
    }
}
