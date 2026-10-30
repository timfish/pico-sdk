//! Probes the unit detects: PicoConnect probes on a 4444's D9 inputs
//! and BNC+ probes on the 4x25A. The driver reports them through the
//! callback set by `ps4000aSetProbeInteractionCallback`, with no
//! context pointer, so the states are kept here by handle.
//!
//! Sources: ps4000a API help, "ps4000aProbeInteractions" and
//! "PS4000A_USER_PROBE_INTERACTIONS"; Pico forum topic p=152524 (the
//! callback works on the 4x25A; a range outside first..=last is
//! refused; a channel with a probe is turned off with
//! `PICO_CONNECT_PROBE_OFF`).
//!
//! On a 4425A each call also held channels whose probe had not
//! changed, and an unplugged probe kept its name, so a state counts as
//! a change only when [`ProbeState::same_probe`] says it differs.

use c_vec::CVec;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use pico_common::{PicoChannel, PicoCoupling, PicoProbe, PicoStatus, ProbeState};
use pico_sys_dynamic::ps4000a::PS4000A_USER_PROBE_INTERACTIONS;
use std::collections::HashMap;

/// The states the driver reported for one unit.
#[derive(Debug, Default)]
pub(crate) struct UnitProbes {
    pub states: HashMap<PicoChannel, ProbeState>,
    /// States not yet taken by `take_changes`, oldest first.
    pub changes: Vec<ProbeState>,
}

lazy_static! {
    static ref PROBES: Mutex<HashMap<i16, UnitProbes>> = Default::default();
}

/// The state of one element of the driver's array. `None` for a
/// channel this crate does not name. A probe that is not connected is
/// [`PicoProbe::None`] with no ranges.
pub(crate) fn state_of(p: &PS4000A_USER_PROBE_INTERACTIONS) -> Option<ProbeState> {
    // Copies, as the struct is packed.
    let channel = p.channel;
    let channel: PicoChannel = num_traits_channel(channel)?;
    let (first, last, current) = (p.rangeFirst_, p.rangeLast_, p.rangeCurrent_);
    let (ranges, unknown_ranges) = ProbeState::split_ranges(first as i32, last as i32);
    let (c_first, c_last, c_current) = (p.couplingFirst_, p.couplingLast_, p.couplingCurrent_);
    let couplings = if c_last >= c_first && c_last - c_first < 4 {
        (c_first..=c_last).filter_map(coupling).collect()
    } else {
        Vec::new()
    };
    let status = p.status_;
    let probe = p.probeName;
    let connected = p.connected != 0;
    let (probe, ranges, unknown_ranges) = if connected {
        (PicoProbe::from_code(probe), ranges, unknown_ranges)
    } else {
        (PicoProbe::None, Vec::new(), Vec::new())
    };
    Some(ProbeState {
        channel,
        probe,
        connected,
        enabled: p.enabled != 0,
        requires_power: p.requiresPower_ != 0,
        powered: p.isPowered_ != 0,
        status: PicoStatus::from_code(status),
        ranges,
        unknown_ranges,
        range: pico_common::PicoRange::from_code(current as i32),
        couplings,
        coupling: coupling(c_current),
    })
}

fn num_traits_channel(value: u32) -> Option<PicoChannel> {
    (value < 8).then(|| PicoChannel::from(value))
}

fn coupling(value: u32) -> Option<PicoCoupling> {
    match value {
        0 => Some(PicoCoupling::AC),
        1 => Some(PicoCoupling::DC),
        _ => None,
    }
}

impl UnitProbes {
    /// Takes one call's states: each replaces its channel's state, and
    /// is kept as a change when its probe differs from the one before.
    /// Gives the number of changes.
    pub fn update(&mut self, states: Vec<ProbeState>) -> usize {
        let mut changed = 0;
        for state in states {
            let same = self
                .states
                .get(&state.channel)
                .is_some_and(|before| before.same_probe(&state));
            self.states.insert(state.channel, state.clone());
            if !same {
                self.changes.push(state);
                changed += 1;
            }
        }
        changed
    }

