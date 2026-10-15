use super::PS6000ADriver;
use crate::oscilloscope::{
    features::{accessory_string, tripped_channels},
    variant_channel_count, ChannelExtrasDriver, OscilloscopeDriverInternal,
};
use pico_common::{PicoChannel, PicoCoupling, PicoInfo, PicoRange, PicoResult, PicoStatus};
use pico_sys_dynamic::ps6000a::{PICO_CHANNEL_OVERVOLTAGE_TRIPPED, PICO_STATUS};

impl PS6000ADriver {
    /// One entry per analog channel, each naming its channel.
    fn overvoltage_flags(
        &self,
        handle: i16,
        context: &str,
        call: impl FnOnce(*mut PICO_CHANNEL_OVERVOLTAGE_TRIPPED, u8) -> PICO_STATUS,
    ) -> PicoResult<Vec<PicoChannel>> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let channels = variant_channel_count(&variant).unwrap_or(8).min(8);
        let mut flags: Vec<PICO_CHANNEL_OVERVOLTAGE_TRIPPED> = (0..channels)
            .map(|c| PICO_CHANNEL_OVERVOLTAGE_TRIPPED {
                channel_: c as u32,
                tripped_: 0,
            })
            .collect();
        PicoStatus::from(call(flags.as_mut_ptr(), channels)).to_result((), context)?;
        Ok(tripped_channels(
            flags.iter().map(|f| (f.channel_, f.tripped_)),
        ))
    }
}

impl ChannelExtrasDriver for PS6000ADriver {
    // ps6000apg-9 §3.8.
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn offset_limits(
        &self,
        handle: i16,
        range: PicoRange,
        coupling: PicoCoupling,
    ) -> PicoResult<(f64, f64)> {
        let (mut max, mut min) = (0f64, 0f64);
        PicoStatus::from(unsafe {
            self.bindings.ps6000aGetAnalogueOffsetLimits(
                handle,
                range.into(),
                coupling.into(),
                &mut max,
                &mut min,
            )
        })
        .to_result((min, max), "offset_limits")
    }

    // ps6000apg-9 §3.39; not the 6428E-D.
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn overvoltage_tripped(&self, handle: i16) -> PicoResult<Vec<PicoChannel>> {
        self.overvoltage_flags(handle, "overvoltage_tripped", |flags, n| unsafe {
            self.bindings
                .ps6000aReportAllChannelsOvervoltageTripStatus(handle, flags, n)
        })
    }

    // ps6000apg-9 §3.38.
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn reset_overvoltage(&self, handle: i16) -> PicoResult<Vec<PicoChannel>> {
        self.overvoltage_flags(handle, "reset_overvoltage", |flags, n| unsafe {
            self.bindings
                .ps6000aResetChannelsAndReportAllChannelsOvervoltageTripStatus(handle, flags, n)
        })
    }

    // ps6000apg-9 §3.6.
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn accessory_info(
        &self,
        handle: i16,
        channel: PicoChannel,
        info: PicoInfo,
    ) -> PicoResult<String> {
        accessory_string("accessory_info", |buffer, len, required| unsafe {
            self.bindings.ps6000aGetAccessoryInfo(
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
