use enum_iterator::Sequence;
use num_derive::ToPrimitive;
use std::{fmt, str::FromStr, sync::Arc};

use crate::ParseError;

/// USB PT-104 channels
///
/// The unit has 4 inputs. The driver also numbers channels 5 to 8, which read only as the
/// other side of a single-ended input on 1 to 4; on a USB PT-104 a read of 5 to 8 beside a
/// differential input returns `PICO_INVALID_CHANNEL`, so they are left out.
///
/// `Ord` follows the driver's channel numbering so iterating an ordered
/// collection keyed by this enum gives channels in numeric order.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
pub enum PT104Channel {
    CHANNEL_1 = 1,
    CHANNEL_2 = 2,
    CHANNEL_3 = 3,
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

/// Sensor types that can be connected to PT-104 channels
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum PT104DataType {
    /// Channel disabled
    #[default]
    Off,
    /// Platinum RTD, 100 ohm
    PT100,
    /// Platinum RTD, 1000 ohm
    PT1000,
    /// Resistance converter up to 375 ohm
    ResistanceTo375R,
    /// Resistance converter up to 10 kilohm
    ResistanceTo10K,
    /// Differential voltage input, range ±115 mV
    DifferentialTo115mV,
    /// Differential voltage input, range ±2500 mV
    DifferentialTo2500mV,
    /// Single-ended voltage input, range 0-115 mV
    SingleEndedTo115mV,
    /// Single-ended voltage input, range 0-2500 mV
    SingleEndedTo2500mV,
}

impl From<PT104DataType> for u32 {
    /// Maps to USBPT104_DATA_TYPES constants from the FFI bindings.
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
    /// From the scaling table of `UsbPt104GetValue` in the USB PT-104 Programmer's Guide
    /// (usbpt104pg r8, section 3.4): temperature in 1/1000 °C, 0 to 2.5 V in 10 nV, 0 to
    /// 115 mV in 1 nV, 0 to 375 Ω in 1 µΩ and 0 to 10 kΩ in 1 mΩ. The wiring does not change
    /// the scale. `None` for [`PT104DataType::Off`].
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

    /// A raw reading in the unit of [`PT104DataType::scale`]
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

/// Resistance wire configuration for PT RTD sensors
///
/// The PT-104 supports both 2-wire, 3-wire, and 4-wire configurations
/// for platinum resistance thermometers. 2-wire is the least accurate,
/// 4-wire is the most accurate.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum PT104Wires {
    TwoWire = 2,
    ThreeWire = 3,
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

/// What was discovered about a PT-104 once it was opened
///
/// Deliberately not `Serialize`/`Deserialize`: `handle` is live driver session state (an open
/// unit handle), not configuration or a capability. Every other field is already trivially
/// serializable (`String`), so a consumer that wants to ship the capability data over the wire
/// can do so from those fields directly without this type needing to derive serde itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PT104Info {
    pub handle: Arc<i16>,
    pub serial: String,
    pub hardware_version: String,
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
