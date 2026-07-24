//! Streaming stub for the ADC-20/ADC-24 (PicoHRDL) data logger.
//!
//! No behavior yet - [`crate::StreamDevice`] will be implemented once the poll/stream model is
//! read from the PicoLog 6 reference (`packages/driver-hrdl/`, see
//! `pico-high-level-drivers-plan.md` in the daq-db repo) and it is confirmed whether this device
//! streams or is poll-only. Kept here so [`crate`] exports a stream event type per family from
//! day one; delete this file if the device turns out to be poll-only.

/// Placeholder stream event for the PicoHRDL. Shape lands with the real streaming implementation.
#[derive(Clone, Debug)]
pub struct HRDLStreamingEvent;
