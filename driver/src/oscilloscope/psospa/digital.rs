//! Guide: psospa API Programmer's Guide (psospa-3) §2.4, §3.43–3.44.
//! One threshold in volts per port and no hysteresis setting.

use super::PSOSPADriver;
use crate::oscilloscope::{
    buffer_pointer,
    features::{no_hysteresis, raw_ports_only, PORT_VOLTS},
    DigitalPortDriver, OscilloscopeDriverInternal, SampleBuffer,
};
use pico_common::{DigitalPort, DigitalPortConfig, Downsample, PicoResult, PicoStatus};
use pico_sys_dynamic::psospa::{
    enPicoAction_PICO_ADD, enPicoDataType_PICO_INT16_T, enPicoRatioMode_PICO_RATIO_MODE_RAW,
    PICO_STREAMING_DATA_INFO,
};

impl PSOSPADriver {
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
        buffer.as_i16(context)?;
        PicoStatus::from(unsafe {
            self.bindings.psospaSetDataBuffer(
                handle,
                port.code(),
                buffer_pointer(buffer),
                buffer.len() as u64,
                enPicoDataType_PICO_INT16_T,
                segment,
                enPicoRatioMode_PICO_RATIO_MODE_RAW,
                enPicoAction_PICO_ADD,
            )
        })
        .to_result((), context)
    }

    /// The entries of the ports with stream buffers, for
    /// `GetStreamingLatestValues`.
    pub(super) fn port_infos(&self, handle: i16) -> Vec<PICO_STREAMING_DATA_INFO> {
        self.ports
            .of(handle)
            .into_iter()
            .map(|port| PICO_STREAMING_DATA_INFO {
                bufferIndex_: 0,
                channel_: port.code(),
                mode_: enPicoRatioMode_PICO_RATIO_MODE_RAW,
                noOfSamples_: 0,
                overflow_: 0,
                startIndex_: 0,
                type_: enPicoDataType_PICO_INT16_T,
            })
            .collect()
    }
}

impl DigitalPortDriver for PSOSPADriver {
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
        let Some(config) = config else {
            self.ports.remove(handle, port);
            return PicoStatus::from(unsafe {
                self.bindings.psospaSetDigitalPortOff(handle, port.code())
            })
            .to_result((), "set_digital_port");
        };
        no_hysteresis(config)?;
        let volts = config.threshold.volts(PORT_VOLTS)?;
        PicoStatus::from(unsafe {
            self.bindings
                .psospaSetDigitalPortOn(handle, port.code(), volts)
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
        self.set_port_buffer(handle, port, &buffer, 0, downsample, "set_digital_buffer")?;
        self.ports.add(handle, port);
        Ok(())
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
    use pico_sys_dynamic::psospa::{enPicoChannel_PICO_PORT0, enPicoChannel_PICO_PORT1};

    #[test]
    fn port_codes_match_the_header() {
        assert_eq!(DigitalPort::Port0.code(), enPicoChannel_PICO_PORT0);
        assert_eq!(DigitalPort::Port1.code(), enPicoChannel_PICO_PORT1);
    }
}
