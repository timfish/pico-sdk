use pico_common::{DigitalPort, DigitalPortConfig, PicoError, PicoResult};

/// The digital inputs of MSO models. Data comes in the analogue
/// buffers, one per port, the port's 8 bits in the low byte.
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
}
