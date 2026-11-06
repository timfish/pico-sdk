//! Oscilloscope features other than streaming, one module for each feature.
//! The types are plain data.
//! The driver of each family turns them into its own calls.
//! `docs/api-coverage-plan.md` gives the guide sections for them.

mod aux_io;
mod bandwidth;
mod block;
mod capabilities;
mod digital;
mod ets;
mod led;
mod outputs;
mod overflow;
mod siggen;
mod trigger;

pub use aux_io::*;
pub use bandwidth::*;
pub use block::*;
pub use capabilities::*;
pub use digital::*;
pub use ets::*;
pub use led::*;
pub use outputs::*;
pub use siggen::*;
pub use trigger::*;
