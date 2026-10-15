//! Data sheets (picotech.com/download/datasheets/): 3204A to 3207B and
//! MSO, PicoScope3200ABSeriesDataSheet.pdf (MM026.en-7) and
//! picotech.com/download/manuals/ps3000ab.en-5.pdf; 3404A to 3406B,
//! PicoScope3400.pdf (MM037.en-6); 3000D and D MSO,
//! picoscope-3000d-series-data-sheet.pdf and
//! PicoScope3000DDMSOSeriesDataSheet-en.pdf (MM054.en-12). Guide:
//! ps3000apg.en-15 (2022).

use crate::oscilloscope::models::{is_mso, model, MHZ};
use pico_common::{
    BandwidthLimit, RapidBlockCapabilities, ScopeCapabilities, SigGenApi, SigGenCapabilities,
    TimestampCounter, TriggerCapabilities, TriggerModel,
};

pub(super) fn of(variant: &str) -> ScopeCapabilities {
    let model = model(variant);
    let bandwidth_mhz = match model {
        "3203D" | "3403D" => Some(50),
        "3204A" | "3204B" | "3204" | "3404A" | "3404B" => Some(60),
        "3204D" | "3404D" => Some(70),
        "3205A" | "3205B" | "3205" | "3405A" | "3405B" | "3205D" | "3405D" => Some(100),
        "3206A" | "3206B" | "3206" | "3406A" | "3406B" | "3206D" | "3406D" => Some(200),
        "3207A" | "3207B" => Some(250),
        _ => None,
    };
    // The 3400 A/B and 3000D have a 20 MHz limiter; the 3200 A/B and
    // MSO do not.
    let limiter = model.starts_with("34") || model.ends_with('D');
    let mso = is_mso(variant);
    ScopeCapabilities {
        block: true,
        // Only the 3207A/B count trigger timestamps (§4.18). The guide
        // gives no layout; the struct is ps5000a's, read the same way.
        rapid_block: Some(RapidBlockCapabilities {
            trigger_offset: true,
            timestamps: matches!(model, "3207A" | "3207B")
                .then_some(TimestampCounter::SincePrevious),
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
        // A models have a function generator only; B, MSO and D models
        // an AWG too.
        sig_gen: Some(SigGenCapabilities {
            api: SigGenApi::Legacy,
            arbitrary: !model.ends_with('A'),
        }),
        aux_io: false,
        channel_leds: false,
        bandwidth_hz: bandwidth_mhz.map(|mhz| mhz * MHZ),
        bandwidth_cases: Vec::new(),
        bandwidth_limits: if limiter {
            vec![BandwidthLimit::HZ_20M]
        } else {
            Vec::new()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_3400_and_d_models_limit() {
        assert!(of("3206B").bandwidth_limits.is_empty());
        assert!(of("3206 MSO").bandwidth_limits.is_empty());
        assert_eq!(
            of("3406DMSO").bandwidth_limits,
            vec![BandwidthLimit::HZ_20M]
        );
        assert_eq!(of("3405A").bandwidth_limits, vec![BandwidthLimit::HZ_20M]);
        assert_eq!(of("3406DMSO").digital_ports, 2);
        assert!(!of("3207A").sig_gen.unwrap().arbitrary);
        assert!(of("3207B").rapid_block.unwrap().timestamps.is_some());
        assert!(of("3206B").rapid_block.unwrap().timestamps.is_none());
    }
}
