use crate::oscilloscope::SampleBuffer;
use pico_common::{
    Downsample, DownsampleKind, OscilloscopeSampleConfig, Overflow, PicoChannel, PicoResult,
    SampleType,
};

/// Continuous streaming, which every range has
pub trait StreamingDriver {
    /// The downsample modes that a stream takes
    fn downsample_modes(&self) -> &'static [DownsampleKind] {
        &[DownsampleKind::Average]
    }
    /// The sample types that the driver writes into buffers
    ///
    /// - A driver of 8-bit units that does not list `I8` writes each 8-bit value shifted left by 8 bits into an `I16` buffer.
    fn sample_types(&self) -> &'static [SampleType] {
        &[SampleType::I16]
    }
    /// Gives the driver the buffer of `channel` for a stream with `downsample`
    ///
    /// - Give a buffer for each channel that is on before [`StreamingDriver::start_streaming`].
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<()>;
    /// Starts the stream
    ///
    /// - `sample_config` gives the interval of the unit, before any downsample.
    /// - Returns the sample configuration that the driver set.
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &OscilloscopeSampleConfig,
        enabled_channels: u8,
        buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<OscilloscopeSampleConfig>;
    /// Gets the newest samples of the stream
    ///
    /// - `callback` gets the start index and the number of the new samples in the buffers, and the channels that went over range in them.
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
        .filter(|(_, flag)| *flag != 0)
        .filter_map(|(code, _)| PicoChannel::try_from(*code).ok())
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
