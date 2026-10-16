use crate::oscilloscope::SampleBuffer;
use pico_common::{
    status_from_raw, BlockConfig, DigitalPort, Downsample, DownsampleKind, Overflow, PicoChannel,
    PicoError, PicoResolution, PicoResult, PicoStatus, Timebase,
};
use std::ffi::c_void;

/// Runs once when an armed capture ends, with the driver's status.
pub type BlockReady = Box<dyn FnOnce(PicoStatus) + Send>;

/// Block mode: one capture around a trigger into the unit's memory,
/// read after it ends.
pub trait BlockDriver {
    /// The downsample modes `get_values` takes. Empty where the family
    /// reads raw samples only.
    fn block_downsample_modes(&self) -> &'static [DownsampleKind] {
        &[]
    }
    /// The interval and most samples of timebase `index`, for a capture
    /// of `samples` into `segment`, with the channels and resolution the
    /// unit has now.
    fn timebase(
        &self,
        _handle: i16,
        _index: u32,
        _samples: u64,
        _segment: u64,
    ) -> PicoResult<Timebase> {
        Err(PicoError::not_supported("timebase"))
    }
    /// The highest timebase index of the family.
    fn max_timebase(&self) -> u32 {
        u32::MAX
    }
    /// The timebase nearest to `interval_s` with `channels` and `ports`
    /// on at `resolution`. Families with a stateless call use the
    /// arguments;
    /// the others search `timebase`, which reads the channels and
    /// resolution the unit has now, so set those first.
    fn nearest_timebase(
        &self,
        handle: i16,
        interval_s: f64,
        _channels: &[PicoChannel],
        _ports: &[DigitalPort],
        _resolution: Option<PicoResolution>,
    ) -> PicoResult<Timebase> {
        search_timebase(
            |index| self.timebase(handle, index, 1, 0),
            interval_s,
            self.max_timebase(),
        )
    }
    /// Give the driver a buffer for reads of `segment` with
    /// `downsample`. An aggregate read needs `min` for the lows; the
    /// highs go into `buffer`.
    fn set_block_buffer(
        &self,
        _handle: i16,
        _channel: PicoChannel,
        _buffer: SampleBuffer,
        _min: Option<SampleBuffer>,
        _segment: u64,
        _downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        Err(PicoError::not_supported("set_block_buffer"))
    }
    /// Arms a capture. `ready` runs when it ends; with none, poll
    /// `is_ready`. Returns the time in ms the unit will be busy
    /// collecting.
    fn run_block(
        &self,
        _handle: i16,
        _config: &BlockConfig,
        _ready: Option<BlockReady>,
    ) -> PicoResult<f64> {
        Err(PicoError::not_supported("run_block"))
    }
    /// Whether the armed capture has ended.
    fn is_ready(&self, _handle: i16) -> PicoResult<bool> {
        Err(PicoError::not_supported("is_ready"))
    }
    /// Reads `count` raw samples from `start` of `segment` into the
    /// buffers, downsampled by `downsample`. Returns the values each
    /// buffer got: `count` raw, or one per group.
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
    /// Time in seconds from the trigger point to the trigger event of
    /// `segment`, below one sample interval.
    fn trigger_time_offset(&self, _handle: i16, _segment: u64) -> PicoResult<f64> {
        Err(PicoError::not_supported("trigger_time_offset"))
    }
}

/// The timebase whose interval is nearest to `interval_s`. Intervals
/// grow with the index; the fastest indexes can be refused for the
/// channels on, the slowest past the family's last.
pub(crate) fn search_timebase(
    mut timebase: impl FnMut(u32) -> PicoResult<Timebase>,
    interval_s: f64,
    max_index: u32,
) -> PicoResult<Timebase> {
    if interval_s.is_nan() || interval_s <= 0.0 {
        return Err(PicoError::from_status(
            PicoStatus::INVALID_TIMEBASE,
            "nearest_timebase",
        ));
    }
    let first = (0..=max_index.min(64))
        .find_map(|index| timebase(index).ok())
        .ok_or_else(|| PicoError::from_status(PicoStatus::INVALID_TIMEBASE, "nearest_timebase"))?;
    if first.interval_s >= interval_s {
        return Ok(first);
    }
    // The fastest valid index below `interval_s` stays in `low`; `high`
    // is valid and not below it, or past the last valid index.
    let mut low = first;
    let mut high = max_index as u64 + 1;
    let mut above: Option<Timebase> = None;
    while (low.index as u64) + 1 < high {
        let mid = ((low.index as u64 + high) / 2) as u32;
        match timebase(mid) {
            Ok(tb) if tb.interval_s < interval_s => low = tb,
            Ok(tb) => {
                high = mid as u64;
                above = Some(tb);
            }
            Err(_) => high = mid as u64,
        }
    }
    Ok(match above {
        Some(above) if above.index as u64 == high => {
            if above.interval_s - interval_s < interval_s - low.interval_s {
                above
            } else {
                low
            }
        }
        _ => low,
    })
}

