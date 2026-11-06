use num_derive::*;
use std::{convert::From, fmt, str::FromStr};

/// The error when a string or a driver value does not convert to one of the enums of this crate
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// An analog input channel of an oscilloscope
///
/// - It prints and parses as the letter, such as `A`.
pub enum PicoChannel {
    /// Channel A
    A = 0,
    /// Channel B
    B = 1,
    /// Channel C
    C = 2,
    /// Channel D
    D = 3,
    /// Channel E
    E = 4,
    /// Channel F
    F = 5,
    /// Channel G
    G = 6,
    /// Channel H
    H = 7,
}

impl FromStr for PicoChannel {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();

        match &input[..] {
            "A" => Ok(PicoChannel::A),
            "B" => Ok(PicoChannel::B),
            "C" => Ok(PicoChannel::C),
            "D" => Ok(PicoChannel::D),
            "E" => Ok(PicoChannel::E),
            "F" => Ok(PicoChannel::F),
            "G" => Ok(PicoChannel::G),
            "H" => Ok(PicoChannel::H),
            _ => Err(ParseError),
        }
    }
}

impl fmt::Display for PicoChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<PicoChannel> for u32 {
    fn from(value: PicoChannel) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid channel")
    }
}

impl From<PicoChannel> for i32 {
    fn from(value: PicoChannel) -> Self {
        num_traits::ToPrimitive::to_i32(&value).expect("Non-valid channel")
    }
}

impl From<PicoChannel> for i16 {
    fn from(value: PicoChannel) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid channel")
    }
}

impl TryFrom<i32> for PicoChannel {
    type Error = ParseError;

    /// The channel for a driver channel number
    ///
    /// - Returns [`ParseError`] when the number is not 0 to 7.
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_i32(value).ok_or(ParseError)
    }
}

impl TryFrom<u32> for PicoChannel {
    type Error = ParseError;

    /// The channel for a driver channel number
    ///
    /// - Returns [`ParseError`] when the number is not 0 to 7.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_u32(value).ok_or(ParseError)
    }
}

#[cfg(test)]
mod channel_tests {
    use super::*;

    #[test]
    fn channel_parse() {
        assert_eq!(PicoChannel::from_str("a"), Ok(PicoChannel::A));
        assert_eq!(PicoChannel::from_str("B "), Ok(PicoChannel::B));
        assert_eq!(PicoChannel::from_str("x"), Err(ParseError));
    }

    #[test]
    fn channel_from_unknown_number_is_err() {
        assert_eq!(PicoChannel::try_from(1i32).ok(), Some(PicoChannel::B));
        assert_eq!(PicoChannel::try_from(8i32).ok(), None);
        assert_eq!(PicoChannel::try_from(8u32).ok(), None);
    }
}

/// The vertical resolution of an oscilloscope with a choice of resolution
///
/// - The value of each variant is the `PICO_DEVICE_RESOLUTION` code of the driver.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PicoResolution {
    /// 8 bits
    DR_8BIT = 0,
    /// 12 bits
    DR_12BIT = 1,
    /// 14 bits
    DR_14BIT = 2,
    /// 15 bits
    DR_15BIT = 3,
    /// 16 bits
    DR_16BIT = 4,
    /// 10 bits
    DR_10BIT = 10,
}

impl PicoResolution {
    /// The number of bits
    pub fn bits(self) -> u8 {
        match self {
            PicoResolution::DR_8BIT => 8,
            PicoResolution::DR_10BIT => 10,
            PicoResolution::DR_12BIT => 12,
            PicoResolution::DR_14BIT => 14,
            PicoResolution::DR_15BIT => 15,
            PicoResolution::DR_16BIT => 16,
        }
    }

    /// The resolution for a driver code
    ///
    /// - Returns `None` for a code that this crate does not know.
    pub fn from_driver_value(value: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(value)
    }

    /// The driver code of the resolution
    pub fn to_driver_value(self) -> u32 {
        num_traits::ToPrimitive::to_u32(&self).expect("resolution fits u32")
    }
}

impl fmt::Display for PicoResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-bit", self.bits())
    }
}

/// The input coupling of an oscilloscope channel
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq, Default)]
pub enum PicoCoupling {
    /// AC coupling
    AC = 0,
    /// DC coupling
    #[default]
    DC = 1,
}

