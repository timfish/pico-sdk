//! One trigger description for all scope families.
//! The driver of each family turns it into its own model.
//! The model is one trigger state for each channel in one conditions struct (ps2000 to ps6000), or lists of conditions (ps4000a, the V2 calls of ps5000a, ps6000a, psospa).
//! Guides: ps2000a §2.5, §3.56–3.61; ps5000a §3.4, §4.66–4.74; ps6000a §2.5, §3.54–3.64; psospa topic: triggering.

use crate::{PicoChannel, PicoError, PicoResult, PicoStatus, TriggerCapabilities};
use std::time::Duration;

/// A source of a trigger
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerSource {
    /// An analog channel
    Channel(PicoChannel),
    /// The EXT input, on models that have one
    External,
    /// The AUX trigger input
    Aux,
    /// The output of the pulse width qualifier
    PulseWidthQualifier,
    /// The digital trigger on the MSO ports, as the `digital` field of [`TriggerConfig::Advanced`] sets it
    Digital,
}

/// What a source must do to be true
///
/// - The level directions use the upper threshold.
///   `RisingOrFalling` uses the lower threshold for rising edges, when there is one.
/// - The other directions are window directions, and use both thresholds.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdDirection {
    /// Above the threshold
    Above,
    /// Below the threshold
    Below,
    /// A rising edge
    Rising,
    /// A falling edge
    Falling,
    /// A rising or a falling edge
    RisingOrFalling,
    /// Inside the window
    Inside,
    /// Outside the window
    Outside,
    /// Into the window
    Enter,
    /// Out of the window
    Exit,
    /// Into or out of the window
    EnterOrExit,
    /// Into the window from below, and back, without crossing it
    PositiveRunt,
    /// Into the window from above, and back, without crossing it
    NegativeRunt,
}

impl ThresholdDirection {
    /// Whether the direction is a level or a window direction
    pub fn mode(self) -> ThresholdMode {
        use ThresholdDirection::*;
        match self {
            Above | Below | Rising | Falling | RisingOrFalling => ThresholdMode::Level,
            _ => ThresholdMode::Window,
        }
    }

    /// Whether the direction is a runt direction
    pub fn is_runt(self) -> bool {
        matches!(
            self,
            ThresholdDirection::PositiveRunt | ThresholdDirection::NegativeRunt
        )
    }

    /// The code of the direction in the driver headers
    ///
    /// - The code is the same in the header of each family.
    pub fn code(self) -> u32 {
        use ThresholdDirection::*;
        match self {
            Above | Inside => 0,
            Below | Outside => 1,
            Rising | Enter => 2,
            Falling | Exit => 3,
            RisingOrFalling | EnterOrExit => 4,
            PositiveRunt => 9,
            NegativeRunt => 10,
        }
    }
}

/// Whether a trigger uses one threshold or a window of two
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdMode {
    /// One threshold
    Level,
    /// A window of two thresholds
    Window,
}

impl ThresholdMode {
    /// The code of the mode in the driver headers
    pub fn code(self) -> u32 {
        match self {
            ThresholdMode::Level => 0,
            ThresholdMode::Window => 1,
        }
    }
}

/// A threshold in ADC counts, with the hysteresis that arms it again, also in ADC counts
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Threshold {
    /// The threshold, in ADC counts
    pub adc: i16,
    /// The hysteresis, in ADC counts
    pub hysteresis: u16,
}

impl Threshold {
    /// Makes a threshold
    pub fn new(adc: i16, hysteresis: u16) -> Threshold {
        Threshold { adc, hysteresis }
    }
}

/// The state that a source must have in a trigger condition
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerState {
    /// The source can have any state
    DontCare,
    /// The source must be true
    True,
    /// The source must be false
    False,
}

impl TriggerState {
    /// The code of the state in the driver headers
    pub fn code(self) -> u32 {
        match self {
            TriggerState::DontCare => 0,
            TriggerState::True => 1,
            TriggerState::False => 2,
        }
    }
}

/// What a digital input must do in a trigger
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalDirection {
    /// The input can do anything
    DontCare,
    /// The input is low
    Low,
    /// The input is high
    High,
    /// A rising edge
    Rising,
    /// A falling edge
    Falling,
    /// A rising or a falling edge
    RisingOrFalling,
}

