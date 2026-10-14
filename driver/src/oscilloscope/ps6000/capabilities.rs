//! Data sheets (picotech.com/download/): 6402 to 6404,
//! datasheets/PicoScope6000SeriesDataSheet.pdf (MM023.en-6); A and B,
//! manuals/PicoScope6000ABSeriesDataSheet.pdf (MM023.en-10); C and D,
//! datasheets/PicoScope6000CDSeriesDataSheet.pdf (MM050-6); 6407,
//! datasheets/picoscope-6407-digitizer-data-sheet.pdf (MM027.en-6).
//! Guide: PicoScope 6000 Series Programmer's Guide.

use crate::oscilloscope::models::{at_range, model, MHZ};
use pico_common::{
    BandwidthLimit, PicoRange, RapidBlockCapabilities, ScopeCapabilities, SigGenApi,
    SigGenCapabilities, TriggerCapabilities, TriggerModel,
};

pub(super) fn of(variant: &str) -> ScopeCapabilities {
    let model = model(variant);
    let base = model.get(..4).unwrap_or(model);
    let lettered = model.len() > 4;
    let (bandwidth_hz, bandwidth_cases) = match (base, lettered) {
        ("6402", false) | ("6403", false) => (
            Some(350 * MHZ),
            vec![at_range(PicoRange::X1_PROBE_50MV, 250 * MHZ)],
        ),
        ("6402", true) => (
            Some(250 * MHZ),
            vec![at_range(PicoRange::X1_PROBE_50MV, 200 * MHZ)],
        ),
        ("6403", true) => (
            Some(350 * MHZ),
            vec![at_range(PicoRange::X1_PROBE_50MV, 250 * MHZ)],
        ),
        ("6404", _) => (Some(500 * MHZ), Vec::new()),
        ("6407", _) => (Some(1000 * MHZ), Vec::new()),
        _ => (None, Vec::new()),
    };
    let bandwidth_limits = match base {
        "6402" | "6403" => vec![BandwidthLimit::HZ_20M],
        "6404" => vec![BandwidthLimit::HZ_25M],
        _ => Vec::new(),
    };
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
            digital: false,
            delay: true,
            holdoff: false,
        },
        // The first 6404 and the 6407 have no ETS.
        ets: !matches!((base, lettered), ("6404", false) | ("6407", _)),
        digital_ports: 0,
        // A and C models have a function generator only.
        sig_gen: Some(SigGenCapabilities {
            api: SigGenApi::Legacy,
            arbitrary: !(model.ends_with('A') || model.ends_with('C')),
        }),
        aux_io: false,
        bandwidth_hz,
        bandwidth_cases,
        bandwidth_limits,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_set_bandwidth_and_awg() {
        assert_eq!(of("6402C").bandwidth_hz, Some(250 * MHZ));
        assert_eq!(of("6402").bandwidth_hz, Some(350 * MHZ));
        assert_eq!(of("6404D").bandwidth_limits, vec![BandwidthLimit::HZ_25M]);
        assert!(!of("6403C").sig_gen.unwrap().arbitrary);
        assert!(of("6403D").sig_gen.unwrap().arbitrary);
        assert!(!of("6404").ets && of("6404B").ets);
    }
}
