//! Scope features beyond streaming, one module per feature. The types
//! are plain data; each family's driver turns them into its own calls.
//! See `docs/api-coverage-plan.md` for the guide sections behind them.

mod aux_io;
mod bandwidth;
mod block;
mod capabilities;
mod digital;
mod ets;
mod outputs;
mod siggen;
mod trigger;

pub use aux_io::*;
pub use bandwidth::*;
pub use block::*;
pub use capabilities::*;
pub use digital::*;
pub use ets::*;
pub use outputs::*;
pub use siggen::*;
pub use trigger::*;
