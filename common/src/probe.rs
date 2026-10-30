use crate::{PicoChannel, PicoCoupling, PicoRange, PicoStatus};
use std::fmt;

/// A probe a scope detects on a channel: Pico's `PICO_CONNECT_PROBE`
/// values (PicoConnectProbes.h), for PicoConnect probes on D9 inputs
/// and BNC+ probes. A value this crate does not name is `Other`.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PicoProbe {
    None,
    D9Bnc,
    D9TwoBnc,
    Differential,
    CurrentClamp200A2kA,
    CurrentClamp40A,
    Cat3Hv1kV,
    CurrentClamp2000ARms,
    TestLeadBlue,
    TestLeadRed,
    TestLeadGreen,
    TestLeadYellow,
    CopProbe,
    Temperature,
    CurrentClamp100A,
    HtPickup,
    X10ScopeProbe,
    CurrentClamp2000A,
    PressureSensor,
    ResistanceLead,
    CurrentClamp60A,
    OpticalSensor,
    CurrentClamp60AV2,
    HighVoltageDifferential,
    TriAxialAccelerometer,
    Microphone,
    PassiveX10,
    ActiveX10_750MHz,
    ActiveX10_1_3GHz,
    PassiveX10_50Ohm,
    /// The driver found an intelligent probe it has not identified yet.
    Intelligent,
    /// The driver could not identify the probe.
    Unknown,
    /// The probe reported a fault.
    Fault,
    /// A value this crate does not name.
    Other(i32),
}

/// Each named probe with its `PICO_CONNECT_PROBE` value.
const CODES: &[(PicoProbe, i32)] = &[
    (PicoProbe::None, 0),
    (PicoProbe::D9Bnc, 4000),
    (PicoProbe::D9TwoBnc, 4001),
    (PicoProbe::Differential, 4002),
    (PicoProbe::CurrentClamp200A2kA, 4003),
    (PicoProbe::CurrentClamp40A, 4004),
    (PicoProbe::Cat3Hv1kV, 4005),
    (PicoProbe::CurrentClamp2000ARms, 4006),
    (PicoProbe::TestLeadBlue, 4404),
    (PicoProbe::TestLeadRed, 4405),
    (PicoProbe::TestLeadGreen, 4406),
    (PicoProbe::TestLeadYellow, 4407),
    (PicoProbe::CopProbe, 4408),
    (PicoProbe::Temperature, 5000),
    (PicoProbe::CurrentClamp100A, 5003),
    (PicoProbe::HtPickup, 5005),
    (PicoProbe::X10ScopeProbe, 5006),
    (PicoProbe::CurrentClamp2000A, 5007),
    (PicoProbe::PressureSensor, 5008),
    (PicoProbe::ResistanceLead, 5009),
    (PicoProbe::CurrentClamp60A, 5010),
    (PicoProbe::OpticalSensor, 5011),
    (PicoProbe::CurrentClamp60AV2, 5012),
    (PicoProbe::HighVoltageDifferential, 5013),
    (PicoProbe::TriAxialAccelerometer, 5014),
    (PicoProbe::Microphone, 5015),
    (PicoProbe::PassiveX10, 6000),
    (PicoProbe::ActiveX10_750MHz, 6001),
    (PicoProbe::ActiveX10_1_3GHz, 6002),
    (PicoProbe::PassiveX10_50Ohm, 6003),
    (PicoProbe::Intelligent, -3),
    (PicoProbe::Unknown, -2),
    (PicoProbe::Fault, -1),
];

impl PicoProbe {
    /// The probe of a `PICO_CONNECT_PROBE` value.
    pub fn from_code(code: i32) -> Self {
        CODES
            .iter()
            .find(|(_, c)| *c == code)
            .map_or(PicoProbe::Other(code), |(p, _)| *p)
    }

    /// The `PICO_CONNECT_PROBE` value.
    pub fn code(self) -> i32 {
        match self {
            PicoProbe::Other(code) => code,
            probe => CODES
                .iter()
                .find(|(p, _)| *p == probe)
                .map(|(_, c)| *c)
                .expect("every named probe has a code"),
        }
    }

