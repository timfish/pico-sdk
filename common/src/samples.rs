use crate::{PicoError, PicoResult, PicoStatus};

/// The integer type of the samples in a buffer or a block
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SampleType {
    /// `i8`
    I8,
    /// `i16`
    I16,
    /// `i32`
    I32,
}

impl SampleType {
    /// The number of bytes in one sample
    pub fn size(self) -> usize {
        match self {
            SampleType::I8 => 1,
            SampleType::I16 => 2,
            SampleType::I32 => 4,
        }
    }
}

/// How a downsample makes one value from each group of samples
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DownsampleKind {
    /// The mean of each group, as 16-bit values
    Average,
    /// The sum of each group, as 32-bit values
    ///
    /// - Only the ps6000a and psospa drivers have it.
    Sum,
    /// The lowest and the highest value of each group, in two buffers
    ///
    /// - Block reads only.
    Aggregate,
    /// The first sample of each group
    ///
    /// - Block reads only.
    Decimate,
}

/// A downsample: the driver makes one value from each `ratio` samples
///
/// - The rate is the rate of the device divided by `ratio`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Downsample {
    /// The number of samples in each group
    pub ratio: u32,
    /// How the driver makes one value from each group
    pub mode: DownsampleKind,
}

impl Downsample {
    /// Makes a [`DownsampleKind::Average`] downsample
    pub fn average(ratio: u32) -> Self {
        Downsample {
            ratio,
            mode: DownsampleKind::Average,
        }
    }

    /// Makes a [`DownsampleKind::Sum`] downsample
    pub fn sum(ratio: u32) -> Self {
        Downsample {
            ratio,
            mode: DownsampleKind::Sum,
        }
    }

    /// Makes a [`DownsampleKind::Aggregate`] downsample
    pub fn aggregate(ratio: u32) -> Self {
        Downsample {
            ratio,
            mode: DownsampleKind::Aggregate,
        }
    }

    /// Makes a [`DownsampleKind::Decimate`] downsample
    pub fn decimate(ratio: u32) -> Self {
        Downsample {
            ratio,
            mode: DownsampleKind::Decimate,
        }
    }

    /// The downsample to give the driver
    ///
    /// - Returns `Ok(None)` for no downsample, or for a ratio of 1.
    /// - Returns Err with [`PicoStatus::INVALID_SAMPLERATIO`] when the ratio is 0.
    ///
    /// ```
    /// use pico_common::Downsample;
    ///
    /// let none = Downsample::effective(Some(Downsample::average(1))).unwrap();
    /// assert_eq!(none, None);
    /// assert!(Downsample::effective(Some(Downsample::average(0))).is_err());
    /// ```
    pub fn effective(downsample: Option<Downsample>) -> PicoResult<Option<Downsample>> {
        match downsample {
            Some(Downsample { ratio: 0, .. }) => Err(PicoError::from_status(
                PicoStatus::INVALID_SAMPLERATIO,
                "downsample",
            )),
            Some(Downsample { ratio: 1, .. }) | None => Ok(None),
            some => Ok(some),
        }
    }

    /// The ratio of an optional downsample
    ///
    /// - Returns 1 for no downsample, and for a ratio of 0.
    pub fn ratio_of(downsample: Option<Downsample>) -> u32 {
        downsample.map_or(1, |d| d.ratio.max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ratio_of_one_is_no_downsample() {
        assert_eq!(Downsample::effective(None).unwrap(), None);
        assert_eq!(
            Downsample::effective(Some(Downsample::sum(1))).unwrap(),
            None
        );
        assert_eq!(
            Downsample::effective(Some(Downsample::average(8))).unwrap(),
            Some(Downsample::average(8))
        );
        assert_eq!(
            Downsample::effective(Some(Downsample::average(0)))
                .unwrap_err()
                .status,
            PicoStatus::INVALID_SAMPLERATIO
        );
        assert_eq!(Downsample::ratio_of(None), 1);
        assert_eq!(Downsample::ratio_of(Some(Downsample::average(16))), 16);
    }
}
