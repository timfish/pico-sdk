//! The four digital I/O pins of the ADC-24 (the ADC-20 has none). One
//! call sets the direction and level of every pin (ADC-20/ADC-24 User's
//! Guide adc20.en r6 §5.4.16), so the driver keeps each open unit's
//! pins. While the unit samples, a pin's level can change but not its
//! direction: set a pin `On` or `Off` before a stream to use it during
//! the stream.

use super::HRDLDriverInternal;
use pico_common::{find_output, HRDLInfo, OutputKind, OutputSpec, OutputValue, PicoResult};

/// The pins set as outputs and the pins set high, bit 0 for pin 1.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct DigitalIo {
    out: u8,
    high: u8,
}

impl DigitalIo {
    /// The pins after `value` on `pin` (1 to 4).
    fn with(self, pin: u8, value: &OutputValue) -> DigitalIo {
        let bit = 1 << (pin - 1);
        match value {
            OutputValue::On => DigitalIo {
                out: self.out | bit,
                high: self.high | bit,
            },
            OutputValue::Off => DigitalIo {
                out: self.out | bit,
                high: self.high & !bit,
            },
            _ => DigitalIo {
                out: self.out & !bit,
                high: self.high & !bit,
            },
        }
    }
}

/// The outputs of the unit `info` describes.
pub fn outputs(info: &HRDLInfo) -> Vec<OutputSpec> {
    if info.is_adc20() {
        return Vec::new();
    }
    (1..=4)
        .map(|n| {
            OutputSpec::new(
                &format!("dio{n}"),
                &format!("Digital I/O {n}"),
                OutputKind::Digital {
                    release: true,
                    trigger_out: false,
                },
                OutputValue::Released,
            )
        })
        .collect()
}

impl HRDLDriverInternal {
    /// Sets output `id` of the unit `info` describes. Returns the
    /// value set.
    #[tracing::instrument(level = "debug", skip(self, info), err(level = "warn", Display))]
    pub fn set_output(
        &self,
        info: &HRDLInfo,
        id: &str,
        value: &OutputValue,
    ) -> PicoResult<OutputValue> {
        let handle = *info.handle;
        let specs = outputs(info);
        find_output(&specs, id)?.check(value)?;
        let pin: u8 = id[3..].parse().expect("listed outputs are dio1 to dio4");
        let mut units = self.digital_io.lock();
        let pins = units
            .get(&handle)
            .copied()
            .unwrap_or_default()
            .with(pin, value);
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings
                .HRDLSetDigitalIOChannel(handle, pins.out as i16, pins.high as i16, 0)
        })
        .to_result((), "set_output")?;
        units.insert(handle, pins);
        Ok(value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn info(variant: &str) -> HRDLInfo {
        HRDLInfo {
            handle: Arc::new(1),
            serial: String::new(),
            hardware_version: String::new(),
            variant: variant.to_string(),
        }
    }

    #[test]
    fn only_the_adc24_has_pins() {
        assert!(outputs(&info("20")).is_empty());
        let pins = outputs(&info("24"));
        assert_eq!(pins.len(), 4);
        assert_eq!(pins[3].id, "dio4");
        assert_eq!(pins[0].safe, OutputValue::Released);
    }

    #[test]
    fn pin_bits_are_the_header_codes() {
        use pico_sys_dynamic::picohrdl::*;
        let codes = [
            enHRDLDigitalIoChannel_HRDL_DIGITAL_IO_CHANNEL_1,
            enHRDLDigitalIoChannel_HRDL_DIGITAL_IO_CHANNEL_2,
            enHRDLDigitalIoChannel_HRDL_DIGITAL_IO_CHANNEL_3,
            enHRDLDigitalIoChannel_HRDL_DIGITAL_IO_CHANNEL_4,
        ];
        for (pin, code) in (1..=4).zip(codes) {
            let pins = DigitalIo::default().with(pin, &OutputValue::On);
            assert_eq!(pins.out as u32, code);
            assert_eq!(pins.high as u32, code);
        }
    }

    #[test]
    fn each_pin_keeps_the_others() {
        let pins = DigitalIo::default()
            .with(1, &OutputValue::On)
            .with(3, &OutputValue::Off)
            .with(4, &OutputValue::On);
        assert_eq!(
            pins,
            DigitalIo {
                out: 0b1101,
                high: 0b1001
            }
        );
        let pins = pins
            .with(4, &OutputValue::Released)
            .with(1, &OutputValue::Off);
        assert_eq!(
            pins,
            DigitalIo {
                out: 0b0101,
                high: 0
            }
        );
    }
}