    /// A short stable name: `current_clamp_60a`, or `probe_5099` for a
    /// value this crate does not name.
    pub fn tag(self) -> String {
        let name = match self {
            PicoProbe::None => "none",
            PicoProbe::D9Bnc => "d9_bnc",
            PicoProbe::D9TwoBnc => "d9_2x_bnc",
            PicoProbe::Differential => "differential",
            PicoProbe::CurrentClamp200A2kA => "current_clamp_200a_2ka",
            PicoProbe::CurrentClamp40A => "current_clamp_40a",
            PicoProbe::Cat3Hv1kV => "cat3_hv_1kv",
            PicoProbe::CurrentClamp2000ARms => "current_clamp_2000arms",
            PicoProbe::TestLeadBlue => "test_lead_blue",
            PicoProbe::TestLeadRed => "test_lead_red",
            PicoProbe::TestLeadGreen => "test_lead_green",
            PicoProbe::TestLeadYellow => "test_lead_yellow",
            PicoProbe::CopProbe => "cop_probe",
            PicoProbe::Temperature => "temperature",
            PicoProbe::CurrentClamp100A => "current_clamp_100a",
            PicoProbe::HtPickup => "ht_pickup",
            PicoProbe::X10ScopeProbe => "x10_scope_probe",
            PicoProbe::CurrentClamp2000A => "current_clamp_2000a",
            PicoProbe::PressureSensor => "pressure_sensor",
            PicoProbe::ResistanceLead => "resistance_lead",
            PicoProbe::CurrentClamp60A => "current_clamp_60a",
            PicoProbe::OpticalSensor => "optical_sensor",
            PicoProbe::CurrentClamp60AV2 => "current_clamp_60a_v2",
            PicoProbe::HighVoltageDifferential => "high_voltage_differential",
            PicoProbe::TriAxialAccelerometer => "tri_axial_accelerometer",
            PicoProbe::Microphone => "microphone",
            PicoProbe::PassiveX10 => "passive_x10",
            PicoProbe::ActiveX10_750MHz => "active_x10_750mhz",
            PicoProbe::ActiveX10_1_3GHz => "active_x10_1_3ghz",
            PicoProbe::PassiveX10_50Ohm => "passive_x10_50ohm",
            PicoProbe::Intelligent => "intelligent",
            PicoProbe::Unknown => "unknown",
            PicoProbe::Fault => "fault",
            PicoProbe::Other(code) => return format!("probe_{code}"),
        };
        name.to_string()
    }
}

impl fmt::Display for PicoProbe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PicoProbe::None => "No probe",
            PicoProbe::D9Bnc => "D9-BNC adaptor",
            PicoProbe::D9TwoBnc => "D9-2×BNC adaptor",
            PicoProbe::Differential => "Differential probe",
            PicoProbe::CurrentClamp200A2kA => "200 A/2 kA current clamp",
            PicoProbe::CurrentClamp40A => "40 A current clamp",
            PicoProbe::Cat3Hv1kV => "1 kV CAT III probe",
            PicoProbe::CurrentClamp2000ARms => "2000 A RMS current clamp",
            PicoProbe::TestLeadBlue => "Test lead (blue)",
            PicoProbe::TestLeadRed => "Test lead (red)",
            PicoProbe::TestLeadGreen => "Test lead (green)",
            PicoProbe::TestLeadYellow => "Test lead (yellow)",
            PicoProbe::CopProbe => "COP probe",
            PicoProbe::Temperature => "Temperature probe",
            PicoProbe::CurrentClamp100A => "100 A current clamp",
            PicoProbe::HtPickup => "HT pickup",
            PicoProbe::X10ScopeProbe => "×10 scope probe",
            PicoProbe::CurrentClamp2000A => "2000 A current clamp",
            PicoProbe::PressureSensor => "Pressure sensor",
            PicoProbe::ResistanceLead => "Resistance lead",
            PicoProbe::CurrentClamp60A => "60 A current clamp",
            PicoProbe::OpticalSensor => "Optical sensor",
            PicoProbe::CurrentClamp60AV2 => "60 A current clamp (v2)",
            PicoProbe::HighVoltageDifferential => "High-voltage differential probe",
            PicoProbe::TriAxialAccelerometer => "Tri-axial accelerometer",
            PicoProbe::Microphone => "Microphone",
            PicoProbe::PassiveX10 => "×10 passive probe",
            PicoProbe::ActiveX10_750MHz => "×10 active probe 750 MHz",
            PicoProbe::ActiveX10_1_3GHz => "×10 active probe 1.3 GHz",
            PicoProbe::PassiveX10_50Ohm => "×10 passive probe 50 Ω",
            PicoProbe::Intelligent => "Intelligent probe",
            PicoProbe::Unknown => "Unknown probe",
            PicoProbe::Fault => "Faulty probe",
            PicoProbe::Other(code) => return write!(f, "Probe {code}"),
        };
        f.write_str(name)
    }
}

