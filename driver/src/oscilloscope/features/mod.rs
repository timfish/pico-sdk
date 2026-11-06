//! One trait for each oscilloscope feature
//!
//! - Each range implements each trait.
//! - A method that the range or the model does not have returns [`PicoError::not_supported`](pico_common::PicoError::not_supported).
//! - [`OscilloscopeDriverInternal::capabilities`](super::OscilloscopeDriverInternal::capabilities) tells which features a unit has.

mod aux_io;
mod block;
mod digital;
mod ets;
mod extras;
mod led;
mod rapid_block;
mod siggen;
mod streaming;
pub(crate) mod trigger;

pub use aux_io::AuxIoDriver;
pub(crate) use block::{
    channel_flags, check_min_buffer, narrow, resolution_or, RatioCodes, ReadyParameter,
};
pub use block::{BlockDriver, BlockReady};
pub use digital::DigitalPortDriver;
pub(crate) use digital::{no_hysteresis, raw_ports_only, StreamPorts, PORT_VOLTS};
pub use ets::EtsDriver;
pub use extras::ChannelExtrasDriver;
pub use led::LedDriver;
pub use rapid_block::RapidBlockDriver;
pub(crate) use rapid_block::{
    offsets_only, overflows, segment_count, since_first, since_previous, CounterEntry,
    SinceFirstEntry,
};
pub use siggen::SigGenDriver;
pub(crate) use streaming::flagged_channels;
pub use streaming::StreamingDriver;
pub use trigger::TriggerDriver;

pub(crate) use ets::{ets_args, ets_buffer_len};
pub(crate) use extras::{accessory_string, offset_limits_f32, tripped_channels};
