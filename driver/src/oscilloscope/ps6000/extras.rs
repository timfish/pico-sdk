use super::PS6000Driver;
use crate::oscilloscope::{features::offset_limits_f32, ChannelExtrasDriver};
use pico_common::{PicoCoupling, PicoRange, PicoResult};

impl ChannelExtrasDriver for PS6000Driver {
    // PicoScope 6000 Series Programmer's Guide §3.6.
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn offset_limits(
        &self,
        handle: i16,
        range: PicoRange,
        coupling: PicoCoupling,
    ) -> PicoResult<(f64, f64)> {
        offset_limits_f32("offset_limits", |max, min| unsafe {
            self.bindings
                .ps6000GetAnalogueOffset(handle, range.into(), coupling.into(), max, min)
        })
    }
}