impl DigitalDirection {
    /// The code of the direction in the driver headers
    pub fn code(self) -> u32 {
        match self {
            DigitalDirection::DontCare => 0,
            DigitalDirection::Low => 1,
            DigitalDirection::High => 2,
            DigitalDirection::Rising => 3,
            DigitalDirection::Falling => 4,
            DigitalDirection::RisingOrFalling => 5,
        }
    }
}

/// A digital input of the MSO ports, and what it must do
///
/// - Bits 0 to 7 are on port 0, and bits 8 to 15 are on port 1.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitalBit {
    /// The input number, 0 to 15
    pub bit: u8,
    /// What the input must do
    pub direction: DigitalDirection,
}

impl DigitalBit {
    /// Makes a digital bit
    pub fn new(bit: u8, direction: DigitalDirection) -> DigitalBit {
        DigitalBit { bit, direction }
    }

    /// The port of the input: 0 or 1
    pub fn port(self) -> u8 {
        self.bit / 8
    }
}

/// The pulse widths that a pulse width qualifier accepts
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseWidthType {
    /// Shorter than `lower`
    LessThan,
    /// Longer than `lower`
    GreaterThan,
    /// From `lower` to `upper`
    InRange,
    /// Not from `lower` to `upper`
    OutOfRange,
}

impl PulseWidthType {
    /// The code of the type in the driver headers
    pub fn code(self) -> u32 {
        match self {
            PulseWidthType::LessThan => 1,
            PulseWidthType::GreaterThan => 2,
            PulseWidthType::InRange => 3,
            PulseWidthType::OutOfRange => 4,
        }
    }
}

/// The thresholds and direction of one source
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelTrigger {
    /// The source: a channel, `External` or `Aux`
    pub source: TriggerSource,
    /// What the source must do
    pub direction: ThresholdDirection,
    /// The upper threshold, or the only threshold of a level direction
    pub upper: Threshold,
    /// The lower threshold of a window, or `None` for a level direction
    pub lower: Option<Threshold>,
}

impl ChannelTrigger {
    /// Makes an edge or level trigger on one threshold
    pub fn level(
        source: TriggerSource,
        direction: ThresholdDirection,
        adc: i16,
        hysteresis: u16,
    ) -> ChannelTrigger {
        ChannelTrigger {
            source,
            direction,
            upper: Threshold::new(adc, hysteresis),
            lower: None,
        }
    }

    /// Makes a window or runt trigger with two thresholds
    pub fn window(
        source: TriggerSource,
        direction: ThresholdDirection,
        lower: Threshold,
        upper: Threshold,
    ) -> ChannelTrigger {
        ChannelTrigger {
            source,
            direction,
            upper,
            lower: Some(lower),
        }
    }

    /// The lower threshold to give the driver
    ///
    /// - It is the lower threshold of a window, or else the upper threshold.
    pub fn lower_or_upper(&self) -> Threshold {
        self.lower.unwrap_or(self.upper)
    }
}

/// Trigger conditions: an OR of AND terms
///
/// - Each inner list is one term, and all of its states must be true.
/// - The trigger fires when one or more terms are true.
pub type TriggerConditions = Vec<Vec<(TriggerSource, TriggerState)>>;

/// A pulse width qualifier
///
/// - It is true for a pulse of the given width, in the given direction, on the sources that its conditions name.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseWidthQualifier {
    /// The sources that the qualifier uses
    pub conditions: TriggerConditions,
    /// The direction of the pulse
    pub direction: ThresholdDirection,
    /// The lower pulse width, in sample intervals
    pub lower: u32,
    /// The upper pulse width, in sample intervals
    ///
    /// - Only [`PulseWidthType::InRange`] and [`PulseWidthType::OutOfRange`] use it.
    pub upper: u32,
    /// The pulse widths that the qualifier accepts
    pub kind: PulseWidthType,
    /// The digital inputs that the qualifier times, on MSO models
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub digital: Vec<DigitalBit>,
}

