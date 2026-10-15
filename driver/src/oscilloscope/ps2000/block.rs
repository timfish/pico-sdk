//! Block mode (ps2000pg.en-12 §3.1): timebase §5.6, run block §5.17,
//! ready §5.16, values §5.9. One segment, no downsample, no ready
//! callback; `ps2000_get_values` fills every channel's buffer at once,
//! from the first sample. The trigger delay of `ps2000_set_trigger`
//! places the trigger, so a block here has no pre-trigger samples.

use super::*;
use crate::oscilloscope::{features::narrow, BlockDriver, BlockReady};
use pico_common::{BlockConfig, Overflow, Timebase};
use pico_sys_dynamic::ps2000::PS2200_MAX_TIMEBASE;

lazy_static! {
    /// The buffers of each handle's block reads, filled together by one
    /// `ps2000_get_values` call.
    static ref BLOCK_BUFFERS: Mutex<HashMap<i16, BufferMap>> = Default::default();
}

impl BlockDriver for PS2000Driver {
    #[tracing::instrument(level = "trace", skip(self))]
    fn timebase(
        &self,
        handle: i16,
        index: u32,
        samples: u64,
        segment: u64,
    ) -> PicoResult<Timebase> {
        if segment != 0 {
            return Err(PicoError::not_supported("ps2000 segments"));
        }
        let mut interval_ns = 0i32;
        let mut units = 0i16;
        let mut max_samples = 0i32;
        PicoStatus::from(unsafe {
            self.bindings.ps2000_get_timebase(
                handle,
                narrow(index as u64, "timebase")?,
                narrow(samples, "samples")?,
                &mut interval_ns,
                &mut units,
                1,
                &mut max_samples,
            )
        })
        .to_result((), "timebase")?;
        Ok(Timebase::from_ns(
            index,
            interval_ns as f64,
            max_samples.max(0) as u64,
        ))
    }

    fn max_timebase(&self) -> u32 {
        PS2200_MAX_TIMEBASE
    }

    fn set_block_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        min: Option<SampleBuffer>,
        segment: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        refuse_downsample(downsample)?;
        if min.is_some() || segment != 0 {
            return Err(PicoError::not_supported("ps2000 segments and aggregates"));
        }
        let buffer = buffer.as_i16("set_block_buffer")?.clone();
        BLOCK_BUFFERS
            .lock()
            .entry(handle)
            .or_default()
            .insert(channel, buffer);
        Ok(())
    }

    /// The ps2000 has no ready callback, so `ready` is dropped unrun;
    /// poll `is_ready`.
    #[tracing::instrument(level = "trace", skip(self, _ready))]
    fn run_block(
        &self,
        handle: i16,
        config: &BlockConfig,
        _ready: Option<BlockReady>,
    ) -> PicoResult<f64> {
        if config.pre_trigger != 0 || config.segment != 0 {
            return Err(PicoError::not_supported(
                "ps2000 pre-trigger samples and segments",
            ));
        }
        let mut indisposed_ms = 0i32;
        PicoStatus::from(unsafe {
            self.bindings.ps2000_run_block(
                handle,
                narrow(config.post_trigger, "samples")?,
                narrow(config.timebase as u64, "timebase")?,
                1,
                &mut indisposed_ms,
            )
        })
        .to_result(indisposed_ms as f64, "run_block")
    }

    fn is_ready(&self, handle: i16) -> PicoResult<bool> {
        match unsafe { self.bindings.ps2000_ready(handle) } {
            ready if ready > 0 => Ok(true),
            0 => Ok(false),
            _ => Err(PicoError::from_status(PicoStatus::NOT_FOUND, "is_ready")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_values(
        &self,
        handle: i16,
        start: u64,
        count: u64,
        downsample: Option<Downsample>,
        segment: u64,
    ) -> PicoResult<(u64, Overflow)> {
        refuse_downsample(downsample)?;
        if start != 0 || segment != 0 {
            return Err(PicoError::not_supported(
                "ps2000 reads start at the first sample of the one segment",
            ));
        }
        let buffers = BLOCK_BUFFERS
            .lock()
            .get(&handle)
            .cloned()
            .unwrap_or_default();
        let mut guards: Vec<_> = (0..4)
            .map(|ch| {
                buffers
                    .get(&PicoChannel::from(ch as u32))
                    .map(|b| b.write())
            })
            .collect();
        let shortest = guards.iter().flatten().map(|g| g.len()).min().unwrap_or(0);
        let count: i32 = narrow(count.min(shortest as u64), "samples")?;
        let mut pointer = |ch: usize| {
            guards[ch]
                .as_mut()
                .map_or(std::ptr::null_mut(), |g| g.as_mut_ptr())
        };
        let (a, b, c, d) = (pointer(0), pointer(1), pointer(2), pointer(3));
        let mut overflow = 0i16;
        let read = unsafe {
            self.bindings
                .ps2000_get_values(handle, a, b, c, d, &mut overflow, count)
        };
        if read <= 0 {
            return Err(PicoError::from_status(
                PicoStatus::NO_SAMPLES_AVAILABLE,
                "get_values",
            ));
        }
        Ok((read as u64, Overflow(overflow as u16)))
    }
}
