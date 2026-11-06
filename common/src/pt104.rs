use enum_iterator::Sequence;
use num_derive::ToPrimitive;
use std::{fmt, str::FromStr, sync::Arc};

use crate::ParseError;

/// A channel of the USB PT-104 temperature data logger
///
/// - The unit has 4 inputs.
/// - The driver also has channels 5 to 8, which read only the other side of a single-ended input on 1 to 4.
///   On a USB PT-104, a read of 5 to 8 next to a differential input returns `PICO_INVALID_CHANNEL`.
///   Thus this enum does not have them.
/// - `Ord` follows the channel numbers of the driver.
///   An ordered collection with this key gives the channels in number order.
/// - It prints and parses as the channel number, such as `1`.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
pub enum PT104Channel {
    /// Channel 1
    CHANNEL_1 = 1,
    /// Channel 2
    CHANNEL_2 = 2,
    /// Channel 3
    CHANNEL_3 = 3,
    /// Channel 4
    CHANNEL_4 = 4,
}

impl From<PT104Channel> for u32 {
    fn from(value: PT104Channel) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid channel")
    }
}

impl fmt::Display for PT104Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u32::from(*self))
    }
}

impl FromStr for PT104Channel {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();

        match input.as_str() {
            "1" => Ok(PT104Channel::CHANNEL_1),
            "2" => Ok(PT104Channel::CHANNEL_2),
            "3" => Ok(PT104Channel::CHANNEL_3),
            "4" => Ok(PT104Channel::CHANNEL_4),
            _ => Err(ParseError),
        }
    }
}

/// The sensor or input type of a PT-104 channel
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum PT104DataType {
    /// Channel disabled
    #[default]
    Off,
    /// Platinum RTD, 100 Ω
    PT100,
    /// Platinum RTD, 1000 Ω
    PT1000,
    /// Resistance converter up to 375 Ω
    ResistanceTo375R,
    /// Resistance converter up to 10 kΩ
    ResistanceTo10K,
    /// Differential voltage input, range ±115 mV
    DifferentialTo115mV,
    /// Differential voltage input, range ±2500 mV
    DifferentialTo2500mV,
    /// Single-ended voltage input, range 0 to 115 mV
    SingleEndedTo115mV,
    /// Single-ended voltage input, range 0 to 2500 mV
    SingleEndedTo2500mV,
}

impl From<PT104DataType> for u32 {
    /// The `USBPT104_DATA_TYPES` value of the driver
    fn from(value: PT104DataType) -> Self {
        match value {
            PT104DataType::Off => 0,
            PT104DataType::PT100 => 1,
            PT104DataType::PT1000 => 2,
            PT104DataType::ResistanceTo375R => 3,
            PT104DataType::ResistanceTo10K => 4,
            PT104DataType::DifferentialTo115mV => 5,
            PT104DataType::DifferentialTo2500mV => 6,
            PT104DataType::SingleEndedTo115mV => 7,
            PT104DataType::SingleEndedTo2500mV => 8,
        }
    }
}

impl PT104DataType {
    /// The factor from a raw `UsbPt104GetValue` reading to the unit, and the unit
    ///
    /// - The scale table of `UsbPt104GetValue` is in the USB PT-104 Programmer's Guide (usbpt104pg r8, section 3.4).
    ///   Temperature is in 1/1000 °C, 0 to 2.5 V in 10 nV, 0 to 115 mV in 1 nV, 0 to 375 Ω in 1 µΩ and 0 to 10 kΩ in 1 mΩ.
    /// - The wiring does not change the scale.
    /// - Returns `None` for [`PT104DataType::Off`].
    ///
    /// ```
    /// use pico_common::PT104DataType;
    ///
    /// assert_eq!(PT104DataType::PT100.scale(), Some((1e-3, "°C")));
    /// assert_eq!(PT104DataType::PT100.to_unit(21_500), Some(21.5));
    /// ```
    pub fn scale(self) -> Option<(f64, &'static str)> {
        match self {
            PT104DataType::Off => None,
            PT104DataType::PT100 | PT104DataType::PT1000 => Some((1e-3, "°C")),
            PT104DataType::ResistanceTo375R => Some((1e-6, "Ω")),
            PT104DataType::ResistanceTo10K => Some((1e-3, "Ω")),
            PT104DataType::DifferentialTo115mV | PT104DataType::SingleEndedTo115mV => {
                Some((1e-9, "V"))
            }
            PT104DataType::DifferentialTo2500mV | PT104DataType::SingleEndedTo2500mV => {
                Some((1e-8, "V"))
            }
        }
    }

    /// Converts a raw reading to the unit of [`PT104DataType::scale`]
    ///
    /// - Returns `None` for [`PT104DataType::Off`].
    pub fn to_unit(self, raw: i32) -> Option<f64> {
        self.scale().map(|(factor, _)| raw as f64 * factor)
    }

    /// Whether the channel reads a single-ended voltage
    pub fn is_single_ended(self) -> bool {
        matches!(
            self,
            PT104DataType::SingleEndedTo115mV | PT104DataType::SingleEndedTo2500mV
        )
    }
}

