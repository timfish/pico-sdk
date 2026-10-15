//! MSO digital ports for scope streams and block captures.

use pico_common::{port_bytes, DigitalPort, PicoResult, PicoStatus, SampleType};
use pico_device::oscilloscope::{OscilloscopeConfig, OscilloscopeDevice, OscilloscopeInfo};
use pico_driver::oscilloscope::SampleBuffer;
use std::collections::HashMap;
use tracing::debug;

/// One `i16` buffer for each port a config turns on.
pub type PortBuffers = HashMap<DigitalPort, SampleBuffer>;

/// Turns off every port of the unit. Call it with the channels off,
/// before a resolution change: ports left on from a stream of ports
/// alone hold the unit at 8 bits, and a change to more then fails.
pub fn ports_off(device: &OscilloscopeDevice, info: &OscilloscopeInfo) -> PicoResult<()> {
    for port in DigitalPort::first(info.capabilities.digital_ports) {
        match device.driver.set_digital_port(*info.handle, port, None) {
            // A ps6000a with no pod on the port has nothing to turn off.
            Err(error) if error.status == PicoStatus::NO_MSO_POD_CONNECTED => {}
            result => result?,
        }
    }
    Ok(())
}

/// Turns on the ports `config` names, with their thresholds, and turns
/// off the unit's other ports. Call it after the channels are set and
/// before the buffers are given (ps2000a guide §2.6.4.1). A unit with
/// no ports and a config with none make no call.
pub fn set_up_ports(
    device: &OscilloscopeDevice,
    info: &OscilloscopeInfo,
    config: &OscilloscopeConfig,
) -> PicoResult<()> {
    let handle = *info.handle;
    for port in DigitalPort::first(info.capabilities.digital_ports) {
        match config.digital.get(&port) {
            Some(port_config) => {
                device
                    .driver
                    .set_digital_port(handle, port, Some(port_config))?;
                debug!(%port, threshold = ?port_config.threshold, "digital port on");
            }
            None => match device.driver.set_digital_port(handle, port, None) {
                Err(error) if error.status == PicoStatus::NO_MSO_POD_CONNECTED => {}
                result => result?,
            },
        }
    }
    Ok(())
}

/// A zeroed buffer of `len` samples for each port `config` turns on.
pub fn port_buffers(config: &OscilloscopeConfig, len: usize) -> PortBuffers {
    config
        .digital
        .keys()
        .map(|port| (*port, SampleBuffer::new(SampleType::I16, len)))
        .collect()
}

/// The bytes of each port's samples in `range` of its buffer.
pub fn read_ports(
    buffers: &PortBuffers,
    range: std::ops::Range<usize>,
) -> HashMap<DigitalPort, Vec<u8>> {
    buffers
        .iter()
        .filter_map(|(port, buffer)| match buffer {
            SampleBuffer::I16(b) => Some((*port, port_bytes(&b.read()[range.clone()]))),
            _ => None,
        })
        .collect()
}
