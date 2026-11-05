use pico_common::PicoResult;
use pico_device::{
    cm3::PLCM3Device, drdaq::DrDAQDevice, hrdl::HRDLDevice, oscilloscope::OscilloscopeDevice,
    pl1000::PL1000Device, pt104::PT104Device, tc08::TC08Device, PicoDevice,
};
use pico_driver::{
    cm3::PLCM3Driver, drdaq::DrDAQDriver, hrdl::HRDLDriver, oscilloscope::OscilloscopeDriver,
    pl1000::PL1000Driver, pt104::PT104Driver, tc08::TC08Driver, PicoDriver,
};

/// Lists the units of one loaded driver
///
/// - Each family has its own implementation, as each finds its units in a different way.
/// - The implementation for [`PicoDriver`] calls the one for its family.
pub trait EnumerateDriver<D> {
    /// Lists the units that the driver can see, with one result for each unit
    ///
    /// - Returns `Err` when the driver cannot list its units.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<D>>>;
}

impl EnumerateDriver<DrDAQDevice> for DrDAQDriver {
    /// The DrDAQ driver has no enumerate call, and reports no serial until a unit is open.
    /// So this opens each attached unit in turn, and the devices come back open.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<DrDAQDevice>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(DrDAQDevice::new_closed(
                    self,
                    info.serial.clone(),
                    Some(info),
                ))
            })
            .collect())
    }
}

impl EnumerateDriver<OscilloscopeDevice> for OscilloscopeDriver {
    /// The oscilloscope driver lists its units without opening them, so the devices come back closed.
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
    /// The HRDL driver has no enumerate call, and reports no serial until a unit is open.
    /// So this opens each attached unit in turn, and the devices come back open.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<HRDLDevice>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(HRDLDevice::new_closed(
                    self,
                    info.serial.clone(),
                    Some(info),
                ))
            })
            .collect())
    }
}

impl EnumerateDriver<PL1000Device> for PL1000Driver {
    /// The PL1000 driver has no enumerate call, and reports no serial until a unit is open.
    /// So this opens each attached unit in turn, and the devices come back open.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PL1000Device>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(PL1000Device::new_closed(
                    self,
                    info.serial.clone(),
                    Some(info),
                ))
            })
            .collect())
    }
}

impl EnumerateDriver<PLCM3Device> for PLCM3Driver {
    /// The CM3 driver lists the attached serials with `PLCM3Enumerate`, and this opens each unit by serial.
    /// The devices come back open.
    /// See the PicoLog CM3 Programmer's Guide (plcm3pg r4), sections 2.3 and 2.7.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PLCM3Device>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(PLCM3Device::new_closed(
                    self,
                    info.serial.clone(),
                    Some(info),
                ))
            })
            .collect())
    }
}

impl EnumerateDriver<PT104Device> for PT104Driver {
    /// The PT-104 driver has no enumerate call for USB units, and reports no serial until a unit is open.
    /// So this opens each attached unit in turn, and the devices come back open.
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<PT104Device>>> {
        Ok(self
            .open_unit_iter()
            .map(|result| {
                let handle = result?;
                let info = self.get_unit_info(handle)?;
                Ok(PT104Device::new_closed(
                    self,
                    info.serial.clone(),
                    Some(info),
                ))
            })
            .collect())
    }
}

impl EnumerateDriver<TC08Device> for TC08Driver {
    /// The TC-08 driver has no enumerate call, and reports no serial until a unit is open.
    /// So this opens each attached unit in turn, and the devices come back open.
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
