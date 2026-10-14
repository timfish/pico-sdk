use pico_common::{PicoChannel, PicoCoupling, PicoError, PicoRange, PicoResult};

/// Channel features beyond range and coupling. The bandwidth limit is
/// part of `OscilloscopeChannelConfig` instead, as some families take it
/// in their channel call.
pub trait ChannelExtrasDriver {
    /// The lowest and highest analogue offset in volts at `range` and
    /// `coupling`.
    fn offset_limits(
        &self,
        _handle: i16,
        _range: PicoRange,
        _coupling: PicoCoupling,
    ) -> PicoResult<(f64, f64)> {
        Err(PicoError::not_supported("offset_limits"))
    }
    /// Channels whose overvoltage protection tripped.
    fn overvoltage_tripped(&self, _handle: i16) -> PicoResult<Vec<PicoChannel>> {
        Err(PicoError::not_supported("overvoltage_tripped"))
    }
    fn reset_overvoltage(&self, _handle: i16) -> PicoResult<()> {
        Err(PicoError::not_supported("reset_overvoltage"))
    }
}
