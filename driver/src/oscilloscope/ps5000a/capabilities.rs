//! Data sheets (picotech.com/download/datasheets/): 5000 A and B,
//! picoscope-5000ab-series-oscilloscope-data-sheet.pdf (MM040.en-11);
//! 5000D and D MSO, picoscope-5000d-series-data-sheet.pdf (MM093.en-4).
//! Every model has a 20 MHz limiter. ETS works at 8 bits only. Guide:
//! ps5000apg.en r4 (2018).

use crate::oscilloscope::models::{at_resolution, is_mso, model, MHZ};
use pico_common::{
    BandwidthLimit, PicoResolution, RapidBlockCapabilities, ScopeCapabilities, SigGenApi,
    SigGenCapabilities, TimestampCounter, TriggerCapabilities, TriggerModel,
};

pub(super) fn of(variant: &str) -> ScopeCapabilities {
    let model = model(variant);
    // The third digit gives the bandwidth: 5242, 5243, 5244 and the 54xx
    // forms.
    let bandwidth_mhz = match model.as_bytes().get(..4) {
        Some([b'5', b'2' | b'4', b'4', b'2']) => Some(60),
        Some([b'5', b'2' | b'4', b'4', b'3']) => Some(100),
        Some([b'5', b'2' | b'4', b'4', b'4']) => Some(200),
        _ => None,
    };
    let bandwidth_cases = match bandwidth_mhz {
        Some(mhz) if mhz > 60 => vec![at_resolution(PicoResolution::DR_16BIT, 60 * MHZ)],
        _ => Vec::new(),
    };
    let mso = is_mso(variant);
    ScopeCapabilities {
        block: true,
        rapid_block: Some(RapidBlockCapabilities {
            trigger_offset: true,
            timestamps: Some(TimestampCounter::SincePrevious),
            missed_triggers: false,
        }),
        triggers: TriggerCapabilities {
            model: TriggerModel::ConditionLists,
            simple: true,
            advanced: true,
            pulse_width: true,
            digital: mso,
            delay: true,
            holdoff: false,
        },
        ets: true,
        digital_ports: if mso { 2 } else { 0 },
        // A models have a function generator only; B and D models an
        // AWG too.
        sig_gen: Some(SigGenCapabilities {
            api: SigGenApi::Legacy,
            arbitrary: !model.ends_with('A'),
        }),
        aux_io: false,
        channel_leds: false,
        bandwidth_hz: bandwidth_mhz.map(|mhz| mhz * MHZ),
        bandwidth_cases,
        bandwidth_limits: vec![BandwidthLimit::HZ_20M],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::PicoRange;

    #[test]
    fn sixteen_bits_cut_the_bandwidth() {
        let caps = of("5444D MSO");
        assert_eq!(caps.bandwidth_hz, Some(200 * MHZ));
        let at = |r| caps.bandwidth_for(Some(r), PicoRange::X1_PROBE_1V, false);
        assert_eq!(at(PicoResolution::DR_8BIT), Some(200 * MHZ));
        assert_eq!(at(PicoResolution::DR_16BIT), Some(60 * MHZ));
        assert_eq!(caps.digital_ports, 2);
        assert_eq!(of("5242A").bandwidth_hz, Some(60 * MHZ));
        assert!(of("5242A").bandwidth_cases.is_empty());
    }
}
