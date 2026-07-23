use std::sync::Arc;

use enum_iterator::Sequence;
use num_derive::ToPrimitive;

/// Pico TC08 Error codes
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, ToPrimitive, Sequence)]
pub enum TC08Channel {
    CHANNEL_CJC = 0,
    CHANNEL_1 = 1,
    CHANNEL_2 = 2,
    CHANNEL_3 = 3,
    CHANNEL_4 = 4,
    CHANNEL_5 = 5,
    CHANNEL_6 = 6,
    CHANNEL_7 = 7,
    CHANNEL_8 = 8,
}

impl From<TC08Channel> for i16 {
    fn from(value: TC08Channel) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid channel")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TCType {
    B,
    E,
    J,
    #[default]
    K,
    N,
    R,
    S,
    T,
}

impl From<TCType> for i8 {
    fn from(value: TCType) -> Self {
        format!("{:?}", value)
            .chars()
            .next()
            .expect("Could not get TCType character") as i8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MainsRejectionFreq {
    #[default]
    _50Hz,
    _60Hz,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TC08Info {
    pub handle: Arc<i16>,
    pub serial: String,
    pub hardware_version: i16,
    pub variant: i16,
}
