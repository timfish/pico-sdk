//! Model names from the variant string the driver reports, for the
//! capability tables of each family.

use pico_common::{BandwidthCase, BandwidthCondition, PicoRange, PicoResolution};

/// The model without its MSO marker: "2206B MSO", "3406DMSO" and
/// "5464E+MSO" give "2206B", "3406D" and "5464E".
pub(crate) fn model(variant: &str) -> &str {
    let variant = variant.trim();
    match variant.strip_suffix("MSO") {
        Some(rest) => rest.strip_suffix([' ', '+']).unwrap_or(rest),
        None => variant,
    }
}

pub(crate) fn is_mso(variant: &str) -> bool {
    variant.trim().ends_with("MSO")
}

pub(crate) const MHZ: u64 = 1_000_000;

pub(crate) fn at_resolution(resolution: PicoResolution, hz: u64) -> BandwidthCase {
    BandwidthCase {
        when: BandwidthCondition::Resolution(resolution),
        hz,
    }
}

pub(crate) fn at_range(most: PicoRange, hz: u64) -> BandwidthCase {
    BandwidthCase {
        when: BandwidthCondition::RangeUpTo(most),
        hz,
    }
}

pub(crate) fn at_high_impedance(hz: u64) -> BandwidthCase {
    BandwidthCase {
        when: BandwidthCondition::HighImpedance,
        hz,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_drops_the_mso_marker() {
        assert_eq!(model("2206B MSO"), "2206B");
        assert_eq!(model("5464E+MSO"), "5464E");
        assert_eq!(model("5464E++MSO"), "5464E+");
        assert_eq!(model("5464E+"), "5464E+");
        assert_eq!(model("3406DMSO"), "3406D");
        assert_eq!(model("2208A"), "2208A");
        assert!(is_mso("3406DMSO") && !is_mso("2208A"));
    }
}
