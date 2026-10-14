use pico_common::{LedColour, LedState, PicoError, PicoLed, PicoResult};

/// The RGB LEDs of units that have them (`ScopeCapabilities::channel_leds`).
/// For each call the last entry for an LED wins.
pub trait LedDriver {
    fn set_led_colours(&self, _handle: i16, _colours: &[(PicoLed, LedColour)]) -> PicoResult<()> {
        Err(PicoError::not_supported("set_led_colours"))
    }
    fn set_led_states(&self, _handle: i16, _states: &[(PicoLed, LedState)]) -> PicoResult<()> {
        Err(PicoError::not_supported("set_led_states"))
    }
    /// The brightness of every LED with a colour, 0 to 100 (the
    /// driver's default is 79).
    fn set_led_brightness(&self, _handle: i16, _brightness: u8) -> PicoResult<()> {
        Err(PicoError::not_supported("set_led_brightness"))
    }
}
