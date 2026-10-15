//! One trait per scope feature. Every family implements every trait; a
//! method the family or the model lacks returns
//! [`PicoError::not_supported`](pico_common::PicoError::not_supported).
//! `ScopeCapabilities`, read at open, says which features a unit has.
//! Each family keeps its impl of a feature in `<family>/<feature>.rs`.

mod aux_io;
mod block;
mod digital;
mod ets;
mod extras;
mod led;
mod rapid_block;
mod siggen;
mod streaming;
mod trigger;

pub use aux_io::AuxIoDriver;
pub use block::BlockDriver;
pub use digital::DigitalPortDriver;
pub(crate) use digital::{no_hysteresis, raw_ports_only, StreamPorts, PORT_VOLTS};
pub use ets::EtsDriver;
pub use extras::ChannelExtrasDriver;
pub use led::LedDriver;
pub use rapid_block::RapidBlockDriver;
pub use siggen::SigGenDriver;
pub(crate) use streaming::flagged_channels;
pub use streaming::StreamingDriver;
pub use trigger::TriggerDriver;
