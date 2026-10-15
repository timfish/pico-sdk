use super::Overflow;
use crate::PicoChannel;
use num_traits::FromPrimitive;
use std::ops::{BitOr, BitOrAssign};

impl Overflow {
    /// The bit field a family's callback or `GetValues` gives: bit 0 for
    /// channel A. Every family's guide gives this layout.
    pub fn from_bits(bits: i16) -> Self {
        Overflow(bits as u16)
    }

    /// Whether `channel` went over range.
    pub fn channel(&self, channel: PicoChannel) -> bool {
        self.0 & (1 << channel as u16) != 0
    }

    /// This with `channel` marked over range.
    pub fn with(self, channel: PicoChannel) -> Self {
        Overflow(self.0 | 1 << channel as u16)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// The channels marked over range, A first.
    pub fn channels(&self) -> impl Iterator<Item = PicoChannel> + '_ {
        (0..16u8)
            .filter(|bit| self.0 & (1 << bit) != 0)
            .filter_map(PicoChannel::from_u8)
    }
}

impl BitOr for Overflow {
    type Output = Overflow;

    fn bitor(self, other: Overflow) -> Overflow {
        Overflow(self.0 | other.0)
    }
}

impl BitOrAssign for Overflow {
    fn bitor_assign(&mut self, other: Overflow) {
        self.0 |= other.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_0_is_channel_a() {
        let overflow = Overflow::from_bits(0b1010);
        assert!(!overflow.channel(PicoChannel::A));
        assert!(overflow.channel(PicoChannel::B));
        assert!(overflow.channel(PicoChannel::D));
        assert_eq!(
            overflow.channels().collect::<Vec<_>>(),
            [PicoChannel::B, PicoChannel::D]
        );
        assert_eq!(
            Overflow::from_bits(-128).channels().next(),
            Some(PicoChannel::H)
        );
    }

    #[test]
    fn flags_join() {
        let mut overflow = Overflow::default().with(PicoChannel::C);
        assert!(!overflow.is_empty());
        overflow |= Overflow::from_bits(1);
        assert_eq!(overflow, Overflow(0b101));
        assert_eq!(Overflow(1) | Overflow(2), Overflow(3));
    }
}
