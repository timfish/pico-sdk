use crate::DeviceOpen;
use pico_common::PicoResult;
use pico_driver::pl1000::PL1000Driver;

/// Configuration for a PicoLog 1000 series device
///
/// Stub: channel/range configuration lands with the PL1000 driver implementation.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PL1000Config {}

#[derive(Clone, Debug)]
pub struct PL1000Device {
    pub driver: PL1000Driver,
    pub serial: String,
}

impl DeviceOpen<PL1000Device> for PL1000Driver {
    fn open_device(&self, _serial: Option<&str>) -> PicoResult<PL1000Device> {
        todo!("PL1000Driver::open_device")
    }
}
