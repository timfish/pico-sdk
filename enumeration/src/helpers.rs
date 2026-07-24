use crate::EnumerationError;
use pico_common::Driver;
use pico_device::{
    cm3::PLCM3Device, drdaq::DrDAQDevice, hrdl::HRDLDevice, oscilloscope::OscilloscopeDevice,
    pl1000::PL1000Device, pt104::PT104Device, tc08::TC08Device, PicoDevice,
};

/// Generic over the device type so it works for both the mixed
/// [`crate::DeviceEnumerator::enumerate`] and the family specific
/// [`crate::DeviceEnumerator::enumerate_oscilloscopes`].
pub trait EnumResultHelpers<D> {
    fn missing_drivers(&self) -> Vec<Driver>;
    fn devices_and_errors(self) -> (Vec<D>, Vec<EnumerationError>);
}

impl<D> EnumResultHelpers<D> for Vec<Result<D, EnumerationError>> {
    fn missing_drivers(&self) -> Vec<Driver> {
        let mut failed_results = self
            .iter()
            .flat_map(|r| match r {
                Err(EnumerationError::DriverLoadError { driver, .. }) => Some(*driver),
                Err(EnumerationError::VersionError { driver, .. }) => Some(*driver),
                _ => None,
            })
            .collect::<Vec<Driver>>();

        failed_results.sort();
        failed_results.dedup();
        failed_results
    }

    fn devices_and_errors(self) -> (Vec<D>, Vec<EnumerationError>) {
        // Split by matching rather than `partition` + `unwrap`, so `D` needs no `Debug` bound
        let mut devices = Vec::new();
        let mut errors = Vec::new();

        for result in self {
            match result {
                Ok(device) => devices.push(device),
                Err(error) => errors.push(error),
            }
        }

        (devices, errors)
    }
}

/// Splits a mixed enumeration result by instrument family
pub trait PicoDeviceHelpers {
    fn cm3s(self) -> Vec<PLCM3Device>;
    fn drdaqs(self) -> Vec<DrDAQDevice>;
    fn hrdls(self) -> Vec<HRDLDevice>;
    fn oscilloscopes(self) -> Vec<OscilloscopeDevice>;
    fn pl1000s(self) -> Vec<PL1000Device>;
    fn pt104s(self) -> Vec<PT104Device>;
    fn tc08s(self) -> Vec<TC08Device>;
}

impl PicoDeviceHelpers for Vec<PicoDevice> {
    fn cm3s(self) -> Vec<PLCM3Device> {
        self.into_iter()
            .filter_map(|d| match d {
                PicoDevice::PLCM3(cm3) => Some(cm3),
                _ => None,
            })
            .collect()
    }

    fn drdaqs(self) -> Vec<DrDAQDevice> {
        self.into_iter()
            .filter_map(|d| match d {
                PicoDevice::DrDAQ(drdaq) => Some(drdaq),
                _ => None,
            })
            .collect()
    }

    fn hrdls(self) -> Vec<HRDLDevice> {
        self.into_iter()
            .filter_map(|d| match d {
                PicoDevice::PicoHRDL(hrdl) => Some(hrdl),
                _ => None,
            })
            .collect()
    }

    fn oscilloscopes(self) -> Vec<OscilloscopeDevice> {
        self.into_iter()
            .filter_map(|d| match d {
                PicoDevice::Oscilloscope(scope) => Some(scope),
                _ => None,
            })
            .collect()
    }

    fn pl1000s(self) -> Vec<PL1000Device> {
        self.into_iter()
            .filter_map(|d| match d {
                PicoDevice::PL1000(pl1000) => Some(pl1000),
                _ => None,
            })
            .collect()
    }

    fn pt104s(self) -> Vec<PT104Device> {
        self.into_iter()
            .filter_map(|d| match d {
                PicoDevice::PT104(pt104) => Some(pt104),
                _ => None,
            })
            .collect()
    }

    fn tc08s(self) -> Vec<TC08Device> {
        self.into_iter()
            .filter_map(|d| match d {
                PicoDevice::TC08(tc08) => Some(tc08),
                _ => None,
            })
            .collect()
    }
}
