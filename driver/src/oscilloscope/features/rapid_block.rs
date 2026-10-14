use pico_common::{Downsample, Overflow, PicoError, PicoResult, TriggerInfo};

/// Rapid block: many block captures into memory segments with a short
/// rearm time, and the trigger timing of each.
pub trait RapidBlockDriver {
    /// Splits the unit's memory into `segments`. Returns the samples a
    /// segment holds.
    fn memory_segments(&self, _handle: i16, _segments: u64) -> PicoResult<u64> {
        Err(PicoError::not_supported("memory_segments"))
    }
    fn max_segments(&self, _handle: i16) -> PicoResult<u64> {
        Err(PicoError::not_supported("max_segments"))
    }
    fn set_captures(&self, _handle: i16, _captures: u64) -> PicoResult<()> {
        Err(PicoError::not_supported("set_captures"))
    }
    /// Captures done and captures processed.
    fn captures_done(&self, _handle: i16) -> PicoResult<(u64, u64)> {
        Err(PicoError::not_supported("captures_done"))
    }
    /// Reads `count` samples of segments `from` to `to` into the
    /// buffers given for them.
    fn get_values_bulk(
        &self,
        _handle: i16,
        _count: u64,
        _from: u64,
        _to: u64,
        _downsample: Option<Downsample>,
    ) -> PicoResult<Vec<Overflow>> {
        Err(PicoError::not_supported("get_values_bulk"))
    }
    /// The trigger timing of segments `from` to `to`, as far as the
    /// family gives it.
    fn trigger_info(&self, _handle: i16, _from: u64, _to: u64) -> PicoResult<Vec<TriggerInfo>> {
        Err(PicoError::not_supported("trigger_info"))
    }
}
