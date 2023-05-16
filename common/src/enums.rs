use num_derive::*;
use std::{convert::From, fmt, str::FromStr};

/// Error when attempting to parse enums from strings
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, Ord, PartialOrd, Hash, PartialEq, Eq)]
/// Pico channel options
pub enum PicoChannel {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
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

impl From<i32> for PicoChannel {
    fn from(value: i32) -> Self {
        num_traits::FromPrimitive::from_i32(value).expect("Non-valid channel")
    }
}

impl From<u32> for PicoChannel {
    fn from(value: u32) -> Self {
        num_traits::FromPrimitive::from_u32(value).expect("Non-valid channel")
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
}

/// Pico coupling options
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq, Default)]
pub enum PicoCoupling {
    AC = 0,
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

impl From<i32> for PicoCoupling {
    fn from(value: i32) -> Self {
        num_traits::FromPrimitive::from_i32(value).expect("Non-valid channel")
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
}

/// Driver downsampling mode
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum DownsampleMode {
    NONE = 0,
    AGGREGATE = 1,
    DECIMATE = 2,
    AVERAGE = 4,
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

/// Pico info options
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoInfo {
    DRIVER_VERSION = 0x0000_0000,
    USB_VERSION = 0x0000_0001,
    HARDWARE_VERSION = 0x0000_0002,
    VARIANT_INFO = 0x0000_0003,
    BATCH_AND_SERIAL = 0x0000_0004,
    CAL_DATE = 0x0000_0005,
    KERNEL_VERSION = 0x0000_0006,
    DIGITAL_HARDWARE_VERSION = 0x0000_0007,
    ANALOGUE_HARDWARE_VERSION = 0x0000_0008,
    FIRMWARE_VERSION_1 = 0x0000_0009,
    FIRMWARE_VERSION_2 = 0x0000_000A,
    MAC_ADDRESS = 0x0000_000B,
    SHADOW_CAL = 0x0000_000C,
    IPP_VERSION = 0x0000_000D,
    DRIVER_PATH = 0x0000_000E,
    FIRMWARE_VERSION_3 = 0x0000_000F,
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
