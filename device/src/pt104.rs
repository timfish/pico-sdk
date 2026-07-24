use crate::DeviceOpen;
use pico_common::PicoResult;
use pico_driver::pt104::PT104Driver;

/// Configuration for a USB PT-104 device
///
/// Stub: channel configuration lands with the PT-104 driver implementation. USB only for now -
/// see `pico-high-level-drivers-plan.md` (daq-db repo) for why Ethernet is out of scope.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PT104Config {}

#[derive(Clone, Debug)]
pub struct PT104Device {
    pub driver: PT104Driver,
    pub serial: String,
}

impl DeviceOpen<PT104Device> for PT104Driver {
    fn open_device(&self, _serial: Option<&str>) -> PicoResult<PT104Device> {
        todo!("PT104Driver::open_device")
    }
}
