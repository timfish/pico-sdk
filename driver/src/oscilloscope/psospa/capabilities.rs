//! Data sheets (picotech.com/download/datasheets/): 3000E,
//! picoscope-3000e-series-data-sheet.pdf (MM131.en-3); 5000E,
//! picoscope-5000e-series-data-sheet.pdf (MM145.en-1). Every model has a
//! 20 MHz function generator and a 200 MS/s AWG, and no ETS. The 3417E
//! and 3418E offer their 350 and 500 MHz limits at 8 bits only. Guide:
//! psospa API Programmer's Guide, online edition.

use crate::oscilloscope::models::{at_resolution, is_mso, model, MHZ};
use pico_common::{
    BandwidthLimit, PicoResolution, RapidBlockCapabilities, ScopeCapabilities, SigGenApi,
    SigGenCapabilities, TimestampCounter, TriggerCapabilities, TriggerModel,
};
use pico_sys_dynamic::psospa::{
    enPicoBandwidthLimiter, enPicoBandwidthLimiter_PICO_BW_100MHZ,
    enPicoBandwidthLimiter_PICO_BW_200MHZ, enPicoBandwidthLimiter_PICO_BW_20MHZ,
    enPicoBandwidthLimiter_PICO_BW_350MHZ, enPicoBandwidthLimiter_PICO_BW_500MHZ,
    enPicoBandwidthLimiter_PICO_BW_50MHZ, enPicoBandwidthLimiter_PICO_BW_60MHZ,
};

pub(super) const LIMITER_CODES: [(BandwidthLimit, enPicoBandwidthLimiter); 7] = [
    (BandwidthLimit::HZ_20M, enPicoBandwidthLimiter_PICO_BW_20MHZ),
    (BandwidthLimit::HZ_50M, enPicoBandwidthLimiter_PICO_BW_50MHZ),
    (BandwidthLimit::HZ_60M, enPicoBandwidthLimiter_PICO_BW_60MHZ),
    (
        BandwidthLimit::HZ_100M,
        enPicoBandwidthLimiter_PICO_BW_100MHZ,
    ),
    (
        BandwidthLimit::HZ_200M,
        enPicoBandwidthLimiter_PICO_BW_200MHZ,
    ),
    (
        BandwidthLimit::HZ_350M,
        enPicoBandwidthLimiter_PICO_BW_350MHZ,
    ),
    (
        BandwidthLimit::HZ_500M,
        enPicoBandwidthLimiter_PICO_BW_500MHZ,
    ),
];

fn limits_mhz(mhz: &[u32]) -> Vec<BandwidthLimit> {
    mhz.iter()
        .map(|m| BandwidthLimit::from_hz(m * 1_000_000))
        .collect()
}

pub(super) fn of(variant: &str) -> ScopeCapabilities {
    let model = model(variant);
    // An E+ model's 8-bit bandwidth is higher than its 16-bit one.
    let (bandwidth_mhz, sixteen_bit_mhz, limits): (Option<u64>, Option<u64>, &[u32]) = match model {
        "3415E" => (Some(100), None, &[20, 50, 100]),
        "3416E" => (Some(200), None, &[20, 50, 100, 200]),
        "3417E" => (Some(350), None, &[20, 50, 100, 200, 350]),
        "3418E" => (Some(500), None, &[20, 50, 100, 200, 350, 500]),
        "5462E" => (Some(60), None, &[20, 60]),
        "5463E" => (Some(100), None, &[20, 60, 100]),
        "5464E" => (Some(200), None, &[20, 60, 100, 200]),
        "5462E+" => (Some(200), Some(60), &[20, 60, 100, 200]),
        "5463E+" => (Some(350), Some(100), &[20, 60, 100, 200, 350]),
        "5464E+" => (Some(500), Some(200), &[20, 60, 100, 200, 350, 500]),
        _ => (None, None, &[]),
    };
    let mso = is_mso(variant);
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
            digital: mso,
            delay: true,
            holdoff: true,
            window: true,
            runt: true,
            external: false,
            aux: true,
            streaming: true,
        },
        ets: false,
        digital_ports: if mso { 2 } else { 0 },
        sig_gen: Some(SigGenCapabilities {
            api: SigGenApi::Properties,
            arbitrary: true,
        }),
        aux_io: true,
        channel_leds: false,
        bandwidth_hz: bandwidth_mhz.map(|mhz| mhz * MHZ),
        bandwidth_cases: sixteen_bit_mhz
            .map(|mhz| at_resolution(PicoResolution::DR_16BIT, mhz * MHZ))
            .into_iter()
            .collect(),
        bandwidth_limits: limits_mhz(limits),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_5464e_is_200_mhz() {
        let caps = of("5464E+MSO");
        assert_eq!(caps.bandwidth_hz, Some(200 * MHZ));
        assert!(caps.bandwidth_cases.is_empty());
        assert_eq!(caps.digital_ports, 2);
        assert_eq!(caps.bandwidth_limits.len(), 4);

        let plus = of("5464E+");
        assert_eq!(plus.bandwidth_hz, Some(500 * MHZ));
        assert_eq!(plus.bandwidth_cases.len(), 1);
        assert_eq!(plus.digital_ports, 0);
    }

    #[test]
    fn every_limit_has_a_code() {
        for variant in ["3418E", "5464E+"] {
            for limit in of(variant).bandwidth_limits {
                assert!(LIMITER_CODES.iter().any(|(l, _)| *l == limit), "{limit}");
            }
        }
    }
}
