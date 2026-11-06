#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

//! The common data types for the Pico Technology oscilloscope and data logger drivers.
//!
//! The crate has the driver names, the status codes and errors, and the channels, ranges and settings of each instrument family.
//! All types are plain data: no type in this crate loads a driver or opens a device.
//! Oscilloscopes and data loggers share only the driver names and the errors.
//! So each family has its own channel, settings and capability types.
//!
//! # Quick start
//!
//! ```
//! use pico_common::{
//!     Driver, OscilloscopeChannelConfig, PicoChannel, PicoCoupling, PicoRange, TCType,
//! };
//!
//! // A driver from its name, and the file name of its library
//! let driver: Driver = "ps5000a".parse().unwrap();
//! assert!(driver.is_scope());
//! println!("load {}", driver.get_binary_name());
//!
//! // The settings of an oscilloscope channel
//! let channel: PicoChannel = "A".parse().unwrap();
//! let config = OscilloscopeChannelConfig {
//!     coupling: PicoCoupling::DC,
//!     range: PicoRange::parse("±2 V", None).unwrap(),
//!     ..Default::default()
//! };
//! assert_eq!(config.range.get_max_scaled_value(), 2.0);
//! println!("channel {channel}: {} {}", config.range, config.coupling);
//!
//! // A data logger setting
//! let thermocouple: TCType = "type k".parse().unwrap();
//! assert_eq!(thermocouple, TCType::K);
//! ```
//!
//! # How the parts fit
//!
//! ```text
//!  USB product id or name ──► Driver         the driver library and its file name
//!                               │
//!                               ▼
//!  settings of each family                   PicoRange, PicoCoupling, TriggerConfig,
//!                               │            TC08Channel, PT104DataType, ...
//!                               ▼
//!  driver call ──► PicoStatus                or TC08Error, HRDLError
//!                               │
//!                               ▼
//!  PicoResult<T> = Result<T, PicoError>      the status, and the call that failed
//! ```
//!
//! # Main types
//!
//! Drivers:
//! - [`Driver`]: each driver library, its USB product ids and its file name.
//! - [`ScopeDriver`]: the oscilloscope drivers only, for a typed load.
//!
//! Oscilloscope settings:
//! - [`PicoChannel`], [`PicoRange`], [`PicoCoupling`], [`PicoResolution`]: a channel and its input.
//! - [`OscilloscopeChannelConfig`], [`OscilloscopeSampleConfig`], [`TimeUnits`]: the settings of a channel and the sample interval.
//! - [`ScopeCapabilities`]: what a scope can do, read when it opens.
//! - [`BlockConfig`], [`RapidBlockConfig`], [`Timebase`], [`Downsample`]: block captures and downsampling.
//! - [`TriggerConfig`], [`ChannelTrigger`], [`PulseWidthQualifier`]: triggers.
//! - [`SigGenConfig`], [`SigGenInfo`]: the signal generator.
//! - [`DigitalPort`], [`DigitalPortConfig`]: the digital ports of MSO models.
//! - [`OutputSpec`], [`OutputValue`], [`Outputs`]: the outputs of a device.
//! - [`PicoProbe`], [`ProbeState`]: the probes on PicoConnect and BNC+ inputs.
//! - [`PicoLed`], [`LedColour`]: the LEDs of the 3000E and 5000E.
//! - [`PowerSource`], [`RateLimit`]: power and the hardware sample rate.
//!
//! Data logger settings:
//! - [`TC08Channel`], [`TCType`], [`MainsRejectionFreq`]: USB TC-08.
//! - [`PT104Channel`], [`PT104DataType`], [`PT104Wires`]: USB PT-104.
//! - [`PL1000Channel`], [`PL1000Info`]: PicoLog 1000 series.
//! - [`HRDLChannel`], [`HRDLRange`], [`HRDLConversionTime`]: ADC-20 and ADC-24.
//! - [`PLCM3Channel`], [`PLCM3DataType`]: PicoLog CM3.
//! - [`DrDAQChannel`], [`DrDAQChannelInfo`]: USB DrDAQ.
//! - [`LoggerReading`]: one read of a PT-104 or CM3 channel.
//!
//! Errors:
//! - [`PicoStatus`]: the status codes of the drivers.
//! - [`PicoError`], [`PicoResult`]: a status with the call that returned it.
//! - [`TC08Error`], [`HRDLError`]: the error codes of the TC-08 and PicoHRDL drivers, with the nearest [`PicoStatus`].
//! - [`ParseError`]: a string that does not parse as an enum of this crate.
//!
//! USB:
//! - [`UsbLink`], [`UsbSpeed`]: how a device connects to the host.
//!
//! # Cargo features
//!
//! - `serde`: adds `Serialize` and `Deserialize` to most data types.
//!   The `Info` types of the data loggers do not get them, as they hold the handle of an open unit.

mod cm3;
mod config;
mod drdaq;
mod driver;
mod enums;
mod error;
mod hrdl;
mod pl1000;
mod power;
mod probe;
mod pt104;
mod range;
mod reading;
mod samples;
mod scope;
mod status;
mod tc08;
mod usb;
mod utils;

pub use cm3::*;
pub use config::*;
pub use drdaq::*;
pub use driver::*;
pub use enums::*;
pub use error::*;
pub use hrdl::*;
pub use pl1000::*;
pub use power::*;
pub use probe::*;
pub use pt104::*;
pub use range::*;
pub use reading::*;
pub use samples::*;
pub use scope::*;
pub use status::*;
pub use tc08::*;
pub use usb::*;
pub use utils::*;