    /// The connected probe on `channel`, if any.
    pub fn connected(&self, channel: PicoChannel) -> Option<&ProbeState> {
        self.states.get(&channel).filter(|s| s.connected)
    }
}

/// The driver's probe callback. A panic must not cross into C, so the
/// body runs under `catch_unwind`.
pub(crate) extern "C" fn callback(
    handle: i16,
    status: u32,
    probes: *mut PS4000A_USER_PROBE_INTERACTIONS,
    count: u32,
) {
    let result = std::panic::catch_unwind(|| {
        if probes.is_null() || count == 0 {
            tracing::debug!(handle, status, "probe callback with no probes");
            return;
        }
        let raw = unsafe { CVec::new(probes, count as usize) };
        let states: Vec<ProbeState> = raw.iter().filter_map(state_of).collect();
        for s in &states {
            tracing::debug!(
                handle,
                channel = %s.channel,
                probe = %s.probe,
                code = s.probe.code(),
                connected = s.connected,
                enabled = s.enabled,
                powered = s.powered,
                requires_power = s.requires_power,
                status = ?s.status,
                ranges = s.ranges.len(),
                unknown_ranges = ?s.unknown_ranges,
                range = ?s.range,
                "probe reported"
            );
        }
        let reported = states.len();
        let changed = PROBES.lock().entry(handle).or_default().update(states);
        if changed > 0 {
            tracing::info!(handle, reported, changed, "probe states changed");
        } else {
            tracing::debug!(handle, reported, "probe states reported with no change");
        }
    });
    if result.is_err() {
        tracing::error!(handle, "probe callback panicked; its states are lost");
    }
}

/// The connected probe on `channel` of `handle`.
pub(crate) fn connected(handle: i16, channel: PicoChannel) -> Option<ProbeState> {
    PROBES
        .lock()
        .get(&handle)
        .and_then(|u| u.connected(channel).cloned())
}

/// Every channel's last state.
pub(crate) fn states(handle: i16) -> Vec<ProbeState> {
    let mut states: Vec<ProbeState> = PROBES
        .lock()
        .get(&handle)
        .map(|u| u.states.values().cloned().collect())
        .unwrap_or_default();
    states.sort_by_key(|s| s.channel);
    states
}

/// The states reported since the last call.
pub(crate) fn take_changes(handle: i16) -> Vec<ProbeState> {
    PROBES
        .lock()
        .get_mut(&handle)
        .map(|u| std::mem::take(&mut u.changes))
        .unwrap_or_default()
}

