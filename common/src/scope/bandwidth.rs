/// A hardware bandwidth limit on a channel, by its cut-off frequency.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BandwidthLimit {
    pub hz: u32,
}

impl BandwidthLimit {
    pub const HZ_20K: BandwidthLimit = BandwidthLimit::from_hz(20_000);
    pub const HZ_100K: BandwidthLimit = BandwidthLimit::from_hz(100_000);
    pub const HZ_200K: BandwidthLimit = BandwidthLimit::from_hz(200_000);
    pub const HZ_1M: BandwidthLimit = BandwidthLimit::from_hz(1_000_000);
    pub const HZ_20M: BandwidthLimit = BandwidthLimit::from_hz(20_000_000);
    pub const HZ_25M: BandwidthLimit = BandwidthLimit::from_hz(25_000_000);
    pub const HZ_50M: BandwidthLimit = BandwidthLimit::from_hz(50_000_000);
    pub const HZ_60M: BandwidthLimit = BandwidthLimit::from_hz(60_000_000);
    pub const HZ_100M: BandwidthLimit = BandwidthLimit::from_hz(100_000_000);
    pub const HZ_200M: BandwidthLimit = BandwidthLimit::from_hz(200_000_000);
    pub const HZ_350M: BandwidthLimit = BandwidthLimit::from_hz(350_000_000);
    pub const HZ_500M: BandwidthLimit = BandwidthLimit::from_hz(500_000_000);

    pub const fn from_hz(hz: u32) -> BandwidthLimit {
        BandwidthLimit { hz }
    }
}

impl std::fmt::Display for BandwidthLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.hz {
            hz if hz >= 1_000_000 && hz % 1_000_000 == 0 => write!(f, "{} MHz", hz / 1_000_000),
            hz if hz >= 1_000 && hz % 1_000 == 0 => write!(f, "{} kHz", hz / 1_000),
            hz => write!(f, "{hz} Hz"),
        }
    }
}
