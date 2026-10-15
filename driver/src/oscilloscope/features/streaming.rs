use crate::oscilloscope::SampleBuffer;
use pico_common::{
    Downsample, DownsampleKind, OscilloscopeSampleConfig, Overflow, PicoChannel, PicoResult,
    SampleType,
};

/// Continuous streaming. Every family has it.
pub trait StreamingDriver {
    /// The streaming downsample modes the driver has.
    fn downsample_modes(&self) -> &'static [DownsampleKind] {
        &[DownsampleKind::Average]
    }
    /// The sample types the driver writes into buffers. Drivers of 8-bit
    /// units without `I8` write 8-bit codes shifted left by 8 in `I16`.
    fn sample_types(&self) -> &'static [SampleType] {
        &[SampleType::I16]
    }
    /// Give the driver a buffer to write data into, for a stream with
    /// `downsample`
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<()>;
    /// Starts the device streaming. `sample_config` is the device's
    /// interval, before any downsample.
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &OscilloscopeSampleConfig,
        enabled_channels: u8,
        buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<OscilloscopeSampleConfig>;
    /// Gets the latest streaming values. `callback` gets the start index
    /// and count of new samples in the buffers, and the channels whose
    /// input went over range in them.
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        channels: &[PicoChannel],
        callback: Box<dyn FnMut(usize, usize, Overflow) + 'a>,
    ) -> PicoResult<()>;
}

/// The channels flagged over range in a `PICO_STREAMING_DATA_INFO` list
/// of `(channel_, overflow_)`. Each entry's `overflow_` is 1 or 0 for its
/// own channel (ps6000a guide §3.16.1, psospa §3.12.1). Entries of
/// digital ports are not channels and are left out.
pub(crate) fn flagged_channels(entries: &[(u32, i16)]) -> Overflow {
    entries
        .iter()
        .filter(|(code, flag)| *flag != 0 && *code < 8)
        .map(|(code, _)| PicoChannel::from(*code))
        .fold(Overflow::default(), Overflow::with)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_entry_flags_its_own_channel() {
        let port0 = 128;
        assert_eq!(
            flagged_channels(&[(0, 0), (1, 1), (port0, 1)]),
            Overflow(0b10)
        );
        assert_eq!(flagged_channels(&[(3, 1), (0, 1)]), Overflow(0b1001));
        assert!(flagged_channels(&[(0, 0), (2, 0)]).is_empty());
        assert!(flagged_channels(&[]).is_empty());
    }
}
