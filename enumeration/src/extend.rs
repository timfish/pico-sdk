use pico_common::PicoResult;
use pico_device::{oscilloscope::OscilloscopeDevice, tc08::TC08Device, PicoDevice};
use pico_driver::{oscilloscope::OscilloscopeDriver, tc08::TC08Driver, PicoDriver};

/// Lists the devices a loaded driver can see
///
/// Implemented per family because discovery works differently for each, then for [`PicoDriver`]
/// to dispatch to the right one.
pub trait EnumerateDriver<D> {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<D>>>;
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
            PicoDriver::Oscilloscope(driver) => driver
                .enumerate_devices()?
                .into_iter()
                .map(|d| d.map(PicoDevice::Oscilloscope))
                .collect(),
            PicoDriver::TC08(driver) => driver
                .enumerate_devices()?
                .into_iter()
                .map(|d| d.map(PicoDevice::TC08))
                .collect(),
        })
    }
}
