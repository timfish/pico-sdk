//! Data sheet: picotech.com/download/datasheets/
//! picoscope-6000e-series-data-sheet.pdf (MM105.en-9), and the
//! specification table of picotech.com's 6000E series page. Every model
//! has a 50 MHz function generator and a 200 MS/s AWG, and no ETS.
//! Guide: ps6000apg-9 (2026).

use crate::oscilloscope::models::{at_high_impedance, model, MHZ};
use pico_common::{
    BandwidthLimit, RapidBlockCapabilities, ScopeCapabilities, SigGenApi, SigGenCapabilities,
    TimestampCounter, TriggerCapabilities, TriggerModel,
};
use pico_sys_dynamic::ps6000a::{
    enPicoBandwidthLimiter, enPicoBandwidthLimiter_PICO_BW_200MHZ,
    enPicoBandwidthLimiter_PICO_BW_20MHZ,
};

pub(super) const LIMITER_CODES: [(BandwidthLimit, enPicoBandwidthLimiter); 2] = [
    (BandwidthLimit::HZ_20M, enPicoBandwidthLimiter_PICO_BW_20MHZ),
    (
        BandwidthLimit::HZ_200M,
        enPicoBandwidthLimiter_PICO_BW_200MHZ,
    ),
];

pub(super) fn of(variant: &str) -> ScopeCapabilities {
    let model = model(variant);
    let (bandwidth_hz, bandwidth_cases, bandwidth_limits) = match model {
        "6403E" => (Some(300 * MHZ), Vec::new(), vec![BandwidthLimit::HZ_20M]),
        "6404E" | "6424E" | "6804E" | "6824E" => {
            (Some(500 * MHZ), Vec::new(), vec![BandwidthLimit::HZ_20M])
        }
        "6405E" | "6425E" => (
            Some(750 * MHZ),
            vec![at_high_impedance(500 * MHZ)],
            vec![BandwidthLimit::HZ_20M, BandwidthLimit::HZ_200M],
        ),
        "6406E" | "6426E" => (
            Some(1000 * MHZ),
            vec![at_high_impedance(500 * MHZ)],
            vec![BandwidthLimit::HZ_20M, BandwidthLimit::HZ_200M],
        ),
        // 50 Ω only; 2.5 GHz on ±500 mV.
        "6428E-D" => (Some(3000 * MHZ), Vec::new(), Vec::new()),
        _ => (None, Vec::new(), Vec::new()),
    };
    // Ports for the optional TA369 pods. Whether the 6428E-D takes them
    // is not confirmed.
    let digital_ports = if model == "6428E-D" { 0 } else { 2 };
    ScopeCapabilities {
        block: true,
        rapid_block: Some(RapidBlockCapabilities {
            trigger_offset: true,
            timestamps: Some(TimestampCounter::SinceFirst),
            missed_triggers: true,
        }),
        triggers: TriggerCapabilities {
            model: TriggerModel::ConditionLists,
            simple: true,
            advanced: true,
            pulse_width: true,
            digital: digital_ports > 0,
            delay: true,
            holdoff: true,
        },
        ets: false,
        digital_ports,
        sig_gen: Some(SigGenCapabilities {
            api: SigGenApi::Properties,
            arbitrary: true,
        }),
        aux_io: true,
        channel_leds: false,
        bandwidth_hz,
        bandwidth_cases,
        bandwidth_limits,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::PicoRange;

    #[test]
    fn a_1_ghz_model_gives_500_mhz_on_1_megohm() {
        let caps = of("6426E");
        assert_eq!(
            caps.bandwidth_for(None, PicoRange::X1_PROBE_1V, true),
            Some(1000 * MHZ)
        );
        assert_eq!(
            caps.bandwidth_for(None, PicoRange::X1_PROBE_1V, false),
            Some(500 * MHZ)
        );
    }
}
