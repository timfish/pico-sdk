use super::PS5000ADriver;
use crate::oscilloscope::{features::offset_limits_f32, ChannelExtrasDriver};
use pico_common::{PicoCoupling, PicoRange, PicoResult};

impl ChannelExtrasDriver for PS5000ADriver {
    // ps5000apg.en r4 §4.7.
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn offset_limits(
        &self,
        handle: i16,
        range: PicoRange,
        coupling: PicoCoupling,
    ) -> PicoResult<(f64, f64)> {
        offset_limits_f32("offset_limits", |max, min| unsafe {
            self.bindings
                .ps5000aGetAnalogueOffset(handle, range.into(), coupling.into(), max, min)
        })
    }
}
