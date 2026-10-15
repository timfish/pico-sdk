use pico_common::{PicoChannel, PicoCoupling, PicoError, PicoInfo, PicoRange, PicoResult};

/// Channel features beyond range and coupling. The bandwidth limit is
/// part of `OscilloscopeChannelConfig` instead, as some families take it
/// in their channel call.
pub trait ChannelExtrasDriver {
    /// The lowest and highest analogue offset in volts at `range` and
    /// `coupling`.
    fn offset_limits(
        &self,
        _handle: i16,
        _range: PicoRange,
        _coupling: PicoCoupling,
    ) -> PicoResult<(f64, f64)> {
        Err(PicoError::not_supported("offset_limits"))
    }
    /// Channels whose overvoltage protection tripped. The protection is
    /// on 50 Ω inputs.
    fn overvoltage_tripped(&self, _handle: i16) -> PicoResult<Vec<PicoChannel>> {
        Err(PicoError::not_supported("overvoltage_tripped"))
    }
    /// Resets the protection of every channel. Returns the channels
    /// that tripped again at once.
    fn reset_overvoltage(&self, _handle: i16) -> PicoResult<Vec<PicoChannel>> {
        Err(PicoError::not_supported("reset_overvoltage"))
    }
    /// An info string of the accessory, such as a probe, on `channel`.
    /// Takes the info types of the unit's own info call.
    fn accessory_info(
        &self,
        _handle: i16,
        _channel: PicoChannel,
        _info: PicoInfo,
    ) -> PicoResult<String> {
        Err(PicoError::not_supported("accessory_info"))
    }
}

/// The analogue offset limits of drivers that give them as two `f32`
/// out values, maximum first.
pub(crate) fn offset_limits_f32(
    context: &str,
    call: impl FnOnce(*mut f32, *mut f32) -> u32,
) -> PicoResult<(f64, f64)> {
    let (mut max, mut min) = (0f32, 0f32);
    pico_common::PicoStatus::from(call(&mut max, &mut min))
        .to_result((min as f64, max as f64), context)
}

/// Reads an accessory info string through `call(buffer, length,
/// required)`, again with a larger buffer when the first is short.
pub(crate) fn accessory_string(
    context: &str,
    mut call: impl FnMut(*mut i8, i16, *mut i16) -> u32,
) -> PicoResult<String> {
    use pico_common::PicoStatus;
    let mut len = 64i16;
    loop {
        let mut buffer = vec![0i8; len as usize];
        let mut required = 0i16;
        PicoStatus::from(call(buffer.as_mut_ptr(), len, &mut required)).to_result((), context)?;
        if required <= len || len == i16::MAX {
            let bytes: Vec<u8> = buffer
                .iter()
                .take_while(|&&b| b != 0)
                .map(|&b| b as u8)
                .collect();
            return Ok(String::from_utf8_lossy(&bytes).into_owned());
        }
        len = required;
    }
}

/// The channels with a non-zero flag in `(channel code, tripped)`
/// pairs.
pub(crate) fn tripped_channels(flags: impl IntoIterator<Item = (u32, u8)>) -> Vec<PicoChannel> {
    flags
        .into_iter()
        .filter(|&(code, tripped)| tripped != 0 && code < 8)
        .map(|(code, _)| PicoChannel::from(code))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_limits_come_back_lowest_first() {
        let limits = offset_limits_f32("test", |max, min| {
            unsafe {
                *max = 2.5;
                *min = -2.5;
            }
            0
        });
        assert_eq!(limits.unwrap(), (-2.5, 2.5));
        let refused = offset_limits_f32("test", |_, _| 0x0C);
        assert!(refused.is_err());
    }

    #[test]
    fn accessory_string_reads_again_when_short() {
        let text = b"TA375 probe";
        let mut calls = 0;
        let read = accessory_string("test", |buffer, len, required| {
            calls += 1;
            unsafe { *required = text.len() as i16 + 1 };
            if (len as usize) > text.len() {
                for (i, b) in text.iter().enumerate() {
                    unsafe { *buffer.add(i) = *b as i8 };
                }
            }
            0
        });
        assert_eq!(read.unwrap(), "TA375 probe");
        assert_eq!(calls, 1);
    }

    #[test]
    fn overvoltage_structs_are_packed_as_the_guides_give() {
        // A PICO_CHANNEL (4 bytes) then a uint8_t, byte-aligned.
        assert_eq!(
            std::mem::size_of::<pico_sys_dynamic::ps6000a::PICO_CHANNEL_OVERVOLTAGE_TRIPPED>(),
            5
        );
        assert_eq!(
            std::mem::size_of::<pico_sys_dynamic::psospa::PICO_CHANNEL_OVERVOLTAGE_TRIPPED>(),
            5
        );
    }

    #[test]
    fn only_tripped_channels_are_listed() {
        let flags = [(0, 0), (1, 1), (2, 0), (3, 7), (99, 1)];
        assert_eq!(
            tripped_channels(flags),
            vec![PicoChannel::B, PicoChannel::D]
        );
    }
}
