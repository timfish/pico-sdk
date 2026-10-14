use super::PSOSPADriver;
use crate::oscilloscope::LedDriver;
use pico_common::{LedColour, LedState, PicoChannel, PicoError, PicoLed, PicoResult, PicoStatus};
use pico_sys_dynamic::psospa::{
    enPicoLedSelect_PICO_LED_AUX, enPicoLedSelect_PICO_LED_AWG, enPicoLedSelect_PICO_LED_CHANNEL_A,
    enPicoLedState_PICO_LED_AUTO, enPicoLedState_PICO_LED_OFF, enPicoLedState_PICO_LED_ON,
    PICO_LED_COLOUR_PROPERTIES, PICO_LED_SELECT, PICO_LED_STATE, PICO_LED_STATE_PROPERTIES,
};

fn led_select(led: PicoLed) -> PICO_LED_SELECT {
    match led {
        PicoLed::Channel(channel) => enPicoLedSelect_PICO_LED_CHANNEL_A + u32::from(channel),
        PicoLed::Awg => enPicoLedSelect_PICO_LED_AWG,
        PicoLed::Aux => enPicoLedSelect_PICO_LED_AUX,
    }
}

fn led_state(state: LedState) -> PICO_LED_STATE {
    match state {
        LedState::Auto => enPicoLedState_PICO_LED_AUTO,
        LedState::Off => enPicoLedState_PICO_LED_OFF,
        LedState::On => enPicoLedState_PICO_LED_ON,
    }
}

fn led_colour_properties(colours: &[(PicoLed, LedColour)]) -> Vec<PICO_LED_COLOUR_PROPERTIES> {
    colours
        .iter()
        .map(|(led, colour)| PICO_LED_COLOUR_PROPERTIES {
            led_: led_select(*led),
            hue_: colour.hue(),
            saturation_: colour.saturation(),
        })
        .collect()
}

fn led_state_properties(states: &[(PicoLed, LedState)]) -> Vec<PICO_LED_STATE_PROPERTIES> {
    states
        .iter()
        .map(|(led, state)| PICO_LED_STATE_PROPERTIES {
            led_: led_select(*led),
            state_: led_state(*state),
        })
        .collect()
}

/// The driver refuses an empty list, which sets nothing anyway.
fn led_list_len<T>(list: &[T], context: &str) -> PicoResult<Option<u32>> {
    match list.len() {
        0 => Ok(None),
        n => u32::try_from(n)
            .map(Some)
            .map_err(|_| PicoError::from_status(PicoStatus::INVALID_PARAMETER, context)),
    }
}

// Units without LEDs refuse a state for channel A; AUTO is the state
// every LED opens in.
pub(super) fn has_channel_leds(driver: &PSOSPADriver, handle: i16) -> bool {
    driver
        .set_led_states(
            handle,
            &[(PicoLed::Channel(PicoChannel::A), LedState::Auto)],
        )
        .is_ok()
}

impl LedDriver for PSOSPADriver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_led_colours(&self, handle: i16, colours: &[(PicoLed, LedColour)]) -> PicoResult<()> {
        let Some(len) = led_list_len(colours, "set_led_colours")? else {
            return Ok(());
        };
        let properties = led_colour_properties(colours);
        PicoStatus::from(unsafe {
            self.bindings
                .psospaSetLedColours(handle, properties.as_ptr(), len)
        })
        .to_result((), "set_led_colours")
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_led_states(&self, handle: i16, states: &[(PicoLed, LedState)]) -> PicoResult<()> {
        let Some(len) = led_list_len(states, "set_led_states")? else {
            return Ok(());
        };
        let properties = led_state_properties(states);
        PicoStatus::from(unsafe {
            self.bindings
                .psospaSetLedStates(handle, properties.as_ptr(), len)
        })
        .to_result((), "set_led_states")
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_led_brightness(&self, handle: i16, brightness: u8) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaSetLedBrightness(handle, brightness) })
            .to_result((), "set_led_brightness")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leds_map_to_the_header_values() {
        assert_eq!(led_select(PicoLed::Channel(PicoChannel::A)), 0);
        assert_eq!(led_select(PicoLed::Channel(PicoChannel::D)), 3);
        assert_eq!(led_select(PicoLed::Channel(PicoChannel::H)), 7);
        assert_eq!(led_select(PicoLed::Awg), 0x1_0000);
        assert_eq!(led_select(PicoLed::Aux), 0x2_0000);
        assert_eq!(led_state(LedState::Auto), -1);
        assert_eq!(led_state(LedState::Off), 0);
        assert_eq!(led_state(LedState::On), 1);
    }

    #[test]
    fn colours_pack_into_byte_aligned_properties() {
        // The guide gives the structs as byte-aligned: 4 + 2 + 1 bytes.
        assert_eq!(std::mem::size_of::<PICO_LED_COLOUR_PROPERTIES>(), 7);
        assert_eq!(std::mem::size_of::<PICO_LED_STATE_PROPERTIES>(), 8);

        let packed = led_colour_properties(&[
            (PicoLed::Channel(PicoChannel::B), LedColour::GREEN),
            (PicoLed::Aux, LedColour::new(200, 40).unwrap()),
        ]);
        let fields: Vec<(u32, u16, u8)> = packed
            .iter()
            .map(|p| (p.led_, p.hue_, p.saturation_))
            .collect();
        assert_eq!(fields, vec![(1, 120, 100), (0x2_0000, 200, 40)]);

        let packed = led_state_properties(&[(PicoLed::Channel(PicoChannel::C), LedState::Off)]);
        let fields: Vec<(u32, i32)> = packed.iter().map(|p| (p.led_, p.state_)).collect();
        assert_eq!(fields, vec![(2, 0)]);
    }

    #[test]
    fn an_empty_list_sets_nothing() {
        assert_eq!(led_list_len::<u8>(&[], "test").unwrap(), None);
        assert_eq!(led_list_len(&[1, 2], "test").unwrap(), Some(2));
    }
}
