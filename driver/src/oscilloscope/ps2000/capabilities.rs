//! Data sheets (picotech.com/download/datasheets/): 2104 and 2105,
//! picoscope-2100-handheld-oscilloscope-data-sheet.pdf (MM010.en-9);
//! 2202, picotech.com/download/manuals/PicoScope2202UsersGuide.pdf
//! (PS2000044-2); 2203 to 2205, picoscope-2203-2204-2205-datasheet.pdf
//! (MM012-5) and PicoScope2200.pdf (MM012.en-8); 2204A and 2205A,
//! PicoScope2200ASeriesDataSheet.pdf (MM051.en-7). Guide: PicoScope 2000
//! Series Programmer's Guide.

use crate::oscilloscope::models::{model, MHZ};
use pico_common::{
    ScopeCapabilities, SigGenApi, SigGenCapabilities, TriggerCapabilities, TriggerModel,
};

pub(super) fn of(variant: &str) -> ScopeCapabilities {
    let model = model(variant);
    let bandwidth_mhz = match model {
        "2104" => Some(10),
        "2105" => Some(25),
        "2202" => Some(2),
        "2203" => Some(5),
        "2204" | "2204A" => Some(10),
        "2205" | "2205A" => Some(25),
        _ => None,
    };
    ScopeCapabilities {
        block: true,
        rapid_block: None,
        triggers: TriggerCapabilities {
            model: TriggerModel::ChannelStates,
            simple: true,
            advanced: true,
            pulse_width: true,
            digital: false,
            delay: true,
            holdoff: false,
            window: true,
            runt: false,
            // The 2200 data sheet (MM012) gives EXT to its 50 MHz and
            // faster models only; every ps2000 model is slower.
            external: false,
            aux: false,
            streaming: false,
        },
        ets: !matches!(model, "2202"),
        digital_ports: 0,
        sig_gen: matches!(model, "2203" | "2204" | "2205" | "2204A" | "2205A").then_some(
            SigGenCapabilities {
                api: SigGenApi::Legacy,
                arbitrary: true,
            },
        ),
        aux_io: false,
        channel_leds: false,
        bandwidth_hz: bandwidth_mhz.map(|mhz| mhz * MHZ),
        bandwidth_cases: Vec::new(),
        bandwidth_limits: Vec::new(),
    }
}
