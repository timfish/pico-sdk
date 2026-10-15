//! The outputs of the USB DrDAQ (usbdrdaqpg.en r9): GPIO 1 to 4 as
//! digital outputs (§3.20), GPIO 1 and 2 also as PWM (§3.23), the
//! signal generator (§3.26, §3.27, §3.31) and the RGB LED (§3.2,
//! §3.24). The guide gives no PWM period limits; the u16 argument and
//! Pico's `usbdrdaqCon` example give 0 to 65535 µs.

use super::DrDAQDriverInternal;
use pico_common::{
    find_output, sig_gen_dc, OutputKind, OutputSpec, OutputValue, PicoError, PicoResult,
    PicoStatus, SigGenConfig, SigGenInfo, SigGenShape, SigGenShapeKind,
};
use pico_sys_dynamic::drdaq::{
    enUsbDrDaqWave_USB_DRDAQ_DC, enUsbDrDaqWave_USB_DRDAQ_RAMP_DOWN,
    enUsbDrDaqWave_USB_DRDAQ_RAMP_UP, enUsbDrDaqWave_USB_DRDAQ_SINE,
    enUsbDrDaqWave_USB_DRDAQ_SQUARE, enUsbDrDaqWave_USB_DRDAQ_TRIANGLE, USB_DRDAQ_WAVE,
};

/// Points an arbitrary waveform takes, from its `int16_t` size.
const AWG_POINTS: (u64, u64) = (1, i16::MAX as u64);
/// The AWG's step rate in points per second (§3.26).
const AWG_UPDATE_RATE: (f64, f64) = (1.0, 2_000_000.0);
/// An arbitrary waveform's full scale (§3.26).
const AWG_FULL_SCALE: f64 = 1000.0;

pub fn sig_gen_info() -> SigGenInfo {
    use SigGenShapeKind::*;
    SigGenInfo {
        shapes: vec![Sine, Square, Triangle, RampUp, RampDown, Dc, Arbitrary],
        frequency_hz: (0.0, 20_000.0),
        amplitude_v: (0.0, 3.0),
        offset_v: (-1.5, 1.5),
        awg_samples: Some(AWG_POINTS),
    }
}

/// The outputs of a USB DrDAQ.
pub fn outputs() -> Vec<OutputSpec> {
    let mut specs: Vec<OutputSpec> = (1..=4)
        .map(|n| {
            let kind = if n <= 2 {
                OutputKind::Pwm {
                    period_us: (0, u16::MAX as u32),
                }
            } else {
                OutputKind::Digital {
                    release: false,
                    trigger_out: false,
                }
            };
            OutputSpec::new(
                &format!("gpio{n}"),
                &format!("GPIO {n}"),
                kind,
                OutputValue::Off,
            )
        })
        .collect();
    specs.push(OutputSpec::new(
        "awg",
        "Signal generator",
        OutputKind::SigGen(sig_gen_info()),
        OutputValue::SigGen(None),
    ));
    specs.push(OutputSpec::new(
        "led",
        "LED",
        OutputKind::Indicator,
        OutputValue::Released,
    ));
    specs
}

#[derive(Debug, Clone, PartialEq)]
enum Call {
    Do {
        gpio: u32,
        on: bool,
    },
    Pwm {
        gpio: u32,
        period_us: u16,
        duty: u8,
    },
    SigGenBuiltIn {
        offset_uv: i32,
        pk_to_pk_uv: u32,
        frequency_hz: i16,
        wave: USB_DRDAQ_WAVE,
    },
    SigGenArbitrary {
        offset_uv: i32,
        pk_to_pk_uv: u32,
        points: Vec<i16>,
        update_rate: i32,
    },
    StopSigGen,
    EnableRgb(bool),
    Rgb(u16, u16, u16),
}

fn refuse(context: &str) -> PicoError {
    PicoError::from_status(PicoStatus::INVALID_PARAMETER, context)
}

fn microvolts(volts: f64) -> f64 {
    (volts * 1e6).round()
}

