use crate::oscilloscope::SampleBuffer;
use pico_common::{DigitalPort, DigitalPortConfig, Downsample, PicoError, PicoResult};

/// The digital inputs of MSO models. Data comes in `i16` buffers like
/// the analogue data, one per port, the port's 8 bits in the low byte
/// (see [`pico_common::port_bytes`]).
pub trait DigitalPortDriver {
    /// The ports the unit has.
    fn digital_ports(&self, _handle: i16) -> PicoResult<Vec<DigitalPort>> {
        Err(PicoError::not_supported("digital_ports"))
    }
    /// Turns a port on with `config`, or off with `None`.
    fn set_digital_port(
        &self,
        _handle: i16,
        _port: DigitalPort,
        _config: Option<&DigitalPortConfig>,
    ) -> PicoResult<()> {
        Err(PicoError::not_supported("set_digital_port"))
    }
    /// Give the driver an `i16` buffer for a port's samples in a
    /// stream, as `StreamingDriver::set_data_buffer` does for a channel.
    /// A port turned off leaves the stream.
    fn set_digital_buffer(
        &self,
        _handle: i16,
        _port: DigitalPort,
        _buffer: SampleBuffer,
        _buffer_len: usize,
        _downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        Err(PicoError::not_supported("set_digital_buffer"))
    }
    /// Give the driver an `i16` buffer for a port's samples of a block
    /// capture into `segment`, as `BlockDriver::set_block_buffer` does
    /// for a channel.
    fn set_digital_block_buffer(
        &self,
        _handle: i16,
        _port: DigitalPort,
        _buffer: SampleBuffer,
        _segment: u64,
        _downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        Err(PicoError::not_supported("set_digital_block_buffer"))
    }
}

/// Refuses a downsample of port data: a mean or sum of bit patterns
/// means nothing.
pub(crate) fn raw_ports_only(downsample: Option<Downsample>, context: &str) -> PicoResult<()> {
    match Downsample::effective(downsample)? {
        None => Ok(()),
        Some(_) => Err(PicoError::from_status(
            pico_common::PicoStatus::RATIO_MODE_NOT_SUPPORTED,
            &format!("{context}: digital ports stream raw samples only"),
        )),
    }
}

/// The threshold range of the ps2000a, ps3000a, ps5000a and psospa
/// ports: ±5 V (ps2000a guide §3.43, ps3000a §4.46, ps5000a §4.50; the
/// 5000E data sheet MM145 for psospa).
pub(crate) const PORT_VOLTS: f64 = 5.0;

/// A family with one fixed hysteresis refuses another.
pub(crate) fn no_hysteresis(config: &DigitalPortConfig) -> PicoResult<()> {
    match config.hysteresis {
        None => Ok(()),
        Some(_) => Err(PicoError::not_supported("digital port hysteresis")),
    }
}

/// The ports with a stream buffer on each handle, for drivers whose
/// `GetStreamingLatestValues` takes a list of what to read (ps6000a,
/// psospa).
#[derive(Default)]
pub(crate) struct StreamPorts(
    parking_lot::RwLock<std::collections::HashMap<i16, std::collections::BTreeSet<DigitalPort>>>,
);

impl StreamPorts {
    pub(crate) fn add(&self, handle: i16, port: DigitalPort) {
        self.0.write().entry(handle).or_default().insert(port);
    }

    pub(crate) fn remove(&self, handle: i16, port: DigitalPort) {
        if let Some(ports) = self.0.write().get_mut(&handle) {
            ports.remove(&port);
        }
    }

    pub(crate) fn clear(&self, handle: i16) {
        self.0.write().remove(&handle);
    }

    pub(crate) fn of(&self, handle: i16) -> Vec<DigitalPort> {
        self.0
            .read()
            .get(&handle)
            .map(|p| p.iter().copied().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::PicoStatus;

    struct NoPorts;
    impl DigitalPortDriver for NoPorts {}

    #[test]
    fn families_without_ports_refuse_them() {
        let config = DigitalPortConfig::volts(1.0);
        let not_supported = PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE;
        assert_eq!(NoPorts.digital_ports(1).unwrap_err().status, not_supported);
        assert_eq!(
            NoPorts
                .set_digital_port(1, DigitalPort::Port0, Some(&config))
                .unwrap_err()
                .status,
            not_supported
        );
    }

    #[test]
    fn port_data_is_raw_only() {
        assert!(raw_ports_only(None, "t").is_ok());
        assert!(raw_ports_only(Some(Downsample::average(1)), "t").is_ok());
        assert_eq!(
            raw_ports_only(Some(Downsample::average(4)), "t")
                .unwrap_err()
                .status,
            PicoStatus::RATIO_MODE_NOT_SUPPORTED
        );
        let mut config = DigitalPortConfig::volts(1.0);
        assert!(no_hysteresis(&config).is_ok());
        config.hysteresis = Some(pico_common::DigitalHysteresis::Low);
        assert!(no_hysteresis(&config).is_err());
    }

    #[test]
    fn stream_ports_follow_on_and_off() {
        let ports = StreamPorts::default();
        ports.add(1, DigitalPort::Port1);
        ports.add(1, DigitalPort::Port0);
        assert_eq!(ports.of(1), DigitalPort::ALL.to_vec());
        ports.remove(1, DigitalPort::Port0);
        assert_eq!(ports.of(1), vec![DigitalPort::Port1]);
        ports.clear(1);
        assert!(ports.of(1).is_empty());
    }
}
