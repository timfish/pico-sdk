//! Guide: ps6000apg-9 (2026) §2.4, §3.48–3.49. Ports are the TA369 MSO
//! pods: PORT0 is Digital 1, PORT1 Digital 2.

use super::PS6000ADriver;
use crate::oscilloscope::{
    buffer_pointer, features::raw_ports_only, DigitalPortDriver, OscilloscopeDriverInternal,
    SampleBuffer,
};
use pico_common::{
    DigitalHysteresis, DigitalPort, DigitalPortConfig, Downsample, PicoResult, PicoStatus,
};
use pico_sys_dynamic::ps6000a::{
    enPicoAction_PICO_ADD, enPicoDataType_PICO_INT16_T,
    enPicoDigitalPortHysteresis_PICO_HIGH_200MV, enPicoDigitalPortHysteresis_PICO_LOW_50MV,
    enPicoDigitalPortHysteresis_PICO_NORMAL_100MV,
    enPicoDigitalPortHysteresis_PICO_VERY_HIGH_400MV, enPicoRatioMode_PICO_RATIO_MODE_RAW,
    PICO_DIGITAL_PORT_HYSTERESIS, PICO_STREAMING_DATA_INFO,
};

/// The pods' threshold range, ±8 V (§3.49).
const POD_VOLTS: f64 = 8.0;

fn hysteresis(h: Option<DigitalHysteresis>) -> PICO_DIGITAL_PORT_HYSTERESIS {
    match h {
        Some(DigitalHysteresis::VeryHigh) => enPicoDigitalPortHysteresis_PICO_VERY_HIGH_400MV,
        Some(DigitalHysteresis::High) => enPicoDigitalPortHysteresis_PICO_HIGH_200MV,
        Some(DigitalHysteresis::Normal) | None => enPicoDigitalPortHysteresis_PICO_NORMAL_100MV,
        Some(DigitalHysteresis::Low) => enPicoDigitalPortHysteresis_PICO_LOW_50MV,
    }
}

impl PS6000ADriver {
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
            self.bindings.ps6000aSetDataBuffer(
                handle,
                port.code(),
                buffer_pointer(buffer),
                buffer.len() as i32,
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

impl DigitalPortDriver for PS6000ADriver {
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
                self.bindings.ps6000aSetDigitalPortOff(handle, port.code())
            })
            .to_result((), "set_digital_port");
        };
        let mut levels = config.threshold.per_bit(POD_VOLTS)?;
        PicoStatus::from(unsafe {
            self.bindings.ps6000aSetDigitalPortOn(
                handle,
                port.code(),
                levels.as_mut_ptr(),
                levels.len() as i16,
                hysteresis(config.hysteresis),
            )
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

    #[test]
    fn hysteresis_takes_the_header_values() {
        assert_eq!(hysteresis(Some(DigitalHysteresis::VeryHigh)), 0);
        assert_eq!(hysteresis(Some(DigitalHysteresis::High)), 1);
        assert_eq!(hysteresis(None), 2);
        assert_eq!(hysteresis(Some(DigitalHysteresis::Low)), 3);
    }

    #[test]
    fn port_codes_match_the_header() {
        use pico_sys_dynamic::ps6000a::{enPicoChannel_PICO_PORT0, enPicoChannel_PICO_PORT1};
        assert_eq!(DigitalPort::Port0.code(), enPicoChannel_PICO_PORT0);
        assert_eq!(DigitalPort::Port1.code(), enPicoChannel_PICO_PORT1);
        // ±8 V pods: 2 V is a quarter of full scale.
        assert_eq!(
            pico_common::DigitalThreshold::Volts(2.0)
                .per_bit(POD_VOLTS)
                .unwrap(),
            [8192; 8]
        );
    }
}