fn sig_gen_call(config: &SigGenConfig) -> PicoResult<Call> {
    let extras = config.duty_percent.is_some()
        || config.phase_deg.is_some()
        || config.sweep.is_some()
        || config.trigger.is_some()
        || config.filter.is_some();
    if extras {
        return Err(refuse(
            "the DrDAQ signal generator takes shape, frequency, amplitude and offset only",
        ));
    }
    let offset_uv = microvolts(config.offset_v) as i32;
    let pk_to_pk_uv = microvolts(config.amplitude_v) as u32;
    let wave = match &config.shape {
        SigGenShape::Sine => enUsbDrDaqWave_USB_DRDAQ_SINE,
        SigGenShape::Square => enUsbDrDaqWave_USB_DRDAQ_SQUARE,
        SigGenShape::Triangle => enUsbDrDaqWave_USB_DRDAQ_TRIANGLE,
        SigGenShape::RampUp => enUsbDrDaqWave_USB_DRDAQ_RAMP_UP,
        SigGenShape::RampDown => enUsbDrDaqWave_USB_DRDAQ_RAMP_DOWN,
        SigGenShape::Dc => enUsbDrDaqWave_USB_DRDAQ_DC,
        SigGenShape::Arbitrary(samples) => {
            let rate = (config.frequency_hz * samples.len() as f64).round();
            if !(AWG_UPDATE_RATE.0..=AWG_UPDATE_RATE.1).contains(&rate) {
                return Err(refuse("AWG update rate"));
            }
            let points = samples
                .iter()
                .map(|&s| (s as f64 * AWG_FULL_SCALE / i16::MAX as f64).round() as i16)
                .collect();
            return Ok(Call::SigGenArbitrary {
                offset_uv,
                pk_to_pk_uv,
                points,
                update_rate: rate as i32,
            });
        }
        _ => return Err(refuse("DrDAQ signal generator shape")),
    };
    Ok(Call::SigGenBuiltIn {
        offset_uv,
        pk_to_pk_uv,
        frequency_hz: config.frequency_hz.round() as i16,
        wave,
    })
}

/// The driver calls for `value` on output `id`, and the value they set.
fn calls(id: &str, value: &OutputValue) -> PicoResult<(Vec<Call>, OutputValue)> {
    let specs = outputs();
    find_output(&specs, id)?.check(value)?;
    let gpio = || {
        id[4..]
            .parse::<u32>()
            .expect("listed outputs are gpio1 to gpio4")
    };
    Ok(match (id, value) {
        ("awg", OutputValue::SigGen(None)) => (vec![Call::StopSigGen], value.clone()),
        ("awg", OutputValue::SigGen(Some(config))) => (vec![sig_gen_call(config)?], value.clone()),
        ("awg", OutputValue::Volts(volts)) => {
            (vec![sig_gen_call(&sig_gen_dc(*volts))?], value.clone())
        }
        ("led", OutputValue::Released) => (vec![Call::EnableRgb(false)], value.clone()),
        ("led", led) => {
            let (r, g, b) = match led {
                OutputValue::Rgb(r, g, b) => (*r, *g, *b),
                OutputValue::On => (255, 255, 255),
                _ => (0, 0, 0),
            };
            (
                vec![
                    Call::EnableRgb(true),
                    Call::Rgb(r as u16, g as u16, b as u16),
                ],
                led.clone(),
            )
        }
        (
            _,
            OutputValue::Pwm {
                period_us,
                duty_percent,
            },
        ) => {
            let duty = duty_percent.round() as u8;
            (
                vec![Call::Pwm {
                    gpio: gpio(),
                    period_us: *period_us as u16,
                    duty,
                }],
                OutputValue::Pwm {
                    period_us: *period_us,
                    duty_percent: duty as f64,
                },
            )
        }
        (_, on_off) => (
            vec![Call::Do {
                gpio: gpio(),
                on: *on_off == OutputValue::On,
            }],
            on_off.clone(),
        ),
    })
}

