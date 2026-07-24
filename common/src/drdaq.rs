use enum_iterator::Sequence;
use num_derive::ToPrimitive;
use std::{fmt, str::FromStr, sync::Arc};

use crate::ParseError;

/// USB DrDAQ onboard sensor channels
///
/// Unlike the other loggers in this family, DrDAQ's channels are not generic analog inputs - each
/// one is a specific onboard (or external probe) sensor. `Ord` follows the driver's channel
/// numbering.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
pub enum DrDAQChannel {
    /// External probe input 1
    CHANNEL_EXT1 = 1,
    /// External probe input 2
    CHANNEL_EXT2 = 2,
    /// External probe input 3
    CHANNEL_EXT3 = 3,
    /// The onboard oscilloscope input
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
    /// `UsbDrDaqSetInterval`/`UsbDrDaqGetSingle` take the channel as `USB_DRDAQ_INPUTS`, a C `uint`
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

/// The valid engineering-unit range and display resolution for a [`DrDAQChannel`], as reported by
/// `UsbDrDaqGetChannelInfo`
///
/// This is device-reported capability data rather than a hard coded per-channel table, following
/// the platform's "capabilities are runtime data, not schema" convention.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrDAQChannelInfo {
    pub min: f32,
    pub max: f32,
    pub decimal_places: i16,
}

/// What was discovered about a USB DrDAQ unit once it was opened
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrDAQInfo {
    pub handle: Arc<i16>,
    pub serial: String,
    pub variant: String,
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
