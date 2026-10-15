//! Data sheets (picotech.com/download/): 4824,
//! manuals/picoscope-4824-data-sheet.pdf (MM052.en-9); 4224A, 4424A and
//! 4824A, datasheets/picoscope-4000a-series-oscilloscope-data-sheet.pdf
//! (MM116.en-8); 4444, datasheets/picoscope-4444-data-sheet.pdf
//! (MM082.en-8); 4225A and 4425A, picoauto.com's PicoScope 4000 series
//! specifications. No model lists ETS. Guide: PicoScope 4000 Series (A
//! API), online edition.

use crate::oscilloscope::models::{at_range, model, MHZ};
use pico_common::{
    BandwidthLimit, PicoRange, RapidBlockCapabilities, ScopeCapabilities, SigGenApi,
    SigGenCapabilities, TriggerCapabilities, TriggerModel,
};

pub(super) fn of(variant: &str) -> ScopeCapabilities {
    let model = model(variant);
    let (bandwidth_hz, bandwidth_cases) = match model {
        "4824" | "4224A" | "4424A" | "4824A" => (
            Some(20 * MHZ),
            vec![at_range(PicoRange::X1_PROBE_20MV, 10 * MHZ)],
        ),
        "4225A" | "4425A" => (
            Some(20 * MHZ),
            vec![at_range(PicoRange::X1_PROBE_50MV, 10 * MHZ)],
        ),
        // With D9-BNC adaptors; PicoConnect 441 and 442 probes give 15
        // and 10 MHz.
        "4444" => (Some(20 * MHZ), Vec::new()),
        _ => (None, Vec::new()),
    };
    ScopeCapabilities {
        block: true,
        rapid_block: Some(RapidBlockCapabilities {
            trigger_offset: true,
            timestamps: None,
            missed_triggers: false,
        }),
        triggers: TriggerCapabilities {
            model: TriggerModel::ConditionLists,
            simple: true,
            advanced: true,
            pulse_width: true,
            digital: false,
            delay: true,
            holdoff: false,
            window: true,
            runt: true,
            external: true,
            aux: true,
            streaming: true,
        },
        ets: false,
        digital_ports: 0,
        sig_gen: matches!(model, "4824" | "4224A" | "4424A" | "4824A").then_some(
            SigGenCapabilities {
                api: SigGenApi::Legacy,
                arbitrary: true,
            },
        ),
        aux_io: false,
        channel_leds: false,
        bandwidth_hz,
        bandwidth_cases,
        bandwidth_limits: if model == "4444" {
            vec![BandwidthLimit::HZ_100K, BandwidthLimit::HZ_1M]
        } else {
            Vec::new()
        },
    }
}
