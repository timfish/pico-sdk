use super::PSOSPADriver;
use crate::oscilloscope::{
    features::tripped_channels, variant_channel_count, ChannelExtrasDriver,
    OscilloscopeDriverInternal,
};
use pico_common::{PicoChannel, PicoCoupling, PicoInfo, PicoRange, PicoResult, PicoStatus};
use pico_sys_dynamic::psospa::{PICO_CHANNEL_OVERVOLTAGE_TRIPPED, PICO_STATUS};

impl PSOSPADriver {
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

// psospa guide (online edition) §3.5, §3.33 and §3.34.
impl ChannelExtrasDriver for PSOSPADriver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn offset_limits(
        &self,
        handle: i16,
        range: PicoRange,
        coupling: PicoCoupling,
    ) -> PicoResult<(f64, f64)> {
        let (mut max, mut min) = (0f64, 0f64);
        PicoStatus::from(unsafe {
            self.bindings.psospaGetAnalogueOffsetLimits(
                handle,
                -range.to_nano_volts(),
                range.to_nano_volts(),
                range.to_probe_range(),
                coupling.into(),
                &mut max,
                &mut min,
            )
        })
        .to_result((min, max), "offset_limits")
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn overvoltage_tripped(&self, handle: i16) -> PicoResult<Vec<PicoChannel>> {
        self.overvoltage_flags(handle, "overvoltage_tripped", |flags, n| unsafe {
            self.bindings
                .psospaReportAllChannelsOvervoltageTripStatus(handle, flags, n)
        })
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn reset_overvoltage(&self, handle: i16) -> PicoResult<Vec<PicoChannel>> {
        self.overvoltage_flags(handle, "reset_overvoltage", |flags, n| unsafe {
            self.bindings
                .psospaResetChannelsAndReportAllChannelsOvervoltageTripStatus(handle, flags, n)
        })
    }
}
