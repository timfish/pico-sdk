use pico_common::PicoResult;
use pico_device::{
    cm3::PLCM3Device, drdaq::DrDAQDevice, hrdl::HRDLDevice, oscilloscope::OscilloscopeDevice,
    pl1000::PL1000Device, pt104::PT104Device, tc08::TC08Device, PicoDevice,
};
use pico_driver::{
    cm3::PLCM3Driver, drdaq::DrDAQDriver, hrdl::HRDLDriver, oscilloscope::OscilloscopeDriver,
    pl1000::PL1000Driver, pt104::PT104Driver, tc08::TC08Driver, PicoDriver,
};

/// Lists the devices a loaded driver can see
///
/// Implemented per family because discovery works differently for each, then for [`PicoDriver`]
/// to dispatch to the right one.
pub trait EnumerateDriver<D> {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<D>>>;
}

impl EnumerateDriver<DrDAQDevice> for DrDAQDriver {
    /// Stub: real enumeration/open behavior lands with the DrDAQ driver implementation.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<DrDAQDevice>>> {
        todo!("DrDAQDriver::enumerate_devices")
    }
}

impl EnumerateDriver<OscilloscopeDevice> for OscilloscopeDriver {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<OscilloscopeDevice>>> {
        Ok(self
            .enumerate_units()?
            .into_iter()
            .map(|unit| {
                Ok(OscilloscopeDevice::new_closed(
                    self,
                    unit.serial,
                    unit.variant,
                ))
            })
            .collect())
    }
}

impl EnumerateDriver<HRDLDevice> for HRDLDriver {
    /// Stub: real enumeration/open behavior lands with the PicoHRDL driver implementation.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<HRDLDevice>>> {
        todo!("HRDLDriver::enumerate_devices")
    }
}

impl EnumerateDriver<PL1000Device> for PL1000Driver {
    /// Stub: real enumeration/open behavior lands with the PL1000 driver implementation.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PL1000Device>>> {
        todo!("PL1000Driver::enumerate_devices")
    }
}

impl EnumerateDriver<PLCM3Device> for PLCM3Driver {
    /// Stub: real enumeration/open behavior lands with the PLCM3 driver implementation.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PLCM3Device>>> {
        todo!("PLCM3Driver::enumerate_devices")
    }
}

impl EnumerateDriver<PT104Device> for PT104Driver {
    /// Stub: real enumeration/open behavior lands with the PT-104 driver implementation.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PT104Device>>> {
        todo!("PT104Driver::enumerate_devices")
    }
}

impl EnumerateDriver<TC08Device> for TC08Driver {
    /// The TC-08 has no enumerate call and will not report a serial number until a unit is open,
    /// so discovery means opening every connected unit. The devices this returns are therefore
    /// already open, unlike the oscilloscope ones.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<TC08Device>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;

                Ok(TC08Device::new_closed(
                    self,
                    info.serial.clone(),
                    Some(info),
                ))
            })
            .collect())
    }
}

impl EnumerateDriver<PicoDevice> for PicoDriver {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PicoDevice>>> {
        Ok(match self {
            PicoDriver::DrDAQ(driver) => driver
                .enumerate_devices()?
                .into_iter()
                .map(|d| d.map(PicoDevice::DrDAQ))
                .collect(),
            PicoDriver::Oscilloscope(driver) => driver
                .enumerate_devices()?
                .into_iter()
                .map(|d| d.map(PicoDevice::Oscilloscope))
                .collect(),
            PicoDriver::PicoHRDL(driver) => driver
                .enumerate_devices()?
                .into_iter()
                .map(|d| d.map(PicoDevice::PicoHRDL))
                .collect(),
            PicoDriver::PL1000(driver) => driver
                .enumerate_devices()?
                .into_iter()
                .map(|d| d.map(PicoDevice::PL1000))
                .collect(),
            PicoDriver::PLCM3(driver) => driver
                .enumerate_devices()?
                .into_iter()
                .map(|d| d.map(PicoDevice::PLCM3))
                .collect(),
            PicoDriver::PT104(driver) => driver
                .enumerate_devices()?
                .into_iter()
                .map(|d| d.map(PicoDevice::PT104))
                .collect(),
            PicoDriver::TC08(driver) => driver
                .enumerate_devices()?
                .into_iter()
                .map(|d| d.map(PicoDevice::TC08))
                .collect(),
        })
    }
}