/// One trigger description for all families
///
/// - A family refuses a term that it does not have, with [`PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE`] or a status that names the term.
/// - [`TriggerConfig::check`] gives the same errors from the capabilities only, before any driver call.
///
/// ```
/// use pico_common::{PicoChannel, ThresholdDirection, TriggerConfig};
///
/// let trigger = TriggerConfig::edge(PicoChannel::A, 1_000, ThresholdDirection::Rising);
/// assert_eq!(trigger.enables(), (true, false));
/// assert_eq!(trigger.delay(), 0);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub enum TriggerConfig {
    /// No trigger
    #[default]
    None,
    /// An edge or level on one source, with the call that each family has
    Simple {
        /// The source: a channel, `External` or `Aux`
        source: TriggerSource,
        /// The threshold, in ADC counts
        threshold_adc: i16,
        /// The direction: `Above`, `Below`, `Rising`, `Falling` or `RisingOrFalling`
        direction: ThresholdDirection,
        /// The delay after the trigger event, in sample intervals
        delay: u64,
        /// The time after which the scope triggers with no event, or `None` to wait with no end
        auto_trigger: Option<Duration>,
    },
    /// Triggers on more than one source, joined by conditions
    Advanced {
        /// The thresholds and direction of each source
        channels: Vec<ChannelTrigger>,
        /// The conditions that join the sources
        conditions: TriggerConditions,
        /// The digital inputs of the trigger, on MSO models
        digital: Vec<DigitalBit>,
        /// A pulse width qualifier
        pulse_width: Option<PulseWidthQualifier>,
        /// The delay after the trigger event, in sample intervals
        delay: u64,
        /// The holdoff, in samples, on families with a holdoff counter
        holdoff: Option<u64>,
        /// The time after which the scope triggers with no event, or `None` to wait with no end
        auto_trigger: Option<Duration>,
    },
}

/// A kind of trigger term, for errors and for the question of what a family has
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerKind {
    /// Simple trigger
    Simple,
    /// Advanced trigger
    Advanced,
    /// Window trigger
    Window,
    /// Runt trigger
    Runt,
    /// Pulse width qualifier
    PulseWidth,
    /// Digital trigger
    Digital,
    /// Trigger delay
    Delay,
    /// Trigger holdoff
    Holdoff,
    /// EXT trigger input
    External,
    /// AUX trigger input
    Aux,
}

impl TriggerKind {
    /// The name of the kind, such as `"window trigger"`
    pub fn name(self) -> &'static str {
        match self {
            TriggerKind::Simple => "simple trigger",
            TriggerKind::Advanced => "advanced trigger",
            TriggerKind::Window => "window trigger",
            TriggerKind::Runt => "runt trigger",
            TriggerKind::PulseWidth => "pulse width qualifier",
            TriggerKind::Digital => "digital trigger",
            TriggerKind::Delay => "trigger delay",
            TriggerKind::Holdoff => "trigger holdoff",
            TriggerKind::External => "EXT trigger input",
            TriggerKind::Aux => "AUX trigger input",
        }
    }
}

impl TriggerCapabilities {
    /// Whether the scope has this kind of trigger term
    pub fn has(&self, kind: TriggerKind) -> bool {
        match kind {
            TriggerKind::Simple => self.simple,
            TriggerKind::Advanced => self.advanced,
            TriggerKind::Window => self.window,
            TriggerKind::Runt => self.runt,
            TriggerKind::PulseWidth => self.pulse_width,
            TriggerKind::Digital => self.digital,
            TriggerKind::Delay => self.delay,
            TriggerKind::Holdoff => self.holdoff,
            TriggerKind::External => self.external,
            TriggerKind::Aux => self.aux,
        }
    }
}

fn refuse(status: PicoStatus, what: &str) -> PicoError {
    PicoError::from_status(status, what)
}

fn source_kind(source: TriggerSource) -> Option<TriggerKind> {
    match source {
        TriggerSource::External => Some(TriggerKind::External),
        TriggerSource::Aux => Some(TriggerKind::Aux),
        _ => None,
    }
}

impl TriggerConfig {
    /// Makes a simple trigger on a channel, with no delay and no auto trigger
    pub fn edge(channel: PicoChannel, threshold_adc: i16, direction: ThresholdDirection) -> Self {
        TriggerConfig::Simple {
            source: TriggerSource::Channel(channel),
            threshold_adc,
            direction,
            delay: 0,
            auto_trigger: None,
        }
    }