/// What a scope reports of the probe on one channel.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeState {
    pub channel: PicoChannel,
    /// [`PicoProbe::None`] while no probe is connected, as the driver
    /// goes on naming the last probe after it is unplugged.
    pub probe: PicoProbe,
    /// False once the probe is unplugged. The driver keeps its settings
    /// and sets them again when it comes back.
    pub connected: bool,
    pub enabled: bool,
    pub requires_power: bool,
    pub powered: bool,
    /// The driver's status for this probe; `None` for a value this
    /// crate does not name.
    pub status: Option<PicoStatus>,
    /// The ranges the probe offers, in the driver's order, each with
    /// the unit it reads in ([`PicoRange::get_units`]). Empty while no
    /// probe is connected.
    pub ranges: Vec<PicoRange>,
    /// Range values the probe offers that this crate does not name.
    pub unknown_ranges: Vec<i32>,
    /// The range the probe is set to, where this crate names it.
    pub range: Option<PicoRange>,
    pub couplings: Vec<PicoCoupling>,
    pub coupling: Option<PicoCoupling>,
}

impl ProbeState {
    /// Whether `other` reports the same probe in the same state: the
    /// probe, connected, power, status, ranges and couplings. The range
    /// and coupling set, and whether the channel is on, are left out:
    /// they follow the application's own channel calls.
    pub fn same_probe(&self, other: &ProbeState) -> bool {
        self.channel == other.channel
            && self.probe == other.probe
            && self.connected == other.connected
            && self.requires_power == other.requires_power
            && self.powered == other.powered
            && self.status == other.status
            && self.ranges == other.ranges
            && self.unknown_ranges == other.unknown_ranges
            && self.couplings == other.couplings
    }

    /// The ranges of `first..=last` this crate names, and the values of
    /// those it does not.
    pub fn split_ranges(first: i32, last: i32) -> (Vec<PicoRange>, Vec<i32>) {
        let mut known = Vec::new();
        let mut unknown = Vec::new();
        if last < first || last - first > MAX_RANGES {
            return (known, unknown);
        }
        for code in first..=last {
            match PicoRange::from_code(code) {
                Some(range) => known.push(range),
                None => unknown.push(code),
            }
        }
        (known, unknown)
    }

    /// The ranges of a connected probe this crate can scale: those with
    /// a known symmetric full scale ([`PicoRange::full_scale`]).
    pub fn scaled_ranges(&self) -> Vec<PicoRange> {
        if !self.connected {
            return Vec::new();
        }
        self.ranges
            .iter()
            .copied()
            .filter(|r| r.full_scale().is_some())
            .collect()
    }

    /// Why a channel with this probe cannot run, if it cannot: a fault,
    /// a probe with no power, or one with no range this crate scales.
    pub fn problem(&self) -> Option<String> {
        if !self.connected {
            return None;
        }
        if self.probe == PicoProbe::Fault {
            return Some(format!("the probe on {} reports a fault", self.channel));
        }
        if self.requires_power && !self.powered {
            return Some(format!(
                "the {} on {} needs power the scope does not give it",
                self.probe, self.channel
            ));
        }
        if self.scaled_ranges().is_empty() {
            return Some(format!(
                "the {} on {} has no range with a known scale",
                self.probe, self.channel
            ));
        }
        None
    }
}

