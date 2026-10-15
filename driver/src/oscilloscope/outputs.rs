//! A scope's outputs: the AUX I/O connector (ps6000a, psospa), the
//! signal generator, and the LEDs of units that have them. Each is a
//! call on the open unit, so a stream goes on.

use super::OscilloscopeDriverInternal;
use pico_common::{
    find_output, sig_gen_dc, AuxIoMode, LedColour, LedState, OutputKind, OutputSpec, OutputValue,
    PicoChannel, PicoLed, PicoResult, ScopeCapabilities, SigGenConfig,
};

/// The outputs of an open scope with `capabilities` and analog
/// `channels`. The signal generator is listed once its driver gives its
/// limits.
pub fn scope_outputs(
    driver: &dyn OscilloscopeDriverInternal,
    handle: i16,
    capabilities: &ScopeCapabilities,
    channels: &[PicoChannel],
) -> Vec<OutputSpec> {
    let mut specs = Vec::new();
    if capabilities.aux_io {
        specs.push(OutputSpec::new(
            "aux",
            "AUX I/O",
            OutputKind::Digital {
                release: true,
                trigger_out: true,
            },
            OutputValue::Released,
        ));
    }
    if capabilities.sig_gen.is_some() {
        match driver.sig_gen_info(handle) {
            Ok(info) => specs.push(OutputSpec::new(
                "awg",
                "Signal generator",
                OutputKind::SigGen(info),
                OutputValue::SigGen(None),
            )),
            Err(error) => tracing::debug!(%error, "signal generator not listed"),
        }
    }
    if capabilities.channel_leds {
        let leds = channels
            .iter()
            .map(|c| PicoLed::Channel(*c))
            .chain([PicoLed::Awg, PicoLed::Aux]);
        for led in leds {
            specs.push(OutputSpec::new(
                &led_id(led),
                &format!("{led} LED"),
                OutputKind::Indicator,
                OutputValue::Released,
            ));
        }
    }
    specs
}

fn led_id(led: PicoLed) -> String {
    match led {
        PicoLed::Channel(c) => format!("led-{}", c.to_string().to_lowercase()),
        PicoLed::Awg => "led-awg".to_string(),
        PicoLed::Aux => "led-aux".to_string(),
    }
}