    /// Makes an advanced trigger that fires when one or more of `channels` fires
    pub fn any_of(channels: Vec<ChannelTrigger>) -> Self {
        let conditions = channels
            .iter()
            .map(|c| vec![(c.source, TriggerState::True)])
            .collect();
        TriggerConfig::Advanced {
            channels,
            conditions,
            digital: Vec::new(),
            pulse_width: None,
            delay: 0,
            holdoff: None,
            auto_trigger: None,
        }
    }

    /// Makes an advanced trigger that fires when all of `channels` are true at the same time
    pub fn all_of(channels: Vec<ChannelTrigger>) -> Self {
        let conditions = vec![channels
            .iter()
            .map(|c| (c.source, TriggerState::True))
            .collect()];
        TriggerConfig::Advanced {
            channels,
            conditions,
            digital: Vec::new(),
            pulse_width: None,
            delay: 0,
            holdoff: None,
            auto_trigger: None,
        }
    }

    /// Whether there is no trigger
    pub fn is_none(&self) -> bool {
        matches!(self, TriggerConfig::None)
    }

    /// Whether the config turns on the trigger, and whether it turns on the pulse width qualifier
    pub fn enables(&self) -> (bool, bool) {
        match self {
            TriggerConfig::None => (false, false),
            TriggerConfig::Simple { .. } => (true, false),
            TriggerConfig::Advanced {
                conditions,
                pulse_width,
                ..
            } => (!conditions.is_empty(), pulse_width.is_some()),
        }
    }

    /// The digital inputs that the trigger and its pulse width qualifier use
    ///
    /// - Bits 0 to 7 are on port 0, and bits 8 to 15 are on port 1.
    pub fn digital_bits(&self) -> Vec<DigitalBit> {
        match self {
            TriggerConfig::Advanced {
                digital,
                pulse_width,
                ..
            } => digital
                .iter()
                .chain(pulse_width.iter().flat_map(|p| &p.digital))
                .copied()
                .collect(),
            _ => Vec::new(),
        }
    }

    /// Each kind of term that the config uses, once
    pub fn kinds(&self) -> Vec<TriggerKind> {
        let mut kinds = Vec::new();
        match self {
            TriggerConfig::None => {}
            TriggerConfig::Simple { source, delay, .. } => {
                kinds.push(TriggerKind::Simple);
                kinds.extend(source_kind(*source));
                if *delay > 0 {
                    kinds.push(TriggerKind::Delay);
                }
            }
            TriggerConfig::Advanced {
                channels,
                conditions,
                digital,
                pulse_width,
                delay,
                holdoff,
                ..
            } => {
                kinds.push(TriggerKind::Advanced);
                for channel in channels {
                    kinds.extend(source_kind(channel.source));
                    if channel.direction.is_runt() {
                        kinds.push(TriggerKind::Runt);
                    } else if channel.direction.mode() == ThresholdMode::Window {
                        kinds.push(TriggerKind::Window);
                    }
                }
                let pwq_conditions = pulse_width.iter().flat_map(|p| &p.conditions);
                for (source, _) in conditions.iter().chain(pwq_conditions).flatten() {
                    kinds.extend(source_kind(*source));
                }
                let pwq_digital = pulse_width.iter().any(|p| !p.digital.is_empty());
                if !digital.is_empty() || pwq_digital {
                    kinds.push(TriggerKind::Digital);
                }
                if pulse_width.is_some() {
                    kinds.push(TriggerKind::PulseWidth);
                }
                if *delay > 0 {
                    kinds.push(TriggerKind::Delay);
                }
                if holdoff.is_some() {
                    kinds.push(TriggerKind::Holdoff);
                }
            }
        }
        let mut seen = Vec::new();
        kinds.retain(|k| {
            let new = !seen.contains(k);
            seen.push(*k);
            new
        });
        kinds
    }

