//! Signal generator plans shared by the families of each API.

pub(crate) mod legacy;
pub(crate) mod properties;

/// Maps samples of -32767 to 32767 onto the span the driver takes.
pub(crate) fn scale_samples(samples: &[i16], (low, high): (f64, f64)) -> Vec<i16> {
    if low == -32767.0 && high == 32767.0 {
        return samples.to_vec();
    }
    samples
        .iter()
        .map(|&s| (low + (f64::from(s) + 32767.0) * (high - low) / 65534.0).round() as i16)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_map_onto_the_driver_span() {
        assert_eq!(
            scale_samples(&[-32767, 0, 32767], (-8191.0, 8191.0)),
            vec![-8191, 0, 8191]
        );
        assert_eq!(scale_samples(&[5, -5], (-32767.0, 32767.0)), vec![5, -5]);
    }
}
