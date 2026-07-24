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
    /// Like the TC-08, the DrDAQ has no enumerate call and reports no serial until a unit is open,
    /// so discovery means opening every connected unit in turn.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<DrDAQDevice>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(DrDAQDevice::new_closed(self, info.serial.clone(), Some(info)))
            })
            .collect())
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
    /// Like the TC-08, the HRDL has no enumerate call and reports no serial until a unit is open,
    /// so discovery means opening every connected unit in turn.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<HRDLDevice>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(HRDLDevice::new_closed(self, info.serial.clone(), Some(info)))
            })
            .collect())
    }
}

impl EnumerateDriver<PL1000Device> for PL1000Driver {
    /// Like the TC-08, the PL1000 has no enumerate call and reports no serial until a unit is
    /// open, so discovery means opening every connected unit in turn.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PL1000Device>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(PL1000Device::new_closed(self, info.serial.clone(), Some(info)))
            })
            .collect())
    }
}

impl EnumerateDriver<PLCM3Device> for PLCM3Driver {
    /// The CM3 opens by serial filter directly, but a null-serial open still opens the next
    /// unopened unit, so discovery uses the same open-every-unit loop as the other loggers.
    /// (See [`pico_driver::cm3::PLCM3DriverInternal::open_unit_iter`]'s note on the assumed
    /// repeated-null-open semantics -- to confirm on hardware.)
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PLCM3Device>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(PLCM3Device::new_closed(self, info.serial.clone(), Some(info)))
            })
            .collect())
    }
}

impl EnumerateDriver<PT104Device> for PT104Driver {
    /// Like the TC-08, the PT-104 (over USB) has no enumerate call and reports no serial until a
    /// unit is open, so discovery means opening every connected unit in turn.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PT104Device>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(PT104Device::new_closed(self, info.serial.clone(), Some(info)))
            })
            .collect())
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