    /// Checks the config against the capabilities, before any driver call
    ///
    /// - Returns Err when the capabilities do not have a kind of term that the config uses.
    /// - Returns Err when the parts of the config do not agree, such as a condition on a source with no trigger.
    /// - The status of the error tells which check failed, such as [`PicoStatus::INVALID_TRIGGER_CHANNEL`].
    pub fn check(&self, caps: &TriggerCapabilities) -> PicoResult<()> {
        for kind in self.kinds() {
            if !caps.has(kind) {
                let status = match kind {
                    TriggerKind::External => PicoStatus::EXTERNAL_TRIGGER_NOT_SUPPORTED,
                    TriggerKind::Aux => PicoStatus::AUX_NOT_SUPPORTED,
                    _ => PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE,
                };
                return Err(refuse(status, kind.name()));
            }
        }
        match self {
            TriggerConfig::None => Ok(()),
            TriggerConfig::Simple {
                source, direction, ..
            } => {
                if !matches!(
                    source,
                    TriggerSource::Channel(_) | TriggerSource::External | TriggerSource::Aux
                ) {
                    return Err(refuse(
                        PicoStatus::INVALID_TRIGGER_CHANNEL,
                        "simple trigger",
                    ));
                }
                if direction.mode() != ThresholdMode::Level {
                    return Err(refuse(
                        PicoStatus::INVALID_SIMPLE_TRIGGER_DIRECTION,
                        "simple trigger",
                    ));
                }
                Ok(())
            }
            TriggerConfig::Advanced {
                channels,
                conditions,
                digital,
                pulse_width,
                ..
            } => {
                let mut sources = Vec::new();
                for channel in channels {
                    if matches!(
                        channel.source,
                        TriggerSource::PulseWidthQualifier | TriggerSource::Digital
                    ) {
                        return Err(refuse(
                            PicoStatus::INVALID_TRIGGER_CHANNEL,
                            "trigger channels",
                        ));
                    }
                    if sources.contains(&channel.source) {
                        return Err(refuse(
                            PicoStatus::DUPLICATE_CONDITION_SOURCE,
                            "trigger channels",
                        ));
                    }
                    sources.push(channel.source);
                    let window = channel.direction.mode() == ThresholdMode::Window;
                    if window && channel.lower.is_none() {
                        return Err(refuse(
                            PicoStatus::INVALID_PARAMETER,
                            "a window trigger needs a lower threshold",
                        ));
                    }
                }
                let has = |source: TriggerSource| match source {
                    TriggerSource::PulseWidthQualifier => pulse_width.is_some(),
                    TriggerSource::Digital => !digital.is_empty(),
                    other => sources.contains(&other),
                };
                for term in conditions {
                    if term.is_empty() {
                        return Err(refuse(PicoStatus::CONDITIONS, "an empty trigger condition"));
                    }
                    for (source, state) in term {
                        if *state != TriggerState::DontCare && !has(*source) {
                            return Err(refuse(
                                PicoStatus::CONDITION_HAS_NO_TRIGGER_PROPERTY,
                                "trigger conditions",
                            ));
                        }
                    }
                }
                if conditions.is_empty() && (!channels.is_empty() || pulse_width.is_some()) {
                    return Err(refuse(
                        PicoStatus::NO_TRIGGER_CONDITIONS_SET,
                        "trigger conditions",
                    ));
                }
                let pwq_digital = pulse_width.iter().flat_map(|p| &p.digital);
                for bit in digital.iter().chain(pwq_digital) {
                    if bit.bit > 15 {
                        return Err(refuse(
                            PicoStatus::INVALID_DIGITAL_CHANNEL,
                            "digital trigger",
                        ));
                    }
                }
                if let Some(pwq) = pulse_width {
                    for (source, state) in pwq.conditions.iter().flatten() {
                        let ok = match source {
                            TriggerSource::PulseWidthQualifier => false,
                            TriggerSource::Digital => !pwq.digital.is_empty(),
                            other => sources.contains(other),
                        };
                        if *state != TriggerState::DontCare && !ok {
                            return Err(refuse(
                                PicoStatus::CONDITION_HAS_NO_TRIGGER_PROPERTY,
                                "pulse width conditions",
                            ));
                        }
                    }
                    let range = matches!(
                        pwq.kind,
                        PulseWidthType::InRange | PulseWidthType::OutOfRange
                    );
                    if range && pwq.upper < pwq.lower {
                        return Err(refuse(
                            PicoStatus::PULSE_WIDTH_QUALIFIER,
                            "pulse width range",
                        ));
                    }
                }
                Ok(())
            }
        }
    }