/// The most range values one probe lists; a wider span is a bad read.
const MAX_RANGES: i32 = 64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_round_trip() {
        for (probe, code) in CODES {
            assert_eq!(PicoProbe::from_code(*code), *probe);
            assert_eq!(probe.code(), *code);
        }
        assert_eq!(PicoProbe::from_code(5099), PicoProbe::Other(5099));
        assert_eq!(PicoProbe::Other(5099).code(), 5099);
        assert_eq!(PicoProbe::Other(5099).tag(), "probe_5099");
        assert_eq!(PicoProbe::CurrentClamp60A.tag(), "current_clamp_60a");
        assert_eq!(PicoProbe::CurrentClamp60A.to_string(), "60 A current clamp");
    }

    #[test]
    fn tags_are_unique() {
        let mut tags: Vec<String> = CODES.iter().map(|(p, _)| p.tag()).collect();
        tags.sort();
        let n = tags.len();
        tags.dedup();
        assert_eq!(tags.len(), n);
    }

    #[test]
    fn ranges_split_into_known_and_unknown() {
        let (known, unknown) = ProbeState::split_ranges(10500, 10505);
        assert_eq!(known.first(), Some(&PicoRange::CURRENT_CLAMP_60A_2A));
        assert_eq!(known.len(), 6);
        assert!(unknown.is_empty());
        let (known, unknown) = ProbeState::split_ranges(10750, 10752);
        assert!(known.is_empty());
        assert_eq!(unknown, vec![10750, 10751, 10752]);
        assert_eq!(ProbeState::split_ranges(5, 4), (Vec::new(), Vec::new()));
        assert_eq!(ProbeState::split_ranges(0, 100_000).0.len(), 0);
    }

    fn clamp(connected: bool) -> ProbeState {
        let (ranges, unknown_ranges) = ProbeState::split_ranges(10500, 10505);
        ProbeState {
            channel: PicoChannel::A,
            probe: PicoProbe::CurrentClamp60A,
            connected,
            enabled: true,
            requires_power: false,
            powered: false,
            status: Some(PicoStatus::OK),
            ranges,
            unknown_ranges,
            range: Some(PicoRange::CURRENT_CLAMP_60A_60A),
            couplings: vec![PicoCoupling::DC],
            coupling: Some(PicoCoupling::DC),
        }
    }

    #[test]
    fn the_same_probe_leaves_out_what_the_application_sets() {
        let a = clamp(true);
        let mut set = a.clone();
        set.enabled = false;
        set.range = Some(PicoRange::CURRENT_CLAMP_60A_2A);
        set.coupling = None;
        assert!(a.same_probe(&set));
        let mut removed = a.clone();
        removed.connected = false;
        assert!(!a.same_probe(&removed));
        let mut unpowered = a.clone();
        unpowered.powered = true;
        assert!(!a.same_probe(&unpowered));
        let mut other = a.clone();
        other.probe = PicoProbe::CurrentClamp60AV2;
        assert!(!a.same_probe(&other));
        let mut fewer = a.clone();
        fewer.ranges.pop();
        assert!(!a.same_probe(&fewer));
        let mut faulted = a.clone();
        faulted.status = None;
        assert!(!a.same_probe(&faulted));
    }

    #[test]
    fn problems_of_a_probe() {
        assert_eq!(clamp(true).problem(), None);
        assert_eq!(clamp(false).problem(), None);
        assert!(clamp(false).scaled_ranges().is_empty());
        let mut unpowered = clamp(true);
        unpowered.requires_power = true;
        assert!(unpowered.problem().unwrap().contains("needs power"));
        let mut unscaled = clamp(true);
        unscaled.ranges = vec![PicoRange::TEMPERATURE_NEG50_TO_150DEGC];
        assert!(unscaled.problem().unwrap().contains("no range"));
    }
}
