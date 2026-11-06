/// One read of a logger channel that you can only poll (PT-104, CM3)
///
/// - These loggers convert their channels in turn, about 720 ms each.
///   A read returns the newest conversion.
/// - A read between two conversions of a channel gives the same value again.
///   The driver flags it with `PICO_WARNING_REPEAT_VALUE` (seen on a USB PT-104).
/// - A PT-104 channel set to PT100 or PT1000 with no sensor attached never converts, and reads [`LoggerReading::NotReady`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoggerReading {
    /// A conversion since the last read
    New(i32),
    /// The same conversion as the last read
    Repeat(i32),
    /// No conversion yet (`PICO_NO_SAMPLES_AVAILABLE`)
    NotReady,
}

impl LoggerReading {
    /// The raw value, new or repeated
    pub fn value(self) -> Option<i32> {
        match self {
            LoggerReading::New(value) | LoggerReading::Repeat(value) => Some(value),
            LoggerReading::NotReady => None,
        }
    }

    /// The raw value only when it is a new conversion
    pub fn new_value(self) -> Option<i32> {
        match self {
            LoggerReading::New(value) => Some(value),
            _ => None,
        }
    }
}

/// How long a PT-104 or CM3 takes to convert one channel, in milliseconds
///
/// - The programmer's guides say "Measurement takes about 720 ms per active channel".
///   See `UsbPt104SetChannel` in the USB PT-104 Programmer's Guide, and `PLCM3SetChannel` in the PicoLog CM3 Programmer's Guide.
pub const LOGGER_CONVERSION_MS: u32 = 720;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_repeat_has_a_value_but_no_new_value() {
        assert_eq!(LoggerReading::Repeat(3).value(), Some(3));
        assert_eq!(LoggerReading::Repeat(3).new_value(), None);
        assert_eq!(LoggerReading::New(3).new_value(), Some(3));
        assert_eq!(LoggerReading::NotReady.value(), None);
    }
}