impl DrDAQDriverInternal {
    /// Sets output `id` of the unit. Returns the value set.
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    pub fn set_output(
        &self,
        handle: i16,
        id: &str,
        value: &OutputValue,
    ) -> PicoResult<OutputValue> {
        let (calls, set) = calls(id, value)?;
        for call in calls {
            let status = unsafe {
                match call {
                    Call::Do { gpio, on } => self.bindings.UsbDrDaqSetDO(handle, gpio, on as i16),
                    Call::Pwm {
                        gpio,
                        period_us,
                        duty,
                    } => self.bindings.UsbDrDaqSetPWM(handle, gpio, period_us, duty),
                    Call::SigGenBuiltIn {
                        offset_uv,
                        pk_to_pk_uv,
                        frequency_hz,
                        wave,
                    } => self.bindings.UsbDrDaqSetSigGenBuiltIn(
                        handle,
                        offset_uv,
                        pk_to_pk_uv,
                        frequency_hz,
                        wave,
                    ),
                    Call::SigGenArbitrary {
                        offset_uv,
                        pk_to_pk_uv,
                        mut points,
                        update_rate,
                    } => self.bindings.UsbDrDaqSetSigGenArbitrary(
                        handle,
                        offset_uv,
                        pk_to_pk_uv,
                        points.as_mut_ptr(),
                        points.len() as i16,
                        update_rate,
                    ),
                    Call::StopSigGen => self.bindings.UsbDrDaqStopSigGen(handle),
                    Call::EnableRgb(on) => self.bindings.UsbDrDaqEnableRGBLED(handle, on as i16),
                    Call::Rgb(r, g, b) => self.bindings.UsbDrDaqSetRGBLED(handle, r, g, b),
                }
            };
            PicoStatus::from(status).to_result((), "set_output")?;
        }
        Ok(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpio_numbers_are_the_header_codes() {
        use pico_sys_dynamic::drdaq::*;
        let codes = [
            enUsbDrDaqDO_USB_DRDAQ_GPIO_1,
            enUsbDrDaqDO_USB_DRDAQ_GPIO_2,
            enUsbDrDaqDO_USB_DRDAQ_GPIO_3,
            enUsbDrDaqDO_USB_DRDAQ_GPIO_4,
        ];
        for (n, code) in (1..=4).zip(codes) {
            let (c, _) = calls(&format!("gpio{n}"), &OutputValue::On).unwrap();
            assert_eq!(
                c,
                vec![Call::Do {
                    gpio: code,
                    on: true
                }]
            );
        }
    }

    #[test]
    fn gpio_take_on_off_and_pwm_only_on_1_and_2() {
        assert_eq!(
            calls("gpio4", &OutputValue::On).unwrap().0,
            vec![Call::Do { gpio: 4, on: true }]
        );
        let pwm = OutputValue::Pwm {
            period_us: 1000,
            duty_percent: 24.6,
        };
        let (c, set) = calls("gpio2", &pwm).unwrap();
        assert_eq!(
            c,
            vec![Call::Pwm {
                gpio: 2,
                period_us: 1000,
                duty: 25
            }]
        );
        assert_eq!(
            set,
            OutputValue::Pwm {
                period_us: 1000,
                duty_percent: 25.0
            }
        );
        assert!(calls("gpio3", &pwm).is_err());
        assert_eq!(
            calls("gpio1", &OutputValue::Off).unwrap().0,
            vec![Call::Do { gpio: 1, on: false }]
        );
    }

    #[test]
    fn sig_gen_takes_microvolts_and_header_wave_codes() {
        let sine = SigGenConfig {
            shape: SigGenShape::Sine,
            frequency_hz: 1000.0,
            amplitude_v: 2.0,
            ..sig_gen_dc(-0.5)
        };
        assert_eq!(
            calls("awg", &OutputValue::SigGen(Some(sine.clone())))
                .unwrap()
                .0,
            vec![Call::SigGenBuiltIn {
                offset_uv: -500_000,
                pk_to_pk_uv: 2_000_000,
                frequency_hz: 1000,
                wave: 0
            }]
        );
        assert_eq!(
            calls("awg", &OutputValue::Volts(1.25)).unwrap().0,
            vec![Call::SigGenBuiltIn {
                offset_uv: 1_250_000,
                pk_to_pk_uv: 0,
                frequency_hz: 0,
                wave: 5
            }]
        );
        assert_eq!(
            calls("awg", &OutputValue::SigGen(None)).unwrap().0,
            vec![Call::StopSigGen]
        );
        let too_big = SigGenConfig {
            amplitude_v: 3.5,
            ..sine.clone()
        };
        assert!(calls("awg", &OutputValue::SigGen(Some(too_big))).is_err());
        let swept = SigGenConfig {
            duty_percent: Some(20.0),
            ..sine
        };
        assert!(calls("awg", &OutputValue::SigGen(Some(swept))).is_err());
    }

    #[test]
    fn arbitrary_points_scale_to_1000_and_rate_follows_frequency() {
        let config = SigGenConfig {
            shape: SigGenShape::Arbitrary(vec![i16::MAX, 0, -i16::MAX, 16384]),
            frequency_hz: 100.0,
            amplitude_v: 1.0,
            ..sig_gen_dc(0.0)
        };
        assert_eq!(
            calls("awg", &OutputValue::SigGen(Some(config.clone())))
                .unwrap()
                .0,
            vec![Call::SigGenArbitrary {
                offset_uv: 0,
                pk_to_pk_uv: 1_000_000,
                points: vec![1000, 0, -1000, 500],
                update_rate: 400
            }]
        );
        let stopped = SigGenConfig {
            frequency_hz: 0.0,
            ..config
        };
        assert!(calls("awg", &OutputValue::SigGen(Some(stopped))).is_err());
    }

    #[test]
    fn led_turns_rgb_mode_on_and_released_turns_it_off() {
        assert_eq!(
            calls("led", &OutputValue::Rgb(10, 20, 30)).unwrap().0,
            vec![Call::EnableRgb(true), Call::Rgb(10, 20, 30)]
        );
        assert_eq!(
            calls("led", &OutputValue::Off).unwrap().0,
            vec![Call::EnableRgb(true), Call::Rgb(0, 0, 0)]
        );
        assert_eq!(
            calls("led", &OutputValue::Released).unwrap().0,
            vec![Call::EnableRgb(false)]
        );
    }
}
