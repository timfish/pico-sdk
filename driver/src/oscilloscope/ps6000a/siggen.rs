use super::PS6000ADriver;
use crate::oscilloscope::{
    siggen::properties::{self, Applied, PropertyCalls},
    SigGenDriver,
};
use pico_common::{PicoResult, PicoStatus, SigGenConfig, SigGenInfo};

/// §3.73.1: the AWG's DAC runs at 200 MHz unless set by hand.
const DAC_HZ: f64 = 200e6;

impl PropertyCalls for PS6000ADriver {
    fn waveform(&self, handle: i16, wave: u32, buffer: &mut [i16]) -> PicoStatus {
        let ptr = if buffer.is_empty() {
            std::ptr::null_mut()
        } else {
            buffer.as_mut_ptr()
        };
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aSigGenWaveform(handle, wave, ptr, buffer.len() as u64)
        })
    }

    fn range(&self, handle: i16, pk_to_pk_v: f64, offset_v: f64) -> PicoStatus {
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aSigGenRange(handle, pk_to_pk_v, offset_v)
        })
    }

    fn duty_cycle(&self, handle: i16, percent: f64) -> PicoStatus {
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aSigGenWaveformDutyCycle(handle, percent)
        })
    }

    // autoTriggerPicoSeconds is reserved and must be 0.
    fn trigger(&self, handle: i16, kind: u32, source: u32, cycles: u64) -> PicoStatus {
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aSigGenTrigger(handle, kind, source, cycles, 0)
        })
    }

    fn frequency(&self, handle: i16, hz: f64) -> PicoStatus {
        PicoStatus::from(unsafe { self.bindings.ps6000aSigGenFrequency(handle, hz) })
    }

    fn frequency_sweep(
        &self,
        handle: i16,
        stop_hz: f64,
        step_hz: f64,
        dwell_s: f64,
        kind: u32,
    ) -> PicoStatus {
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aSigGenFrequencySweep(handle, stop_hz, step_hz, dwell_s, kind)
        })
    }

    fn limits(&self, handle: i16, param: u32) -> PicoResult<(f64, f64, f64)> {
        let (mut least, mut most, mut step) = (0.0, 0.0, 0.0);
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aSigGenLimits(handle, param, &mut least, &mut most, &mut step)
        })
        .to_result((least, most, step), "sig_gen_limits")
    }

    fn software_trigger(&self, handle: i16, state: u32) -> PicoStatus {
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aSigGenSoftwareTriggerControl(handle, state)
        })
    }

    fn pause(&self, handle: i16) -> PicoStatus {
        PicoStatus::from(unsafe { self.bindings.ps6000aSigGenPause(handle) })
    }

    fn restart(&self, handle: i16) -> PicoStatus {
        PicoStatus::from(unsafe { self.bindings.ps6000aSigGenRestart(handle) })
    }

    fn dac_hz(&self) -> Option<f64> {
        Some(DAC_HZ)
    }

    fn filter(&self, handle: i16, state: u32) -> Option<PicoStatus> {
        Some(PicoStatus::from(unsafe {
            self.bindings.ps6000aSigGenFilter(handle, state)
        }))
    }

    fn has_filter(&self) -> bool {
        true
    }

    // The driver picks the DAC clock and prescaler (§3.65).
    fn apply(&self, handle: i16, on: bool, sweep: bool, trigger: bool) -> PicoResult<Applied> {
        let mut out = Applied::default();
        PicoStatus::from(unsafe {
            self.bindings.ps6000aSigGenApply(
                handle,
                i16::from(on),
                i16::from(sweep),
                i16::from(trigger),
                1,
                0,
                &mut out.frequency_hz,
                &mut out.stop_hz,
                &mut out.increment_hz,
                &mut out.dwell_s,
            )
        })
        .to_result(out, "sig_gen_apply")
    }

    // §3.69 gives no lowest frequency: the smallest sweep step from a
    // 1 kHz start stands in for it, and the highest stop frequency for
    // the highest.
    fn frequency_limits(&self, handle: i16, wave: u32, samples: u64) -> PicoResult<(f64, f64)> {
        let mut samples = samples;
        let samples_ptr = if samples == 0 {
            std::ptr::null_mut()
        } else {
            &mut samples as *mut u64
        };
        let mut start = 1000.0;
        let (mut most_stop, mut least_step, mut most_step, mut least_dwell, mut most_dwell) =
            (0.0, 0.0, 0.0, 0.0, 0.0);
        PicoStatus::from(unsafe {
            self.bindings.ps6000aSigGenFrequencyLimits(
                handle,
                wave,
                samples_ptr,
                &mut start,
                1,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut most_stop,
                &mut least_step,
                &mut most_step,
                &mut least_dwell,
                &mut most_dwell,
            )
        })
        .to_result((least_step, most_stop), "sig_gen_frequency_limits")
    }
}

impl SigGenDriver for PS6000ADriver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn sig_gen_info(&self, handle: i16) -> PicoResult<SigGenInfo> {
        properties::info(self, handle)
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn sig_gen_apply(
        &self,
        handle: i16,
        config: Option<&SigGenConfig>,
    ) -> PicoResult<Option<SigGenConfig>> {
        properties::apply(self, handle, config)
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn sig_gen_software_trigger(&self, handle: i16, state: bool) -> PicoResult<()> {
        properties::software_trigger(self, handle, state)
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn sig_gen_pause(&self, handle: i16) -> PicoResult<()> {
        self.pause(handle).to_result((), "sig_gen_pause")
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn sig_gen_restart(&self, handle: i16) -> PicoResult<()> {
        self.restart(handle).to_result((), "sig_gen_restart")
    }
}
