//! [`Outputs`] on open devices. A device that went into a streaming
//! runner has given its info to the runner; the runner's own `Outputs`
//! reaches the handle it holds, so outputs change while it streams.

use crate::{
    cm3::PLCM3Device, drdaq::DrDAQDevice, hrdl::HRDLDevice, oscilloscope::OscilloscopeDevice,
    oscilloscope::OscilloscopeInfo, pl1000::PL1000Device, pt104::PT104Device, tc08::TC08Device,
};
use pico_common::{
    DrDAQInfo, HRDLInfo, OutputSpec, OutputValue, Outputs, PL1000Info, PLCM3Info, PT104Info,
    PicoChannel, PicoError, PicoResult, PicoStatus, TC08Info,
};
use pico_driver::oscilloscope::outputs::{scope_outputs, set_scope_output};

/// The outputs of a device, given the info of its open unit.
pub trait DeviceOutputs<Info> {
    fn outputs_of(&self, info: &Info) -> PicoResult<Vec<OutputSpec>>;
    fn set_output_of(&self, info: &Info, id: &str, value: &OutputValue) -> PicoResult<OutputValue>;
}

fn closed(context: &str) -> PicoError {
    PicoError::from_status(PicoStatus::INVALID_HANDLE, context)
}

fn none_listed(id: &str) -> PicoError {
    PicoError::from_status(PicoStatus::INVALID_PARAMETER, &format!("no output {id}"))
}

impl DeviceOutputs<OscilloscopeInfo> for OscilloscopeDevice {
    fn outputs_of(&self, info: &OscilloscopeInfo) -> PicoResult<Vec<OutputSpec>> {
        let mut channels: Vec<PicoChannel> = info.valid_channel_ranges.keys().copied().collect();
        channels.sort_by_key(|c| u32::from(*c));
        Ok(scope_outputs(
            &**self.driver,
            *info.handle,
            &info.capabilities,
            &channels,
        ))
    }

    fn set_output_of(
        &self,
        info: &OscilloscopeInfo,
        id: &str,
        value: &OutputValue,
    ) -> PicoResult<OutputValue> {
        let specs = self.outputs_of(info)?;
        set_scope_output(&**self.driver, *info.handle, &specs, id, value)
    }
}

impl DeviceOutputs<PL1000Info> for PL1000Device {
    fn outputs_of(&self, _info: &PL1000Info) -> PicoResult<Vec<OutputSpec>> {
        Ok(pico_driver::pl1000::outputs())
    }

    fn set_output_of(
        &self,
        info: &PL1000Info,
        id: &str,
        value: &OutputValue,
    ) -> PicoResult<OutputValue> {
        self.driver.set_output(*info.handle, id, value)
    }
}

impl DeviceOutputs<DrDAQInfo> for DrDAQDevice {
    fn outputs_of(&self, _info: &DrDAQInfo) -> PicoResult<Vec<OutputSpec>> {
        Ok(pico_driver::drdaq::outputs())
    }

    fn set_output_of(
        &self,
        info: &DrDAQInfo,
        id: &str,
        value: &OutputValue,
    ) -> PicoResult<OutputValue> {
        self.driver.set_output(*info.handle, id, value)
    }
}

impl DeviceOutputs<HRDLInfo> for HRDLDevice {
    fn outputs_of(&self, info: &HRDLInfo) -> PicoResult<Vec<OutputSpec>> {
        Ok(pico_driver::hrdl::outputs(info))
    }

    fn set_output_of(
        &self,
        info: &HRDLInfo,
        id: &str,
        value: &OutputValue,
    ) -> PicoResult<OutputValue> {
        self.driver.set_output(info, id, value)
    }
}

/// Loggers with no outputs (TC-08, PT-104, CM3; their headers have no
/// output call).
macro_rules! no_outputs {
    ($device:ty, $info:ty) => {
        impl DeviceOutputs<$info> for $device {
            fn outputs_of(&self, _info: &$info) -> PicoResult<Vec<OutputSpec>> {
                Ok(Vec::new())
            }

            fn set_output_of(
                &self,
                _info: &$info,
                id: &str,
                _value: &OutputValue,
            ) -> PicoResult<OutputValue> {
                Err(none_listed(id))
            }
        }
    };
}

no_outputs!(TC08Device, TC08Info);
no_outputs!(PT104Device, PT104Info);
no_outputs!(PLCM3Device, PLCM3Info);

/// `Outputs` on a device that holds its open unit's info.
macro_rules! outputs_while_open {
    ($($device:ty),*) => {$(
        impl Outputs for $device {
            fn outputs(&self) -> PicoResult<Vec<OutputSpec>> {
                let info = self.info.as_ref().ok_or_else(|| closed("outputs"))?;
                self.outputs_of(info)
            }

            fn set_output(&self, id: &str, value: &OutputValue) -> PicoResult<OutputValue> {
                let info = self.info.as_ref().ok_or_else(|| closed("set_output"))?;
                self.set_output_of(info, id, value)
            }
        }
    )*};
}

outputs_while_open!(
    OscilloscopeDevice,
    PL1000Device,
    DrDAQDevice,
    HRDLDevice,
    TC08Device,
    PT104Device,
    PLCM3Device
);
