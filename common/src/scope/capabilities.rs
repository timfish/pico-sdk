use crate::{BandwidthLimit, PicoRange, PicoResolution};

/// What a scope can do beyond streaming, read when it opens. Filled from
/// the family's programmer's guide and the model's data sheet, and from
/// the driver where it reports a value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScopeCapabilities {
    pub block: bool,
    pub rapid_block: Option<RapidBlockCapabilities>,
    pub triggers: TriggerCapabilities,
    pub ets: bool,
    /// MSO ports of 8 digital inputs each.
    pub digital_ports: u8,
    pub sig_gen: Option<SigGenCapabilities>,
    pub aux_io: bool,
    /// An RGB LED by each channel input that `LedDriver` sets
    /// (3000E and 5000E). Asked of the driver at open.
    pub channel_leds: bool,
    /// Analogue bandwidth at -3 dB, from the model's data sheet, at its
    /// best. `None` for a model the tables do not list.
    pub bandwidth_hz: Option<u64>,
    /// Lower bandwidths the data sheet gives for some settings.
    pub bandwidth_cases: Vec<BandwidthCase>,
    /// Bandwidth limits a channel can take, beside full bandwidth.
    pub bandwidth_limits: Vec<BandwidthLimit>,
}

impl ScopeCapabilities {
    /// The bandwidth at `resolution` and `range`, on a 50 Ω input or on
    /// 1 MΩ.
    pub fn bandwidth_for(
        &self,
        resolution: Option<PicoResolution>,
        range: PicoRange,
        fifty_ohm: bool,
    ) -> Option<u64> {
        let full = self.bandwidth_hz?;
        let lower = self.bandwidth_cases.iter().filter_map(|case| {
            let applies = match case.when {
                BandwidthCondition::Resolution(r) => resolution == Some(r),
                BandwidthCondition::RangeUpTo(most) => {
                    range.to_nano_volts() <= most.to_nano_volts()
                }
                BandwidthCondition::HighImpedance => !fifty_ohm,
            };
            applies.then_some(case.hz)
        });
        Some(lower.fold(full, u64::min))
    }
}

/// A lower bandwidth under a condition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BandwidthCase {
    pub when: BandwidthCondition,
    pub hz: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BandwidthCondition {
    Resolution(PicoResolution),
    /// Ranges of this span or smaller.
    RangeUpTo(PicoRange),
    /// The 1 MΩ input of a unit that also has 50 Ω.
    HighImpedance,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RapidBlockCapabilities {
    /// The trigger time offset of each segment.
    pub trigger_offset: bool,
    pub timestamps: Option<TimestampCounter>,
    /// The count of trigger events missed between segments.
    pub missed_triggers: bool,
}

/// How a family's segment timestamps count; see
/// [`crate::SegmentTimestamp`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampCounter {
    SincePrevious,
    SinceFirst,
}

/// The two trigger models of the headers.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TriggerModel {
    /// Per-channel trigger states in one conditions struct.
    #[default]
    ChannelStates,
    /// Lists of conditions with an action (ps6000a, psospa, and the V2
    /// calls of ps3000a and ps5000a).
    ConditionLists,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TriggerCapabilities {
    pub model: TriggerModel,
    pub simple: bool,
    pub advanced: bool,
    pub pulse_width: bool,
    pub digital: bool,
    pub delay: bool,
    pub holdoff: bool,
}

/// The signal generator calls a family has.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigGenApi {
    /// One call sets the whole output.
    Legacy,
    /// One call per property, then an apply call (ps6000a, psospa).
    Properties,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigGenCapabilities {
    pub api: SigGenApi,
    pub arbitrary: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_lowest_case_that_applies_wins() {
        let caps = ScopeCapabilities {
            bandwidth_hz: Some(200_000_000),
            bandwidth_cases: vec![
                BandwidthCase {
                    when: BandwidthCondition::Resolution(PicoResolution::DR_16BIT),
                    hz: 60_000_000,
                },
                BandwidthCase {
                    when: BandwidthCondition::RangeUpTo(PicoRange::X1_PROBE_50MV),
                    hz: 100_000_000,
                },
            ],
            ..Default::default()
        };
        let at = |res, range| caps.bandwidth_for(Some(res), range, false);
        use PicoResolution::*;
        assert_eq!(at(DR_8BIT, PicoRange::X1_PROBE_1V), Some(200_000_000));
        assert_eq!(at(DR_8BIT, PicoRange::X1_PROBE_20MV), Some(100_000_000));
        assert_eq!(at(DR_16BIT, PicoRange::X1_PROBE_20MV), Some(60_000_000));
        assert_eq!(
            ScopeCapabilities::default().bandwidth_for(None, PicoRange::X1_PROBE_1V, false),
            None
        );
    }
}
