use pico_common::PicoResult;
use pico_device::{oscilloscope, tc08, PicoDevice};
use pico_driver::PicoDriver;

pub trait EnumerateDriver<D> {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<D>>>;
}

impl EnumerateDriver<oscilloscope::OscilloscopeDevice> for oscilloscope::ArcDriver {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<oscilloscope::OscilloscopeDevice>>> {
        self.enumerate_units().map(|units| {
            units
                .into_iter()
                .map(|unit| {
                    Ok(oscilloscope::OscilloscopeDevice::new_closed(
                        self,
                        unit.serial.to_string(),
                        unit.variant,
                    ))
                })
                .collect()
        })
    }
}

impl EnumerateDriver<tc08::TC08Device> for tc08::ArcDriver {
    fn enumerate_devices(&self) -> PicoResult<Vec<PicoResult<tc08::TC08Device>>> {
        Ok(self
            .open_unit_iter()
            .flat_map(|result| {
                result.map(|handle| {
                    self.get_unit_info(handle).map(|info| {
                        tc08::TC08Device::new_closed(self.clone(), info.serial.clone(), Some(info))
                    })
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