impl fmt::Display for PT104DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PT104DataType::Off => write!(f, "Off"),
            PT104DataType::PT100 => write!(f, "PT100"),
            PT104DataType::PT1000 => write!(f, "PT1000"),
            PT104DataType::ResistanceTo375R => write!(f, "Resistance 0-375R"),
            PT104DataType::ResistanceTo10K => write!(f, "Resistance 0-10kR"),
            PT104DataType::DifferentialTo115mV => write!(f, "Differential ±115mV"),
            PT104DataType::DifferentialTo2500mV => write!(f, "Differential ±2500mV"),
            PT104DataType::SingleEndedTo115mV => write!(f, "Single-ended 0-115mV"),
            PT104DataType::SingleEndedTo2500mV => write!(f, "Single-ended 0-2500mV"),
        }
    }
}

impl FromStr for PT104DataType {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace([' ', '-', '±'], "").to_uppercase();

        match input.as_str() {
            "OFF" => Ok(PT104DataType::Off),
            "PT100" => Ok(PT104DataType::PT100),
            "PT1000" => Ok(PT104DataType::PT1000),
            "RESISTANCE0375R" | "RESISTANCE375R" | "RESISTANCETO375R" => {
                Ok(PT104DataType::ResistanceTo375R)
            }
            "RESISTANCE010KR" | "RESISTANCE10K" | "RESISTANCE10KR" | "RESISTANCETO10K" => {
                Ok(PT104DataType::ResistanceTo10K)
            }
            "DIFFERENTIAL115MV" | "DIFFERENTIALTO115MV" => Ok(PT104DataType::DifferentialTo115mV),
            "DIFFERENTIAL2500MV" | "DIFFERENTIALTO2500MV" => {
                Ok(PT104DataType::DifferentialTo2500mV)
            }
            "SINGLEENDED0115MV" | "SINGLEENDED115MV" | "SINGLEENDEDTO115MV" => {
                Ok(PT104DataType::SingleEndedTo115mV)
            }
            "SINGLEENDED02500MV" | "SINGLEENDED2500MV" | "SINGLEENDEDTO2500MV" => {
                Ok(PT104DataType::SingleEndedTo2500mV)
            }
            _ => Err(ParseError),
        }
    }
}

/// The number of wires to a platinum resistance sensor on a PT-104
///
/// - 2 wires give the least accuracy, and 4 wires give the most.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum PT104Wires {
    /// 2 wires
    TwoWire = 2,
    /// 3 wires
    ThreeWire = 3,
    /// 4 wires
    #[default]
    FourWire = 4,
}

impl From<PT104Wires> for i16 {
    fn from(value: PT104Wires) -> Self {
        value as i16
    }
}

impl fmt::Display for PT104Wires {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PT104Wires::TwoWire => write!(f, "2-wire"),
            PT104Wires::ThreeWire => write!(f, "3-wire"),
            PT104Wires::FourWire => write!(f, "4-wire"),
        }
    }
}

/// What the driver tells about a PT-104 unit after it opens
///
/// - It has no `Serialize` or `Deserialize`, as `handle` is the handle of an open unit.
///   The other fields are strings, which you can serialize one by one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PT104Info {
    /// The driver handle of the open unit
    pub handle: Arc<i16>,
    /// The serial number
    pub serial: String,
    /// The hardware version
    pub hardware_version: String,
    /// The firmware version
    pub firmware_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels_order() {
        let mut channels = [PT104Channel::CHANNEL_3, PT104Channel::CHANNEL_1];
        channels.sort();
        assert_eq!(channels[0], PT104Channel::CHANNEL_1);
    }

    #[test]
    fn data_type_values_match_ffi() {
        assert_eq!(u32::from(PT104DataType::Off), 0);
        assert_eq!(u32::from(PT104DataType::PT100), 1);
        assert_eq!(u32::from(PT104DataType::PT1000), 2);
    }

    fn assert_close(value: Option<f64>, expected: f64) {
        let value = value.expect("a value");
        assert!(
            (value - expected).abs() <= expected.abs() * 1e-12,
            "{value} != {expected}"
        );
    }

    #[test]
    fn scale_follows_the_guide() {
        assert_eq!(PT104DataType::Off.scale(), None);
        assert_close(PT104DataType::PT100.to_unit(21_500), 21.5);
        assert_eq!(PT104DataType::PT1000.scale(), Some((1e-3, "°C")));
        assert_close(PT104DataType::ResistanceTo375R.to_unit(100_000_000), 100.0);
        assert_close(PT104DataType::ResistanceTo10K.to_unit(4_700_000), 4700.0);
        assert_close(
            PT104DataType::DifferentialTo115mV.to_unit(115_000_000),
            0.115,
        );
        assert_close(PT104DataType::SingleEndedTo2500mV.to_unit(250_000_000), 2.5);
        // Every full-scale reading fits an i32.
        assert!(2.5 / 1e-8 < i32::MAX as f64);
        assert!(375.0 / 1e-6 < i32::MAX as f64);
    }

    #[test]
    fn four_channels() {
        assert_eq!(enum_iterator::all::<PT104Channel>().count(), 4);
        assert_eq!(PT104Channel::from_str("5"), Err(ParseError));
    }

    #[test]
    fn display_from_str_round_trip() {
        for data_type in enum_iterator::all::<PT104DataType>() {
            assert_eq!(
                PT104DataType::from_str(&data_type.to_string().to_uppercase()),
                Ok(data_type)
            );
        }

        for channel in enum_iterator::all::<PT104Channel>() {
            assert_eq!(PT104Channel::from_str(&channel.to_string()), Ok(channel));
        }
    }
}
