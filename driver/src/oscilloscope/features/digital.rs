use crate::oscilloscope::SampleBuffer;
use pico_common::{DigitalPort, DigitalPortConfig, Downsample, PicoError, PicoResult};
use std::collections::HashMap;

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
/// psospa), and how far each buffer of the list is filled.
#[derive(Default)]
pub(crate) struct StreamPorts {
    ports: parking_lot::RwLock<HashMap<i16, std::collections::BTreeSet<DigitalPort>>>,
    joined: parking_lot::Mutex<HashMap<i16, Joined>>,
}

impl StreamPorts {
    pub(crate) fn add(&self, handle: i16, port: DigitalPort) {
        self.ports.write().entry(handle).or_default().insert(port);
    }

    pub(crate) fn remove(&self, handle: i16, port: DigitalPort) {
        if let Some(ports) = self.ports.write().get_mut(&handle) {
            ports.remove(&port);
        }
    }

    pub(crate) fn clear(&self, handle: i16) {
        self.ports.write().remove(&handle);
        self.buffers_set(handle);
    }

    pub(crate) fn of(&self, handle: i16) -> Vec<DigitalPort> {
        self.ports
            .read()
            .get(&handle)
            .map(|p| p.iter().copied().collect())
            .unwrap_or_default()
    }

    /// New buffers start at index 0.
    pub(crate) fn buffers_set(&self, handle: i16) {
        self.joined.lock().remove(&handle);
    }

    /// The samples every buffer of `handle` holds that were not given
    /// before, from one call's `(channel, start, count)` entries.
    pub(crate) fn join(
        &self,
        handle: i16,
        entries: &[(u32, usize, usize)],
    ) -> Option<(usize, usize)> {
        self.joined.lock().entry(handle).or_default().join(entries)
    }
}

/// How far each buffer of one stream is filled. On a 5464E the port
/// buffers' samples came in other calls than channel A's, so samples
/// go out once every buffer that has reported holds them.
#[derive(Debug, Default)]
pub(crate) struct Joined {
    /// The end of the samples each buffer has, by channel code.
    ends: HashMap<u32, usize>,
    /// The end of the samples given out.
    given: usize,
}

impl Joined {
    pub(crate) fn join(&mut self, entries: &[(u32, usize, usize)]) -> Option<(usize, usize)> {
        for &(channel, start, count) in entries {
            if count > 0 {
                self.ends.insert(channel, start + count);
            }
        }
        let first = entries.first().map(|e| (e.1, e.2));
        if entries.iter().any(|e| Some((e.1, e.2)) != first) {
            tracing::debug!(?entries, "buffers returned samples in different calls");
        }
        // A buffer that never reported does not hold the others back.
        let ready = self.ends.values().copied().min()?;
        (ready > self.given).then(|| {
            let range = (self.given, ready - self.given);
            self.given = ready;
            range
        })
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
    fn samples_go_out_once_every_buffer_holds_them() {
        let (a, port) = (0, 128);
        let mut j = Joined::default();
        // Both at once.
        assert_eq!(j.join(&[(a, 0, 100), (port, 0, 100)]), Some((0, 100)));
        // A first; the port's come in the next call.
        assert_eq!(j.join(&[(a, 100, 50), (port, 0, 0)]), None);
        assert_eq!(j.join(&[(a, 0, 0), (port, 100, 50)]), Some((100, 50)));
        // The port ahead of A.
        assert_eq!(j.join(&[(a, 150, 10), (port, 150, 30)]), Some((150, 10)));
        assert_eq!(j.join(&[(a, 160, 20), (port, 0, 0)]), Some((160, 20)));
        assert_eq!(j.join(&[(a, 0, 0), (port, 0, 0)]), None);

        // A buffer that never reported holds nothing back.
        let mut j = Joined::default();
        assert_eq!(j.join(&[(a, 0, 0), (port, 0, 40)]), Some((0, 40)));
    }

    #[test]
    fn new_buffers_start_again_at_zero() {
        let ports = StreamPorts::default();
        assert_eq!(ports.join(1, &[(0, 0, 100)]), Some((0, 100)));
        ports.buffers_set(1);
        assert_eq!(ports.join(1, &[(0, 0, 10)]), Some((0, 10)));
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