impl FromStr for PicoCoupling {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();

        match &input[..] {
            "AC" => Ok(PicoCoupling::AC),
            "DC" => Ok(PicoCoupling::DC),
            _ => Err(ParseError),
        }
    }
}

impl fmt::Display for PicoCoupling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<PicoCoupling> for u32 {
    fn from(value: PicoCoupling) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid coupling")
    }
}

impl From<PicoCoupling> for i32 {
    fn from(value: PicoCoupling) -> Self {
        num_traits::ToPrimitive::to_i32(&value).expect("Non-valid coupling")
    }
}

impl From<PicoCoupling> for i16 {
    fn from(value: PicoCoupling) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid coupling")
    }
}

impl TryFrom<i32> for PicoCoupling {
    type Error = ParseError;

    /// The coupling for a driver value
    ///
    /// - Returns [`ParseError`] when the value is not 0 or 1.
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_i32(value).ok_or(ParseError)
    }
}

#[cfg(test)]
mod coupling_tests {
    use super::*;

    #[test]
    fn coupling_parse() {
        assert_eq!(PicoCoupling::from_str("ac"), Ok(PicoCoupling::AC));
        assert_eq!(PicoCoupling::from_str("DC"), Ok(PicoCoupling::DC));
        assert_eq!(PicoCoupling::from_str("ad"), Err(ParseError));
    }

    #[test]
    fn coupling_from_unknown_number_is_err() {
        assert_eq!(PicoCoupling::try_from(0i32).ok(), Some(PicoCoupling::AC));
        assert_eq!(PicoCoupling::try_from(2i32).ok(), None);
    }
}

/// A downsample mode, as the driver takes it
///
/// - The value of each variant is the driver value.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum DownsampleMode {
    /// No downsample
    NONE = 0,
    /// The lowest and the highest value of each group
    AGGREGATE = 1,
    /// The first sample of each group
    DECIMATE = 2,
    /// The mean of each group
    AVERAGE = 4,
    /// The distribution of each group
    DISTRIBUTION = 8,
}

impl From<DownsampleMode> for u32 {
    fn from(value: DownsampleMode) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid downsample mode")
    }
}

impl From<DownsampleMode> for i32 {
    fn from(value: DownsampleMode) -> Self {
        num_traits::ToPrimitive::to_i32(&value).expect("Non-valid downsample mode")
    }
}

/// An info field that the `GetUnitInfo` call of a driver reads
///
/// - The value of each variant is the `PICO_INFO` value of the driver.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoInfo {
    /// The driver version
    DRIVER_VERSION = 0x0000_0000,
    /// The USB version
    USB_VERSION = 0x0000_0001,
    /// The hardware version
    HARDWARE_VERSION = 0x0000_0002,
    /// The model name
    VARIANT_INFO = 0x0000_0003,
    /// The batch and serial number
    BATCH_AND_SERIAL = 0x0000_0004,
    /// The calibration date
    CAL_DATE = 0x0000_0005,
    /// The kernel driver version
    KERNEL_VERSION = 0x0000_0006,
    /// The digital hardware version
    DIGITAL_HARDWARE_VERSION = 0x0000_0007,
    /// The analog hardware version
    ANALOGUE_HARDWARE_VERSION = 0x0000_0008,
    /// The first firmware version
    FIRMWARE_VERSION_1 = 0x0000_0009,
    /// The second firmware version
    FIRMWARE_VERSION_2 = 0x0000_000A,
    /// The MAC address
    MAC_ADDRESS = 0x0000_000B,
    /// The shadow calibration status
    SHADOW_CAL = 0x0000_000C,
    /// The IPP library version
    IPP_VERSION = 0x0000_000D,
    /// The path of the driver library
    DRIVER_PATH = 0x0000_000E,
    /// The third firmware version
    FIRMWARE_VERSION_3 = 0x0000_000F,
    /// The front panel firmware version
    FRONT_PANEL_FIRMWARE_VERSION = 0x0000_0010,
}

impl From<PicoInfo> for u32 {
    fn from(value: PicoInfo) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid info type")
    }
}

impl From<PicoInfo> for i16 {
    fn from(value: PicoInfo) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid info type")
    }
}
