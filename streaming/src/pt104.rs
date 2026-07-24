//! Streaming stub for the USB PT-104 platinum resistance temperature data logger.
//!
//! No behavior yet - [`crate::StreamDevice`] will be implemented once the poll/stream model is
//! read from the PicoLog 6 reference (`packages/driver-pt-104/`, see
//! `pico-high-level-drivers-plan.md` in the daq-db repo). The reference's `driver.ts` exposes a
//! `pollingRequired` flag for this device - read it to confirm streaming vs poll-only before
//! filling this in; delete this file if the device turns out to be poll-only. USB only for now,
//! the Ethernet discovery path is out of scope.

/// Placeholder stream event for the PT-104. Shape lands with the real streaming implementation.
#[derive(Clone, Debug)]
pub struct PT104StreamingEvent;
