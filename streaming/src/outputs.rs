use crate::state::{StreamDevice, StreamingRunner};
use pico_common::{OutputSpec, OutputValue, Outputs, PicoError, PicoResult, PicoStatus};
use pico_device::outputs::DeviceOutputs;
use std::fmt;

/// A runner's outputs, set on the handle it holds between two steps,
/// so a stream goes on.
impl<Device, Config, Info, Stream, Event> Outputs
    for StreamingRunner<Device, Config, Info, Stream, Event>
where
    Device: Clone
        + StreamDevice<Config, Info, Stream, Event>
        + DeviceOutputs<Info>
        + Send
        + Sync
        + 'static,
    Config: Clone + Send + Sync + 'static,
    Info: Clone + fmt::Debug + Send + Sync + 'static,
    Stream: Clone + fmt::Debug + Send + Sync + 'static,
    Event: Clone + Send + Sync + 'static,
{
    fn outputs(&self) -> PicoResult<Vec<OutputSpec>> {
        self.with_open(|device, info| device.outputs_of(info))
            .unwrap_or_else(|| Err(closed("outputs")))
    }

    fn set_output(&self, id: &str, value: &OutputValue) -> PicoResult<OutputValue> {
        self.with_open(|device, info| device.set_output_of(info, id, value))
            .unwrap_or_else(|| Err(closed("set_output")))
    }
}

fn closed(context: &str) -> PicoError {
    PicoError::from_status(PicoStatus::INVALID_HANDLE, context)
}
