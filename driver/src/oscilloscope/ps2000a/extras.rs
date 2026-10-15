use super::PS2000ADriver;
use crate::oscilloscope::{features::offset_limits_f32, ChannelExtrasDriver};
use pico_common::{PicoCoupling, PicoRange, PicoResult};

impl ChannelExtrasDriver for PS2000ADriver {
    // ps2000apg.en-12 §3.6.
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn offset_limits(
        &self,
        handle: i16,
        range: PicoRange,
        coupling: PicoCoupling,
    ) -> PicoResult<(f64, f64)> {
        offset_limits_f32("offset_limits", |max, min| unsafe {
            self.bindings
                .ps2000aGetAnalogueOffset(handle, range.into(), coupling.into(), max, min)
        })
    }
}