    /// The auto trigger time, or `None` when there is none
    pub fn auto_trigger(&self) -> Option<Duration> {
        match self {
            TriggerConfig::None => None,
            TriggerConfig::Simple { auto_trigger, .. }
            | TriggerConfig::Advanced { auto_trigger, .. } => *auto_trigger,
        }
    }

    /// The trigger delay, in sample intervals
    pub fn delay(&self) -> u64 {
        match self {
            TriggerConfig::None => 0,
            TriggerConfig::Simple { delay, .. } | TriggerConfig::Advanced { delay, .. } => *delay,
        }
    }
}

/// An auto trigger time in whole units of `unit`
///
/// - It rounds up, so that a short time does not become 0, which waits with no end.
/// - Returns `Ok(0)` for `None`.
/// - Returns Err with [`PicoStatus::INVALID_PARAMETER`] when the count is more than `max`.
/// - Panics when `unit` is zero.
pub fn auto_trigger_units(auto: Option<Duration>, unit: Duration, max: u64) -> PicoResult<u64> {
    let Some(auto) = auto else {
        return Ok(0);
    };
    let units = auto.as_nanos().div_ceil(unit.as_nanos());
    u64::try_from(units)
        .ok()
        .filter(|u| *u <= max)
        .ok_or_else(|| refuse(PicoStatus::INVALID_PARAMETER, "auto-trigger time"))
}

/// Groups `bits` by port, in port order
pub fn digital_by_port(bits: &[DigitalBit]) -> Vec<(u8, Vec<DigitalBit>)> {
    let mut ports: Vec<(u8, Vec<DigitalBit>)> = Vec::new();
    for bit in bits {
        match ports.iter_mut().find(|(p, _)| *p == bit.port()) {
            Some((_, list)) => list.push(*bit),
            None => ports.push((bit.port(), vec![*bit])),
        }
    }
    ports.sort_by_key(|(p, _)| *p);
    ports
}

#[cfg(test)]
mod tests {
    use super::*;

    fn every() -> TriggerCapabilities {
        TriggerCapabilities {
            simple: true,
            advanced: true,
            pulse_width: true,
            digital: true,
            delay: true,
            holdoff: true,
            window: true,
            runt: true,
            external: true,
            aux: true,
            ..Default::default()
        }
    }

    fn a(direction: ThresholdDirection) -> ChannelTrigger {
        ChannelTrigger::level(TriggerSource::Channel(PicoChannel::A), direction, 1000, 50)
    }

    #[test]
    fn a_family_without_a_kind_refuses_it_by_name() {
        let config = TriggerConfig::Advanced {
            channels: vec![a(ThresholdDirection::Rising)],
            conditions: vec![vec![(
                TriggerSource::Channel(PicoChannel::A),
                TriggerState::True,
            )]],
            digital: Vec::new(),
            pulse_width: None,
            delay: 0,
            holdoff: Some(100),
            auto_trigger: None,
        };
        let caps = TriggerCapabilities {
            holdoff: false,
            ..every()
        };
        let error = config.check(&caps).unwrap_err();
        assert!(error.is_not_supported());
        assert!(error.to_string().contains("holdoff"));
        assert!(config.check(&every()).is_ok());

        let ext = TriggerConfig::Simple {
            source: TriggerSource::External,
            threshold_adc: 0,
            direction: ThresholdDirection::Rising,
            delay: 0,
            auto_trigger: None,
        };
        let caps = TriggerCapabilities {
            external: false,
            ..every()
        };
        assert_eq!(
            ext.check(&caps).unwrap_err().status,
            PicoStatus::EXTERNAL_TRIGGER_NOT_SUPPORTED
        );
    }

    #[test]
    fn kinds_follow_the_terms() {
        let window = ChannelTrigger::window(
            TriggerSource::Channel(PicoChannel::B),
            ThresholdDirection::Enter,
            Threshold::new(-100, 10),
            Threshold::new(100, 10),
        );
        let runt = ChannelTrigger::window(
            TriggerSource::Aux,
            ThresholdDirection::PositiveRunt,
            Threshold::new(-100, 10),
            Threshold::new(100, 10),
        );
        let config = TriggerConfig::any_of(vec![window, runt]);
        assert_eq!(
            config.kinds(),
            vec![
                TriggerKind::Advanced,
                TriggerKind::Window,
                TriggerKind::Aux,
                TriggerKind::Runt
            ]
        );
        let caps = TriggerCapabilities {
            runt: false,
            ..every()
        };
        assert!(config.check(&caps).unwrap_err().is_not_supported());
    }