/// A family's ratio mode codes. `None` for a mode it lacks.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RatioCodes {
    pub none: u32,
    pub aggregate: Option<u32>,
    pub decimate: Option<u32>,
    pub average: Option<u32>,
    pub sum: Option<u32>,
}

impl RatioCodes {
    /// The ratio and code of a block read with `downsample`.
    pub fn of(&self, downsample: Option<Downsample>) -> PicoResult<(u32, u32)> {
        let Some(downsample) = downsample else {
            return Ok((1, self.none));
        };
        if downsample.ratio == 0 {
            return Err(PicoError::from_status(
                PicoStatus::INVALID_SAMPLERATIO,
                "downsample",
            ));
        }
        let code = match downsample.mode {
            DownsampleKind::Aggregate => self.aggregate,
            DownsampleKind::Decimate => self.decimate,
            DownsampleKind::Average => self.average,
            DownsampleKind::Sum => self.sum,
        };
        code.map(|code| (downsample.ratio, code)).ok_or_else(|| {
            PicoError::from_status(
                PicoStatus::RATIO_MODE_NOT_SUPPORTED,
                &format!("{:?} downsample", downsample.mode),
            )
        })
    }

    /// The modes the codes cover.
    #[cfg(test)]
    pub fn modes(&self) -> Vec<DownsampleKind> {
        [
            (self.aggregate, DownsampleKind::Aggregate),
            (self.decimate, DownsampleKind::Decimate),
            (self.average, DownsampleKind::Average),
            (self.sum, DownsampleKind::Sum),
        ]
        .into_iter()
        .filter_map(|(code, mode)| code.map(|_| mode))
        .collect()
    }
}

/// Checks a buffer pair against the downsample it is for: an aggregate
/// needs a buffer for the lows, the others take none.
pub(crate) fn check_min_buffer(
    min: &Option<SampleBuffer>,
    downsample: Option<Downsample>,
) -> PicoResult<()> {
    let aggregate = matches!(
        downsample,
        Some(Downsample {
            mode: DownsampleKind::Aggregate,
            ..
        })
    );
    if aggregate == min.is_some() {
        Ok(())
    } else {
        Err(PicoError::from_status(
            PicoStatus::NULL_PARAMETER,
            "an aggregate read takes a buffer for the lows; other reads take none",
        ))
    }
}

/// The block ready callback every family's header declares:
/// `(handle, status, parameter)`.
pub(crate) type ReadyFn = unsafe extern "C" fn(i16, u32, *mut c_void);

unsafe extern "C" fn block_ready(_handle: i16, status: u32, parameter: *mut c_void) {
    if parameter.is_null() {
        return;
    }
    let ready = Box::from_raw(parameter as *mut BlockReady);
    ready(status_from_raw(status));
}

/// The callback and parameter to pass to a family's run block call.
/// The driver calls back once; pass the parameter to
/// [`ReadyParameter::failed`] if the call fails, so it is freed. A
/// capture stopped before it ends may never call back, which leaves
/// the box unfreed.
pub(crate) struct ReadyParameter(*mut c_void);

impl ReadyParameter {
    pub fn new(ready: Option<BlockReady>) -> (Option<ReadyFn>, ReadyParameter) {
        match ready {
            Some(ready) => (
                Some(block_ready as ReadyFn),
                ReadyParameter(Box::into_raw(Box::new(ready)) as *mut c_void),
            ),
            None => (None, ReadyParameter(std::ptr::null_mut())),
        }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.0
    }

    /// Frees the callback after a run block call the driver refused.
    pub fn failed(self) {
        if !self.0.is_null() {
            drop(unsafe { Box::from_raw(self.0 as *mut BlockReady) });
        }
    }
}

/// The channel flags of the stateless timebase calls: bit 0 for
/// channel A, bit 16 for `PORT0` (ps5000apg r4 §4.35; ps6000a §3.30;
/// `PICO_PORT0_FLAGS` in the ps5000a, ps6000a and psospa headers).
pub(crate) fn channel_flags(channels: &[PicoChannel], ports: &[DigitalPort]) -> u32 {
    let channels = channels.iter().fold(0, |flags, ch| flags | 1 << *ch as u32);
    ports
        .iter()
        .fold(channels, |flags, port| flags | 1 << (16 + port.index()))
}

