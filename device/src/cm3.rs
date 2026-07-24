use crate::DeviceOpen;
use pico_common::PicoResult;
use pico_driver::cm3::PLCM3Driver;

/// Configuration for a PicoLog CM3 device
///
/// Stub: channel configuration lands with the PLCM3 driver implementation.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PLCM3Config {}

#[derive(Clone, Debug)]
pub struct PLCM3Device {
    pub driver: PLCM3Driver,
    pub serial: String,
}

impl DeviceOpen<PLCM3Device> for PLCM3Driver {
    fn open_device(&self, _serial: Option<&str>) -> PicoResult<PLCM3Device> {
        todo!("PLCM3Driver::open_device")
    }
}
