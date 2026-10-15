//! Guide: ps5000apg.en r4 §3.3, §4.50.

use super::PS5000ADriver;
use crate::oscilloscope::{
    features::{no_hysteresis, raw_ports_only, PORT_VOLTS},
    DigitalPortDriver, OscilloscopeDriverInternal, SampleBuffer,
};
use pico_common::{DigitalPort, DigitalPortConfig, Downsample, PicoError, PicoResult, PicoStatus};
use pico_sys_dynamic::ps5000a::enPS5000ARatioMode_PS5000A_RATIO_MODE_NONE;

impl PS5000ADriver {
    fn set_port_buffer(
        &self,
        handle: i16,
        port: DigitalPort,
        buffer: &SampleBuffer,
        segment: u64,
        downsample: Option<Downsample>,
        context: &str,
    ) -> PicoResult<()> {
        raw_ports_only(downsample, context)?;
        let mut buffer = buffer.as_i16(context)?.write();
        let segment = u32::try_from(segment)
            .map_err(|_| PicoError::from_status(PicoStatus::SEGMENT_OUT_OF_RANGE, context))?;
        PicoStatus::from(unsafe {
            self.bindings.ps5000aSetDataBuffer(
                handle,
                port.code(),
                buffer.as_mut_ptr(),
                buffer.len() as i32,
                segment,
                enPS5000ARatioMode_PS5000A_RATIO_MODE_NONE,
            )
        })
        .to_result((), context)
    }
}

impl DigitalPortDriver for PS5000ADriver {
    fn digital_ports(&self, handle: i16) -> PicoResult<Vec<DigitalPort>> {
        Ok(DigitalPort::first(self.capabilities(handle)?.digital_ports))
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_digital_port(
        &self,
        handle: i16,
        port: DigitalPort,
        config: Option<&DigitalPortConfig>,
    ) -> PicoResult<()> {
        let level = match config {
            Some(config) => {
                no_hysteresis(config)?;
                config.threshold.counts(PORT_VOLTS)?
            }
            None => 0,
        };
        PicoStatus::from(unsafe {
            self.bindings
                .ps5000aSetDigitalPort(handle, port.code(), config.is_some().into(), level)
        })
        .to_result((), "set_digital_port")
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_digital_buffer(
        &self,
        handle: i16,
        port: DigitalPort,
        buffer: SampleBuffer,
        _buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        self.set_port_buffer(handle, port, &buffer, 0, downsample, "set_digital_buffer")
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_digital_block_buffer(
        &self,
        handle: i16,
        port: DigitalPort,
        buffer: SampleBuffer,
        segment: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        self.set_port_buffer(
            handle,
            port,
            &buffer,
            segment,
            downsample,
            "set_digital_block_buffer",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::DigitalThreshold;
    use pico_sys_dynamic::ps5000a::{
        enPS5000AChannel_PS5000A_DIGITAL_PORT0, enPS5000AChannel_PS5000A_DIGITAL_PORT1,
    };

    #[test]
    fn ports_and_levels_match_the_guide() {
        assert_eq!(
            DigitalPort::Port0.code(),
            enPS5000AChannel_PS5000A_DIGITAL_PORT0
        );
        assert_eq!(
            DigitalPort::Port1.code(),
            enPS5000AChannel_PS5000A_DIGITAL_PORT1
        );
        // ±5 V in counts of ±32767.
        assert_eq!(
            DigitalThreshold::Volts(2.5).counts(PORT_VOLTS).unwrap(),
            16384
        );
        assert!(DigitalThreshold::Volts(-6.0).counts(PORT_VOLTS).is_err());
    }
}