/// The resolution asked for, else the one the unit has now.
pub(crate) fn resolution_or(
    asked: Option<PicoResolution>,
    now: Option<PicoResolution>,
) -> PicoResult<PicoResolution> {
    asked.or(now).ok_or_else(|| {
        PicoError::from_status(PicoStatus::INVALID_DEVICE_RESOLUTION, "nearest_timebase")
    })
}

/// Converts a count for a family whose driver takes a narrower type.
pub(crate) fn narrow<T: TryFrom<u64>>(value: u64, what: &str) -> PicoResult<T> {
    T::try_from(value).map_err(|_| {
        PicoError::from_status(PicoStatus::TOO_MANY_SAMPLES, &format!("{what} {value}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    };

    /// Timebases that double up to index 3, then step by 8 ns, with 0
    /// refused as when more channels are on (the shape of ps2000a §2.7).
    fn fake(index: u32) -> PicoResult<Timebase> {
        let ns = match index {
            0 => return Err(PicoError::from_status(PicoStatus::INVALID_TIMEBASE, "")),
            1..=2 => 2f64.powi(index as i32),
            3..=1000 => (index - 2) as f64 * 8.0,
            _ => return Err(PicoError::from_status(PicoStatus::INVALID_TIMEBASE, "")),
        };
        Ok(Timebase::from_ns(index, ns, 1000))
    }

    #[test]
    fn the_search_finds_the_nearest_interval() {
        let at = |s: f64| search_timebase(fake, s, u32::MAX).unwrap().index;
        assert_eq!(at(1e-9), 1);
        assert_eq!(at(4e-9), 2);
        assert_eq!(at(8e-9), 3);
        assert_eq!(at(1e-6), 127);
        assert_eq!(at(1.003e-6), 127);
        assert_eq!(at(1.005e-6), 128);
        assert_eq!(at(1.0), 1000);
        assert!(search_timebase(fake, 0.0, u32::MAX).is_err());
        assert_eq!(channel_flags(&[PicoChannel::A, PicoChannel::C], &[]), 0b101);
        assert_eq!(
            channel_flags(&[PicoChannel::B], &[DigitalPort::Port1]),
            0x2_0002
        );
    }

    #[test]
    fn the_search_takes_few_calls() {
        let calls = AtomicU32::new(0);
        search_timebase(
            |i| {
                calls.fetch_add(1, Ordering::Relaxed);
                fake(i)
            },
            1e-6,
            u32::MAX,
        )
        .unwrap();
        assert!(calls.load(Ordering::Relaxed) < 40);
    }

    #[test]
    fn ratio_codes_refuse_modes_the_family_lacks() {
        let codes = RatioCodes {
            none: 0,
            aggregate: Some(1),
            decimate: None,
            average: Some(2),
            sum: None,
        };
        assert_eq!(codes.of(None).unwrap(), (1, 0));
        assert_eq!(codes.of(Some(Downsample::aggregate(8))).unwrap(), (8, 1));
        assert_eq!(
            codes.of(Some(Downsample::decimate(8))).unwrap_err().status,
            PicoStatus::RATIO_MODE_NOT_SUPPORTED
        );
        assert_eq!(
            codes.of(Some(Downsample::average(0))).unwrap_err().status,
            PicoStatus::INVALID_SAMPLERATIO
        );
    }

    #[test]
    fn aggregate_reads_need_a_buffer_for_the_lows() {
        let buffer = Some(SampleBuffer::new(pico_common::SampleType::I16, 4));
        assert!(check_min_buffer(&buffer, Some(Downsample::aggregate(2))).is_ok());
        assert!(check_min_buffer(&None, Some(Downsample::aggregate(2))).is_err());
        assert!(check_min_buffer(&buffer, None).is_err());
        assert!(check_min_buffer(&None, Some(Downsample::average(2))).is_ok());
    }

    #[test]
    fn the_ready_callback_runs_once_with_the_status() {
        let seen = Arc::new(AtomicU32::new(u32::MAX));
        let s = seen.clone();
        let (callback, parameter) = ReadyParameter::new(Some(Box::new(move |status| {
            s.store(status as u32, Ordering::SeqCst);
        })));
        unsafe { callback.unwrap()(1, 0, parameter.as_ptr()) };
        assert_eq!(seen.load(Ordering::SeqCst), 0);

        let (callback, parameter) = ReadyParameter::new(None);
        assert!(callback.is_none() && parameter.as_ptr().is_null());
        let (_, parameter) = ReadyParameter::new(Some(Box::new(|_| {})));
        parameter.failed();
    }
}