    #[test]
    fn configs_that_do_not_hold_together_are_refused() {
        let simple_window = TriggerConfig::Simple {
            source: TriggerSource::Channel(PicoChannel::A),
            threshold_adc: 0,
            direction: ThresholdDirection::Inside,
            delay: 0,
            auto_trigger: None,
        };
        assert_eq!(
            simple_window.check(&every()).unwrap_err().status,
            PicoStatus::INVALID_SIMPLE_TRIGGER_DIRECTION
        );

        let mut no_lower = a(ThresholdDirection::Inside);
        no_lower.lower = None;
        assert_eq!(
            TriggerConfig::any_of(vec![no_lower])
                .check(&every())
                .unwrap_err()
                .status,
            PicoStatus::INVALID_PARAMETER
        );

        let orphan = TriggerConfig::Advanced {
            channels: vec![a(ThresholdDirection::Rising)],
            conditions: vec![vec![(
                TriggerSource::Channel(PicoChannel::B),
                TriggerState::True,
            )]],
            digital: Vec::new(),
            pulse_width: None,
            delay: 0,
            holdoff: None,
            auto_trigger: None,
        };
        assert_eq!(
            orphan.check(&every()).unwrap_err().status,
            PicoStatus::CONDITION_HAS_NO_TRIGGER_PROPERTY
        );

        let twice = TriggerConfig::any_of(vec![
            a(ThresholdDirection::Rising),
            a(ThresholdDirection::Falling),
        ]);
        assert_eq!(
            twice.check(&every()).unwrap_err().status,
            PicoStatus::DUPLICATE_CONDITION_SOURCE
        );
    }

    #[test]
    fn a_pulse_width_trigger_on_digital_bits_checks() {
        let config = TriggerConfig::Advanced {
            channels: Vec::new(),
            conditions: vec![vec![(
                TriggerSource::PulseWidthQualifier,
                TriggerState::True,
            )]],
            digital: Vec::new(),
            pulse_width: Some(PulseWidthQualifier {
                conditions: vec![vec![(TriggerSource::Digital, TriggerState::True)]],
                direction: ThresholdDirection::Rising,
                lower: 100,
                upper: 0,
                kind: PulseWidthType::GreaterThan,
                digital: vec![DigitalBit::new(9, DigitalDirection::High)],
            }),
            delay: 0,
            holdoff: None,
            auto_trigger: None,
        };
        assert!(config.check(&every()).is_ok());
        assert_eq!(config.enables(), (true, true));
        assert!(config
            .check(&TriggerCapabilities {
                digital: false,
                ..every()
            })
            .unwrap_err()
            .is_not_supported());
    }

    #[test]
    fn auto_trigger_rounds_up_and_has_a_limit() {
        let ms = Duration::from_millis(1);
        assert_eq!(auto_trigger_units(None, ms, 10).unwrap(), 0);
        assert_eq!(
            auto_trigger_units(Some(Duration::from_micros(10)), ms, 10).unwrap(),
            1
        );
        assert_eq!(
            auto_trigger_units(Some(Duration::from_millis(10)), ms, 10).unwrap(),
            10
        );
        assert!(auto_trigger_units(Some(Duration::from_millis(11)), ms, 10).is_err());
    }

    #[test]
    fn digital_bits_split_by_port() {
        let bits = [
            DigitalBit::new(9, DigitalDirection::High),
            DigitalBit::new(0, DigitalDirection::Rising),
            DigitalBit::new(3, DigitalDirection::Low),
        ];
        let ports = digital_by_port(&bits);
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].0, 0);
        assert_eq!(ports[0].1.len(), 2);
        assert_eq!(ports[1], (1, vec![bits[0]]));
    }

    #[test]
    fn window_directions_take_window_mode() {
        assert_eq!(ThresholdDirection::Rising.mode(), ThresholdMode::Level);
        assert_eq!(ThresholdDirection::Exit.mode(), ThresholdMode::Window);
        assert_eq!(ThresholdDirection::NegativeRunt.code(), 10);
        assert_eq!(ThresholdDirection::Exit.code(), 3);
    }
}
