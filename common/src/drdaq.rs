use enum_iterator::Sequence;
use num_derive::ToPrimitive;
use std::{fmt, str::FromStr, sync::Arc};

use crate::ParseError;

/// A channel of the USB DrDAQ data logger
///
/// - Each channel is one sensor on the board, or one input for an external probe.
///   The channels are not general analog inputs, as on the other loggers.
/// - `Ord` follows the channel numbers of the driver.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
pub enum DrDAQChannel {
    /// External probe input 1
    CHANNEL_EXT1 = 1,
    /// External probe input 2
    CHANNEL_EXT2 = 2,
    /// External probe input 3
    CHANNEL_EXT3 = 3,
    /// Onboard oscilloscope input
    CHANNEL_SCOPE = 4,
    /// Onboard pH probe input
    CHANNEL_PH = 5,
    /// Onboard resistance input
    CHANNEL_RES = 6,
    /// Onboard light sensor
    CHANNEL_LIGHT = 7,
    /// Onboard temperature sensor
    CHANNEL_TEMP = 8,
    /// Onboard microphone, raw waveform
    CHANNEL_MIC_WAVE = 9,
    /// Onboard microphone, sound level
    CHANNEL_MIC_LEVEL = 10,
}

impl From<DrDAQChannel> for u32 {
    /// The `USB_DRDAQ_INPUTS` value, a C `uint`, that `UsbDrDaqSetInterval` and `UsbDrDaqGetSingle` take
    fn from(value: DrDAQChannel) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid channel")
    }
}

impl fmt::Display for DrDAQChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DrDAQChannel::CHANNEL_EXT1 => write!(f, "External 1"),
            DrDAQChannel::CHANNEL_EXT2 => write!(f, "External 2"),
            DrDAQChannel::CHANNEL_EXT3 => write!(f, "External 3"),
            DrDAQChannel::CHANNEL_SCOPE => write!(f, "Scope"),
            DrDAQChannel::CHANNEL_PH => write!(f, "pH"),
            DrDAQChannel::CHANNEL_RES => write!(f, "Resistance"),
            DrDAQChannel::CHANNEL_LIGHT => write!(f, "Light"),
            DrDAQChannel::CHANNEL_TEMP => write!(f, "Temperature"),
            DrDAQChannel::CHANNEL_MIC_WAVE => write!(f, "Microphone (waveform)"),
            DrDAQChannel::CHANNEL_MIC_LEVEL => write!(f, "Microphone (level)"),
        }
    }
}

impl FromStr for DrDAQChannel {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace([' ', '(', ')'], "").to_uppercase();

        match input.as_str() {
            "EXT1" | "EXTERNAL1" | "1" => Ok(DrDAQChannel::CHANNEL_EXT1),
            "EXT2" | "EXTERNAL2" | "2" => Ok(DrDAQChannel::CHANNEL_EXT2),
            "EXT3" | "EXTERNAL3" | "3" => Ok(DrDAQChannel::CHANNEL_EXT3),
            "SCOPE" | "4" => Ok(DrDAQChannel::CHANNEL_SCOPE),
            "PH" | "5" => Ok(DrDAQChannel::CHANNEL_PH),
            "RES" | "RESISTANCE" | "6" => Ok(DrDAQChannel::CHANNEL_RES),
            "LIGHT" | "7" => Ok(DrDAQChannel::CHANNEL_LIGHT),
            "TEMP" | "TEMPERATURE" | "8" => Ok(DrDAQChannel::CHANNEL_TEMP),
            "MICROPHONEWAVEFORM" | "MICWAVE" | "9" => Ok(DrDAQChannel::CHANNEL_MIC_WAVE),
            "MICROPHONELEVEL" | "MICLEVEL" | "10" => Ok(DrDAQChannel::CHANNEL_MIC_LEVEL),
            _ => Err(ParseError),
        }
    }
}

/// The range and display resolution of a [`DrDAQChannel`], in its unit, as `UsbDrDaqGetChannelInfo` returns them
///
/// - The device gives these values; this crate has no table of them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrDAQChannelInfo {
    /// The minimum value, in the unit of the channel
    pub min: f32,
    /// The maximum value, in the unit of the channel
    pub max: f32,
    /// The number of decimal places to show
    pub decimal_places: i16,
    /// The number that divides a value from `UsbDrDaqGetValues` or `UsbDrDaqGetValuesF` to give the scaled value
    ///
    /// - A pH of 7.65 comes back as 765 with a divider of 100 (USB DrDAQ Programmer's Guide, usbdrdaqpg.en r9, sections 2.6 and 3.3).
    pub divider: i16,
}

impl DrDAQChannelInfo {
    /// Divides a value from the driver by the divider of the channel
    ///
    /// - A divider of 0 or less counts as 1.
    ///
    /// ```
    /// use pico_common::DrDAQChannelInfo;
    ///
    /// let ph = DrDAQChannelInfo {
    ///     min: 0.0,
    ///     max: 14.0,
    ///     decimal_places: 2,
    ///     divider: 100,
    /// };
    /// assert_eq!(ph.scale(765.0), 7.65);
    /// ```
    pub fn scale(&self, value: f32) -> f32 {
        if self.divider > 0 {
            value / self.divider as f32
        } else {
            value
        }
    }
}

/// What the driver tells about a USB DrDAQ unit after it opens
///
/// - It has no `Serialize` or `Deserialize`, as `handle` is the handle of an open unit.
///   The other fields are strings, which you can serialize one by one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrDAQInfo {
    /// The driver handle of the open unit
    pub handle: Arc<i16>,
    /// The serial number
    pub serial: String,
    /// The model name
    pub variant: String,
    /// The hardware version
    pub hardware_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels_order() {
        let mut channels = [DrDAQChannel::CHANNEL_TEMP, DrDAQChannel::CHANNEL_EXT1];
        channels.sort();
        assert_eq!(channels[0], DrDAQChannel::CHANNEL_EXT1);
    }

    #[test]
    fn channel_values_match_ffi() {
        assert_eq!(u32::from(DrDAQChannel::CHANNEL_EXT1), 1);
        assert_eq!(u32::from(DrDAQChannel::CHANNEL_SCOPE), 4);
        assert_eq!(u32::from(DrDAQChannel::CHANNEL_MIC_LEVEL), 10);
    }

    #[test]
    fn display_from_str_round_trip() {
        for channel in enum_iterator::all::<DrDAQChannel>() {
            assert_eq!(
                DrDAQChannel::from_str(&channel.to_string().to_uppercase()),
                Ok(channel)
            );
        }
    }
}
