use crate::{BandwidthLimit, PicoRange, PicoResolution};

/// What a scope can do other than streaming, read when it opens
///
/// - The values come from the programmer's guide of the family, from the data sheet of the model, and from the driver when it reports a value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScopeCapabilities {
    /// Whether the scope can do block captures
    pub block: bool,
    /// What the scope can do in rapid block mode, or `None` when it has no rapid block mode
    pub rapid_block: Option<RapidBlockCapabilities>,
    /// The triggers of the scope
    pub triggers: TriggerCapabilities,
    /// Whether the scope can do equivalent-time sampling
    pub ets: bool,
    /// The number of MSO ports, each with 8 digital inputs
    pub digital_ports: u8,
    /// The signal generator, or `None` when the scope has none
    pub sig_gen: Option<SigGenCapabilities>,
    /// Whether the scope has an AUX I/O connector that you can set ([`AuxIoMode`](crate::AuxIoMode))
    pub aux_io: bool,
    /// Whether the scope has an RGB LED next to each channel input (3000E and 5000E)
    ///
    /// - The value comes from the driver at open.
    pub channel_leds: bool,
    /// The best analog bandwidth at -3 dB, in hertz, from the data sheet of the model
    ///
    /// - `None` for a model that the tables of this crate do not list.
    pub bandwidth_hz: Option<u64>,
    /// The lower bandwidths that the data sheet gives for some settings
    pub bandwidth_cases: Vec<BandwidthCase>,
    /// The bandwidth limits that a channel can use, other than full bandwidth
    pub bandwidth_limits: Vec<BandwidthLimit>,
}

impl ScopeCapabilities {
    /// The bandwidth in hertz at `resolution` and `range`, on a 50 Ω input or on a 1 MΩ input
    ///
    /// - It is the lowest of [`ScopeCapabilities::bandwidth_hz`] and each case of [`ScopeCapabilities::bandwidth_cases`] that applies.
    /// - A range case applies only to ×1 and ×10 probe ranges.
    /// - Returns `None` when [`ScopeCapabilities::bandwidth_hz`] is `None`.
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
                // A probe's range says nothing of the input's own
                // range, so no case of it applies.
                BandwidthCondition::RangeUpTo(most) => range
                    .nano_volts()
                    .zip(most.nano_volts())
                    .is_some_and(|(r, m)| r <= m),
                BandwidthCondition::HighImpedance => !fifty_ohm,
            };
            applies.then_some(case.hz)
        });
        Some(lower.fold(full, u64::min))
    }
}

/// A lower bandwidth that applies under a condition
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BandwidthCase {
    /// The condition
    pub when: BandwidthCondition,
    /// The bandwidth under the condition, in hertz
    pub hz: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// When a [`BandwidthCase`] applies
pub enum BandwidthCondition {
    /// At this resolution
    Resolution(PicoResolution),
    /// On ranges of this span or smaller
    RangeUpTo(PicoRange),
    /// On the 1 MΩ input of a unit that also has a 50 Ω input
    HighImpedance,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
/// What a scope can do in rapid block mode
pub struct RapidBlockCapabilities {
    /// Whether the driver gives the trigger time offset of each segment
    pub trigger_offset: bool,
    /// How the segment timestamps count, or `None` when the driver gives no timestamps
    pub timestamps: Option<TimestampCounter>,
    /// Whether the driver gives the number of trigger events missed between segments
    pub missed_triggers: bool,
}

/// How the segment timestamps of a family count
///
/// - See [`SegmentTimestamp`](crate::SegmentTimestamp).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampCounter {
    /// From the trigger of the segment before, as [`SegmentTimestamp::SincePrevious`](crate::SegmentTimestamp::SincePrevious)
    SincePrevious,
    /// From a start that is not fixed, as [`SegmentTimestamp::SinceFirst`](crate::SegmentTimestamp::SinceFirst)
    SinceFirst,
}

/// The two trigger models of the driver headers
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TriggerModel {
    /// One trigger state for each channel, in one conditions struct
    #[default]
    ChannelStates,
    /// Lists of conditions, each with an action (ps6000a, psospa, and the V2 calls of ps3000a and ps5000a)
    ConditionLists,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
/// The triggers that a scope has
pub struct TriggerCapabilities {
    /// The trigger model of the family
    pub model: TriggerModel,
    /// Whether the scope has a simple trigger
    pub simple: bool,
    /// Whether the scope has advanced triggers
    pub advanced: bool,
    /// Whether the scope has a pulse width qualifier
    pub pulse_width: bool,
    /// Whether a trigger can use the digital inputs (MSO models only)
    pub digital: bool,
    /// Whether the scope has a trigger delay
    pub delay: bool,
    /// Whether the scope has a holdoff counter, in samples
    pub holdoff: bool,
    /// Whether the scope has the window directions: inside, outside, enter and exit
    pub window: bool,
    /// Whether the scope has the positive and negative runt directions
    pub runt: bool,
    /// Whether the scope has an EXT trigger input
    ///
    /// - For ps2000 and ps2000a, the value comes from the data sheet of the model.
    /// - For the other families, the value comes from the API of the family.
    ///   On a model with no EXT input, the driver returns an error.
    pub external: bool,
    /// Whether the scope has an AUX trigger input
    ///
    /// - The value follows the same rule as [`TriggerCapabilities::external`].
    pub aux: bool,
    /// Whether a stream marks the sample where the trigger fired
    pub streaming: bool,
}

/// The signal generator calls of a family
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigGenApi {
    /// One call sets the whole output
    Legacy,
    /// One call for each property, then an apply call (ps6000a, psospa)
    Properties,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// What the signal generator of a scope can do
pub struct SigGenCapabilities {
    /// The signal generator calls of the family
    pub api: SigGenApi,
    /// Whether it has an arbitrary waveform generator (AWG)
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
            at(DR_8BIT, PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_5V),
            Some(200_000_000),
            "a probe's range takes no range case"
        );
        assert_eq!(
            ScopeCapabilities::default().bandwidth_for(None, PicoRange::X1_PROBE_1V, false),
            None
        );
    }
}
