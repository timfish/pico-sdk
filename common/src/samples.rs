use crate::{PicoError, PicoResult, PicoStatus};

/// The integer type of the samples in a buffer or a block.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SampleType {
    I8,
    I16,
    I32,
}

impl SampleType {
    /// Bytes a sample takes.
    pub fn size(self) -> usize {
        match self {
            SampleType::I8 => 1,
            SampleType::I16 => 2,
            SampleType::I32 => 4,
        }
    }
}

/// How a streaming downsample joins each group of samples.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DownsampleKind {
    /// The mean of each group, as 16-bit values.
    Average,
    /// The sum of each group, as 32-bit values. Only the ps6000a and
    /// psospa drivers have it.
    Sum,
}

/// A streaming downsample: the driver joins each `ratio` samples into
/// one, so the stored rate is the device's rate divided by `ratio`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Downsample {
    pub ratio: u32,
    pub mode: DownsampleKind,
}

impl Downsample {
    pub fn average(ratio: u32) -> Self {
        Downsample {
            ratio,
            mode: DownsampleKind::Average,
        }
    }

    pub fn sum(ratio: u32) -> Self {
        Downsample {
            ratio,
            mode: DownsampleKind::Sum,
        }
    }

    /// The downsample a driver needs: `None` for no downsample or a
    /// ratio of 1, an error for a ratio of 0.
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

    /// The ratio of an optional downsample, 1 for none.
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
