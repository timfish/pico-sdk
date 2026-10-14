//! Data sheets (picotech.com/download/datasheets/): 4224, 4424, 4226 and
//! 4227, PicoScope4226_7Datasheet.pdf (MM002.en-6); 4262,
//! picoscope-4262-data-sheet.pdf (MM030.en-12). No Pico document for the
//! 4223 and 4423 was found, so their figures are left out. Guide:
//! PicoScope 4000 Series Programmer's Guide.

use crate::oscilloscope::models::{at_range, model, MHZ};
use pico_common::{
    BandwidthLimit, PicoRange, RapidBlockCapabilities, ScopeCapabilities, SigGenApi,
    SigGenCapabilities, TriggerCapabilities, TriggerModel,
};

pub(super) fn of(variant: &str) -> ScopeCapabilities {
    let model = model(variant);
    let (bandwidth_hz, bandwidth_cases) = match model {
        "4224" | "4424" => (
            Some(20 * MHZ),
            vec![at_range(PicoRange::X1_PROBE_50MV, 10 * MHZ)],
        ),
        "4226" => (Some(50 * MHZ), Vec::new()),
        "4227" => (Some(100 * MHZ), Vec::new()),
        "4262" => (
            Some(5 * MHZ),
            vec![
                at_range(PicoRange::X1_PROBE_20MV, 4 * MHZ),
                at_range(PicoRange::X1_PROBE_10MV, 3 * MHZ),
            ],
        ),
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
            model: TriggerModel::ChannelStates,
            simple: true,
            advanced: true,
            pulse_width: true,
            digital: false,
            delay: true,
            holdoff: false,
        },
        ets: matches!(model, "4226" | "4227"),
        digital_ports: 0,
        sig_gen: matches!(model, "4226" | "4227" | "4262").then_some(SigGenCapabilities {
            api: SigGenApi::Legacy,
            arbitrary: true,
        }),
        aux_io: false,
        bandwidth_hz,
        bandwidth_cases,
        bandwidth_limits: if model == "4262" {
            vec![BandwidthLimit::HZ_200K]
        } else {
            Vec::new()
        },
    }
}
