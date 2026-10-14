use crate::oscilloscope::SampleBuffer;
use pico_common::{
    Downsample, DownsampleKind, OscilloscopeSampleConfig, PicoChannel, PicoResult, SampleType,
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
    /// Gets the latest streaming values
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        channels: &[PicoChannel],
        callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoResult<()>;
}
