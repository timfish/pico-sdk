//! Data sheets (picotech.com/download/datasheets/): 2206 to 2208,
//! PicoScope2200.pdf (MM012.en-8); 2205 MSO,
//! picoscope-2205mso-data-sheet.pdf (MM031.en-8); 2204A to 2208A,
//! PicoScope2200ASeriesDataSheet.pdf (MM051.en-7); 2000 series A, B and
//! MSO, picoscope-2000-series-data-sheet-en.pdf (MM071.en-11). No model
//! has a hardware bandwidth limiter. Guide: ps2000apg.en-12 (2022).

use crate::oscilloscope::models::{is_mso, model, MHZ};
use pico_common::{
    RapidBlockCapabilities, ScopeCapabilities, SigGenApi, SigGenCapabilities, TriggerCapabilities,
    TriggerModel,
};

pub(super) fn of(variant: &str) -> ScopeCapabilities {
    let bandwidth_mhz = match model(variant) {
        "2204A" => Some(10),
        "2205" | "2205A" | "2405A" => Some(25),
        "2206" | "2206A" | "2206B" | "2406B" => Some(50),
        "2207B" | "2407B" => Some(70),
        "2207" | "2207A" | "2208B" | "2408B" => Some(100),
        "2208" | "2208A" => Some(200),
        _ => None,
    };
    let mso = is_mso(variant);
    ScopeCapabilities {
        block: true,
        rapid_block: Some(RapidBlockCapabilities {
            trigger_offset: true,
            timestamps: None,
            missed_triggers: false,
        }),
        triggers: TriggerCapabilities {
            model: TriggerModel::ChannelStates,
            simple: true,
            advanced: true,
            pulse_width: true,
            digital: mso,
            delay: true,
            holdoff: false,
        },
        ets: true,
        digital_ports: if mso { 2 } else { 0 },
        sig_gen: Some(SigGenCapabilities {
            api: SigGenApi::Legacy,
            arbitrary: true,
        }),
        aux_io: false,
        channel_leds: false,
        bandwidth_hz: bandwidth_mhz.map(|mhz| mhz * MHZ),
        bandwidth_cases: Vec::new(),
        bandwidth_limits: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_2208a_is_200_mhz_with_no_ports() {
        let caps = of("2208A");
        assert_eq!(caps.bandwidth_hz, Some(200 * MHZ));
        assert_eq!(caps.digital_ports, 0);
        assert_eq!(of("2206B MSO").digital_ports, 2);
        assert_eq!(of("2206B MSO").bandwidth_hz, Some(50 * MHZ));
    }
}