fn led_of(id: &str) -> Option<PicoLed> {
    match id.strip_prefix("led-")? {
        "awg" => Some(PicoLed::Awg),
        "aux" => Some(PicoLed::Aux),
        channel => channel.to_uppercase().parse().ok().map(PicoLed::Channel),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Call {
    Aux(AuxIoMode),
    SigGen(Option<SigGenConfig>),
    LedColour(PicoLed, LedColour),
    LedState(PicoLed, LedState),
}

/// The driver calls for `value` on output `id`.
fn calls(specs: &[OutputSpec], id: &str, value: &OutputValue) -> PicoResult<Vec<Call>> {
    find_output(specs, id)?.check(value)?;
    use OutputValue as V;
    Ok(match (id, value) {
        ("aux", value) => vec![Call::Aux(match value {
            V::On => AuxIoMode::High,
            V::Off => AuxIoMode::Low,
            V::TriggerOut => AuxIoMode::TriggerOut,
            _ => AuxIoMode::Input,
        })],
        ("awg", V::SigGen(config)) => vec![Call::SigGen(config.clone())],
        ("awg", V::Volts(volts)) => vec![Call::SigGen(Some(sig_gen_dc(*volts)))],
        (id, value) => {
            let led = led_of(id).expect("listed LEDs have LED ids");
            match value {
                V::Rgb(r, g, b) => vec![
                    Call::LedColour(led, LedColour::from_rgb(*r, *g, *b)),
                    Call::LedState(led, LedState::On),
                ],
                V::On => vec![Call::LedState(led, LedState::On)],
                V::Off => vec![Call::LedState(led, LedState::Off)],
                _ => vec![Call::LedState(led, LedState::Auto)],
            }
        }
    })
}

/// Sets output `id`, one of `specs`. Returns the value set: the
/// signal generator's as its driver reports it.
#[tracing::instrument(level = "debug", skip(driver, specs), err(level = "warn", Display))]
pub fn set_scope_output(
    driver: &dyn OscilloscopeDriverInternal,
    handle: i16,
    specs: &[OutputSpec],
    id: &str,
    value: &OutputValue,
) -> PicoResult<OutputValue> {
    let mut set = value.clone();
    for call in calls(specs, id, value)? {
        match call {
            Call::Aux(mode) => driver.set_aux_io_mode(handle, mode)?,
            Call::SigGen(config) => {
                let applied = driver.sig_gen_apply(handle, config.as_ref())?;
                if matches!(value, OutputValue::SigGen(_)) {
                    set = OutputValue::SigGen(applied);
                }
            }
            Call::LedColour(led, colour) => driver.set_led_colours(handle, &[(led, colour)])?,
            Call::LedState(led, state) => driver.set_led_states(handle, &[(led, state)])?,
        }
    }
    Ok(set)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::{SigGenInfo, SigGenShapeKind};

    fn specs() -> Vec<OutputSpec> {
        let mut specs = vec![
            OutputSpec::new(
                "aux",
                "AUX I/O",
                OutputKind::Digital {
                    release: true,
                    trigger_out: true,
                },
                OutputValue::Released,
            ),
            OutputSpec::new(
                "awg",
                "Signal generator",
                OutputKind::SigGen(SigGenInfo {
                    shapes: vec![SigGenShapeKind::Dc],
                    frequency_hz: (0.0, 20e6),
                    amplitude_v: (0.0, 4.0),
                    offset_v: (-2.0, 2.0),
                    awg_samples: None,
                }),
                OutputValue::SigGen(None),
            ),
        ];
        for led in ["led-a", "led-aux"] {
            specs.push(OutputSpec::new(
                led,
                led,
                OutputKind::Indicator,
                OutputValue::Released,
            ));
        }
        specs
    }

    #[test]
    fn aux_values_map_to_modes() {
        let specs = specs();
        let mode = |value| calls(&specs, "aux", &value).unwrap();
        assert_eq!(mode(OutputValue::On), vec![Call::Aux(AuxIoMode::High)]);
        assert_eq!(mode(OutputValue::Off), vec![Call::Aux(AuxIoMode::Low)]);
        assert_eq!(
            mode(OutputValue::Released),
            vec![Call::Aux(AuxIoMode::Input)]
        );
        assert_eq!(
            mode(OutputValue::TriggerOut),
            vec![Call::Aux(AuxIoMode::TriggerOut)]
        );
        assert!(calls(&specs, "aux", &OutputValue::Volts(1.0)).is_err());
    }

    #[test]
    fn volts_on_the_generator_is_a_dc_setting() {
        assert_eq!(
            calls(&specs(), "awg", &OutputValue::Volts(1.5)).unwrap(),
            vec![Call::SigGen(Some(sig_gen_dc(1.5)))]
        );
        assert!(calls(&specs(), "awg", &OutputValue::Volts(2.5)).is_err());
    }

    #[test]
    fn leds_take_a_colour_and_go_back_to_auto() {
        let specs = specs();
        assert_eq!(
            calls(&specs, "led-a", &OutputValue::Rgb(0, 255, 0)).unwrap(),
            vec![
                Call::LedColour(PicoLed::Channel(PicoChannel::A), LedColour::GREEN),
                Call::LedState(PicoLed::Channel(PicoChannel::A), LedState::On),
            ]
        );
        assert_eq!(
            calls(&specs, "led-aux", &OutputValue::Released).unwrap(),
            vec![Call::LedState(PicoLed::Aux, LedState::Auto)]
        );
        assert!(calls(&specs, "led-b", &OutputValue::On).is_err());
    }

    #[test]
    fn led_ids_go_both_ways() {
        for led in [
            PicoLed::Channel(PicoChannel::A),
            PicoLed::Channel(PicoChannel::H),
            PicoLed::Awg,
            PicoLed::Aux,
        ] {
            assert_eq!(led_of(&led_id(led)), Some(led));
        }
        assert_eq!(led_id(PicoLed::Channel(PicoChannel::C)), "led-c");
    }
}
