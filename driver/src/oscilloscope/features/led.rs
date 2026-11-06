use pico_common::{LedColour, LedState, PicoError, PicoLed, PicoResult};

/// The RGB LEDs of the units that have them
///
/// - `ScopeCapabilities::channel_leds` tells if a unit has them.
/// - When a call gives an LED more than one time, the last entry sets it.
/// - Each call returns Err with `NOT_USED` when the library lacks its function.
pub trait LedDriver {
    /// Sets the colour of each LED in `colours`
    fn set_led_colours(&self, _handle: i16, _colours: &[(PicoLed, LedColour)]) -> PicoResult<()> {
        Err(PicoError::not_supported("set_led_colours"))
    }
    /// Sets each LED in `states` on, off or to the control of the driver
    fn set_led_states(&self, _handle: i16, _states: &[(PicoLed, LedState)]) -> PicoResult<()> {
        Err(PicoError::not_supported("set_led_states"))
    }
    /// Sets the brightness of all LEDs that have a colour, from 0 to 100
    ///
    /// - The default of the driver is 79.
    fn set_led_brightness(&self, _handle: i16, _brightness: u8) -> PicoResult<()> {
        Err(PicoError::not_supported("set_led_brightness"))
    }
}
