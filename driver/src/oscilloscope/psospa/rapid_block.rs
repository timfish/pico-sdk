//! Rapid block (psospa guide, topics: rapidblockmode,
//! usingrapidblockmode, psospamemorysegments, psospagetvaluesbulk,
//! psospagettriggerinfo, pico_trigger_info).

use super::*;
use crate::oscilloscope::{
    features::{offsets_only, overflows, segment_count, since_first, CounterEntry},
    RapidBlockDriver,
};
use pico_common::{Overflow, TriggerInfo};
use pico_sys_dynamic::psospa::PICO_TRIGGER_INFO;

impl RapidBlockDriver for PSOSPADriver {
    fn memory_segments(&self, handle: i16, segments: u64) -> PicoResult<u64> {
        let mut samples = 0u64;
        PicoStatus::from(unsafe {
            self.bindings
                .psospaMemorySegments(handle, segments, &mut samples)
        })
        .to_result(samples, "memory_segments")
    }

    fn set_captures(&self, handle: i16, captures: u64) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaSetNoOfCaptures(handle, captures) })
            .to_result((), "set_captures")
    }

    fn captures_done(&self, handle: i16) -> PicoResult<(u64, u64)> {
        let mut captured = 0u64;
        let mut processed = 0u64;
        PicoStatus::from(unsafe { self.bindings.psospaGetNoOfCaptures(handle, &mut captured) })
            .to_result((), "captures_done")?;
        PicoStatus::from(unsafe {
            self.bindings
                .psospaGetNoOfProcessedCaptures(handle, &mut processed)
        })
        .to_result((captured, processed), "captures_done")
    }

    fn set_bulk_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        min: Option<SampleBuffer>,
        segment: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        self.set_buffers(handle, channel, buffer, min, segment, downsample)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_values_bulk(
        &self,
        handle: i16,
        count: u64,
        from: u64,
        to: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<(u64, Vec<Overflow>)> {
        let mode = super::block::block_mode(downsample, SampleType::I16)?;
        let mut samples = count;
        let mut overflow = vec![0i16; segment_count(from, to)?];
        PicoStatus::from(unsafe {
            self.bindings.psospaGetValuesBulk(
                handle,
                0,
                &mut samples,
                from,
                to,
                mode.ratio,
                mode.mode,
                overflow.as_mut_ptr(),
            )
        })
        .to_result((), "get_values_bulk")?;
        Ok((samples, overflows(&overflow)))
    }

    fn trigger_info(&self, handle: i16, from: u64, to: u64) -> PicoResult<Vec<TriggerInfo>> {
        let n = segment_count(from, to)?;
        let mut times = vec![0i64; n];
        let mut units = vec![0u32; n];
        PicoStatus::from(unsafe {
            self.bindings.psospaGetValuesTriggerTimeOffsetBulk(
                handle,
                times.as_mut_ptr(),
                units.as_mut_ptr(),
                from,
                to,
            )
        })
        .to_result((), "trigger_info")?;
        let offsets = offsets_only(from, &times, &units);

        let blank = PICO_TRIGGER_INFO {
            status: 0,
            segmentIndex: 0,
            triggerIndex: 0,
            triggerTime: 0.0,
            timeUnits: 0,
            missedTriggers: 0,
            timeStampCounter: 0,
        };
        let mut raw = vec![blank; n];
        PicoStatus::from(unsafe {
            self.bindings
                .psospaGetTriggerInfo(handle, raw.as_mut_ptr(), from, n as u64)
        })
        .to_result((), "trigger_info")?;
        let entries: Vec<CounterEntry> = raw
            .iter()
            .map(|e| CounterEntry {
                status: e.status,
                trigger_index: Some(e.triggerIndex),
                counter: e.timeStampCounter,
                missed: Some(e.missedTriggers),
                ..Default::default()
            })
            .collect();
        Ok(since_first(offsets, &entries))
    }
}
