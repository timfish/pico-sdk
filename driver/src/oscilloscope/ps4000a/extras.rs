use super::PS4000ADriver;
use crate::oscilloscope::{
    features::{accessory_string, offset_limits_f32},
    ChannelExtrasDriver,
};
use pico_common::{PicoChannel, PicoCoupling, PicoInfo, PicoRange, PicoResult};

// The online 4000A guide; GetAccessoryInfo is in the header only.
impl ChannelExtrasDriver for PS4000ADriver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn offset_limits(
        &self,
        handle: i16,
        range: PicoRange,
        coupling: PicoCoupling,
    ) -> PicoResult<(f64, f64)> {
        offset_limits_f32("offset_limits", |max, min| unsafe {
            self.bindings
                .ps4000aGetAnalogueOffset(handle, range.into(), coupling.into(), max, min)
        })
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn accessory_info(
        &self,
        handle: i16,
        channel: PicoChannel,
        info: PicoInfo,
    ) -> PicoResult<String> {
        accessory_string("accessory_info", |buffer, len, required| unsafe {
            self.bindings.ps4000aGetAccessoryInfo(
                handle,
                channel.into(),
                buffer,
                len,
                required,
                info.into(),
            )
        })
    }
}
