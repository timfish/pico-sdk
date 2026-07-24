#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

//! Common enums, structs and traits used for communication with Pico Technology drivers.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! Types are grouped by instrument family. Oscilloscopes and data loggers have very little in
//! common beyond driver loading and error reporting, so each family gets its own channel,
//! configuration and capability types rather than a union of every setting any device might have.

mod config;
mod driver;
mod enums;
mod error;
mod pl1000;
mod pt104;
mod range;
mod status;
mod tc08;
mod utils;

pub use config::*;
pub use driver::*;
pub use enums::*;
pub use error::*;
pub use pl1000::*;
pub use pt104::*;
pub use range::*;
pub use status::*;
pub use tc08::*;
pub use utils::*;