/// Forgets a closed unit.
pub(crate) fn forget(handle: i16) {
    PROBES.lock().remove(&handle);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::PicoRange;

    fn raw(channel: u32, probe: i32, connected: bool) -> PS4000A_USER_PROBE_INTERACTIONS {
        PS4000A_USER_PROBE_INTERACTIONS {
            connected: connected as u16,
            channel,
            enabled: 1,
            probeName: probe,
            requiresPower_: 0,
            isPowered_: 0,
            status_: 0,
            probeOff: 1024,
            rangeFirst_: 10500,
            rangeLast_: 10505,
            rangeCurrent_: 10505,
            couplingFirst_: 1,
            couplingLast_: 1,
            couplingCurrent_: 1,
            filterFlags_: 0,
            filterCurrent_: 0,
            defaultFilter_: 0,
        }
    }

    #[test]
    fn a_clamp_reads_with_every_range_inclusive() {
        let s = state_of(&raw(1, 5010, true)).unwrap();
        assert_eq!(s.channel, PicoChannel::B);
        assert_eq!(s.probe, PicoProbe::CurrentClamp60A);
        assert!(s.connected);
        assert_eq!(s.ranges.len(), 6, "first..=last");
        assert_eq!(s.ranges.last(), Some(&PicoRange::CURRENT_CLAMP_60A_60A));
        assert_eq!(s.range, Some(PicoRange::CURRENT_CLAMP_60A_60A));
        assert_eq!(s.couplings, vec![PicoCoupling::DC]);
        assert_eq!(s.status, Some(PicoStatus::OK));
    }

    #[test]
    fn unknown_values_do_not_panic() {
        let mut r = raw(1, 9999, true);
        r.rangeFirst_ = 10750;
        r.rangeLast_ = 10758;
        r.rangeCurrent_ = 10750;
        r.status_ = 0xFFFF_FFF0;
        r.couplingFirst_ = 7;
        let s = state_of(&r).unwrap();
        assert_eq!(s.probe, PicoProbe::Other(9999));
        assert!(s.ranges.is_empty());
        assert_eq!(s.unknown_ranges.len(), 9);
        assert_eq!(s.range, None);
        assert_eq!(s.status, None);
        assert!(s.couplings.is_empty());
        assert_eq!(state_of(&raw(40, 5010, true)), None);
    }

    #[test]
    fn updates_are_incremental_and_kept_as_changes() {
        let mut unit = UnitProbes::default();
        unit.update(vec![
            state_of(&raw(0, 4404, true)).unwrap(),
            state_of(&raw(1, 5010, true)).unwrap(),
        ]);
        unit.update(vec![state_of(&raw(1, 5010, false)).unwrap()]);
        assert!(unit.connected(PicoChannel::A).is_some());
        assert!(unit.connected(PicoChannel::B).is_none());
        assert_eq!(unit.states.len(), 2);
        assert_eq!(unit.changes.len(), 3);
    }

    /// As a 4425A did: one call for a probe pulled from A also held B
    /// and C, whose leads had not moved, and A's probe kept its name.
    #[test]
    fn only_real_changes_are_kept() {
        let mut unit = UnitProbes::default();
        let open = vec![
            state_of(&raw(0, 5013, true)).unwrap(),
            state_of(&raw(1, 4404, true)).unwrap(),
            state_of(&raw(2, 4407, true)).unwrap(),
        ];
        assert_eq!(unit.update(open), 3);
        unit.changes.clear();
        let mut lead_b = raw(1, 4404, true);
        // The application set B's range and turned C off meanwhile.
        lead_b.rangeCurrent_ = 10501;
        let mut lead_c = raw(2, 4407, true);
        lead_c.enabled = 0;
        let pulled = vec![
            state_of(&lead_b).unwrap(),
            state_of(&lead_c).unwrap(),
            state_of(&raw(0, 5013, false)).unwrap(),
        ];
        assert_eq!(unit.update(pulled), 1);
        assert_eq!(unit.changes.len(), 1);
        let a = &unit.changes[0];
        assert_eq!(a.channel, PicoChannel::A);
        assert_eq!(a.probe, PicoProbe::None);
        assert!(a.ranges.is_empty());
        assert_eq!(a.probe.to_string(), "No probe");
        // The states hold what was reported last.
        assert_eq!(
            unit.states[&PicoChannel::B].range,
            Some(PicoRange::CURRENT_CLAMP_60A_5A)
        );
        // Plugged in again, the probe is a change once.
        assert_eq!(unit.update(vec![state_of(&raw(0, 5013, true)).unwrap()]), 1);
        assert_eq!(unit.update(vec![state_of(&raw(0, 5013, true)).unwrap()]), 0);
    }

    #[test]
    fn the_callback_keeps_states_by_handle() {
        let mut probes = [raw(0, 5013, true), raw(3, 0, false)];
        callback(-77, 0, probes.as_mut_ptr(), 2);
        // A null array is ignored.
        callback(-77, 0, std::ptr::null_mut(), 3);
        assert_eq!(
            connected(-77, PicoChannel::A).map(|s| s.probe),
            Some(PicoProbe::HighVoltageDifferential)
        );
        assert_eq!(states(-77).len(), 2);
        assert_eq!(take_changes(-77).len(), 2);
        assert!(take_changes(-77).is_empty());
        forget(-77);
        assert!(states(-77).is_empty());
    }
}
