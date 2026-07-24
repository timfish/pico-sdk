use crate::DeviceOpen;
use pico_common::PicoResult;
use pico_driver::hrdl::HRDLDriver;

/// Configuration for an ADC-20/ADC-24 (PicoHRDL) device
///
/// Stub: channel/range configuration lands with the PicoHRDL driver implementation.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct HRDLConfig {}

#[derive(Clone, Debug)]
pub struct HRDLDevice {
    pub driver: HRDLDriver,
    pub serial: String,
}

impl DeviceOpen<HRDLDevice> for HRDLDriver {
    fn open_device(&self, _serial: Option<&str>) -> PicoResult<HRDLDevice> {
        todo!("HRDLDriver::open_device")
    }
}
