use crate::oscilloscope::SampleBuffer;
use pico_common::{
    BlockConfig, Downsample, DownsampleKind, Overflow, PicoChannel, PicoError, PicoResult,
    PicoStatus, Timebase,
};

/// Block mode: one capture around a trigger into the unit's memory,
/// read after it ends.
pub trait BlockDriver {
    /// The interval and most samples of timebase `index`, for a capture
    /// of `samples` into `segment`.
    fn timebase(
        &self,
        _handle: i16,
        _index: u32,
        _samples: u64,
        _segment: u64,
    ) -> PicoResult<Timebase> {
        Err(PicoError::not_supported("timebase"))
    }
    /// The timebase nearest to `interval_s` with `channels` on.
    fn nearest_timebase(
        &self,
        _handle: i16,
        _interval_s: f64,
        _channels: &[PicoChannel],
    ) -> PicoResult<Timebase> {
        Err(PicoError::not_supported("nearest_timebase"))
    }
    /// Give the driver a buffer for a block capture into `segment`.
    fn set_block_buffer(
        &self,
        _handle: i16,
        _channel: PicoChannel,
        _buffer: SampleBuffer,
        _segment: u64,
        _downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        Err(PicoError::not_supported("set_block_buffer"))
    }
    /// Arms a capture. `ready` runs when it ends. Returns the time in ms
    /// the unit will be busy collecting.
    fn run_block(
        &self,
        _handle: i16,
        _config: &BlockConfig,
        _ready: Box<dyn FnOnce(PicoStatus) + Send>,
    ) -> PicoResult<f64> {
        Err(PicoError::not_supported("run_block"))
    }
    /// Whether the armed capture has ended.
    fn is_ready(&self, _handle: i16) -> PicoResult<bool> {
        Err(PicoError::not_supported("is_ready"))
    }
    /// Reads `count` samples from `start` of `segment` into the buffers.
    /// Returns the count read.
    fn get_values(
        &self,
        _handle: i16,
        _start: u64,
        _count: u64,
        _downsample: Option<Downsample>,
        _segment: u64,
    ) -> PicoResult<(u64, Overflow)> {
        Err(PicoError::not_supported("get_values"))
    }
    /// The largest downsample ratio of `samples` in `segment`.
    fn max_downsample_ratio(
        &self,
        _handle: i16,
        _samples: u64,
        _mode: DownsampleKind,
        _segment: u64,
    ) -> PicoResult<u64> {
        Err(PicoError::not_supported("max_downsample_ratio"))
    }
}
