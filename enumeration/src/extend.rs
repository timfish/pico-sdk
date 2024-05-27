use pico_common::PicoResult;
use pico_device::{oscilloscope::OscilloscopeDevice, tc08::TC08Device, PicoDevice};
use pico_driver::{oscilloscope::OscilloscopeDriver, tc08::TC08Driver, PicoDriver};

pub trait EnumerateDriver<D> {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<D>>>;
}

impl EnumerateDriver<OscilloscopeDevice> for OscilloscopeDriver {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<OscilloscopeDevice>>> {
        self.enumerate_units().map(|units| {
            units
                .into_iter()
                .map(|unit| {
                    Ok(OscilloscopeDevice::new_closed(
                        self,
                        unit.serial.to_string(),
                        unit.variant,
                    ))
                })
                .collect()
        })
    }
}

impl EnumerateDriver<TC08Device> for TC08Driver {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<TC08Device>>> {
        Ok(self
            .open_unit_iter()
            .flat_map(|result| {
                result.map(|handle| {
                    self.get_unit_info(handle)
                        .map(|info| TC08Device::new_closed(self, info.serial.clone(), Some(info)))
                })
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
