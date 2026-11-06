//! The MSO digital ports of a scope, for streams and block captures
//!
//! The scope stream and [`crate::BlockRunner`] call these functions.
//! Call them yourself only when you drive the driver of a scope directly.

use pico_common::{
    port_bytes, DigitalPort, PicoError, PicoResult, PicoStatus, SampleType, TriggerConfig,
};
use pico_device::oscilloscope::{OscilloscopeConfig, OscilloscopeDevice, OscilloscopeInfo};
use pico_driver::oscilloscope::SampleBuffer;
use std::collections::HashMap;
use tracing::debug;

/// One `i16` buffer for each port that a config turns on
pub type PortBuffers = HashMap<DigitalPort, SampleBuffer>;

/// Turns off every port of the unit
///
/// - Call it with the channels off, before a change of resolution.
///   Ports left on from a stream of ports with no channel hold the unit at 8 bits, and a change to more bits then fails.
/// - A port with no MSO pod is not an error.
/// - Returns Err when the driver refuses a call.
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

/// Turns on the ports that `config` names, with their thresholds, and turns off the other ports of the unit
///
/// - Call it after you set the channels, and before you give the buffers (ps2000a guide §2.6.4.1).
/// - A unit with no ports and a config with none make no call.
/// - Returns Err when the driver refuses a call.
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

/// Refuses a digital trigger on a port that `config` leaves off
///
/// - The unit compares only the inputs of a port that is on, with its threshold.
/// - Returns Err with `INVALID_DIGITAL_PORT` when the trigger names a line of a port that is off.
pub fn check_trigger_ports(config: &OscilloscopeConfig, trigger: &TriggerConfig) -> PicoResult<()> {
    for bit in trigger.digital_bits() {
        let on = DigitalPort::ALL
            .into_iter()
            .any(|p| p.index() == bit.port() && config.digital.contains_key(&p));
        if !on {
            return Err(PicoError::from_status(
                PicoStatus::INVALID_DIGITAL_PORT,
                &format!(
                    "the digital trigger names D{}, whose port {} is off",
                    bit.bit,
                    bit.port()
                ),
            ));
        }
    }
    Ok(())
}

/// A buffer of `len` zero samples for each port that `config` turns on
pub fn port_buffers(config: &OscilloscopeConfig, len: usize) -> PortBuffers {
    config
        .digital
        .keys()
        .map(|port| (*port, SampleBuffer::new(SampleType::I16, len)))
        .collect()
}

/// The samples in `range` of the buffer of each port, one byte for each sample
///
/// - Panics when `range` is past the end of a buffer.
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
