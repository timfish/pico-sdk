use std::{ffi::CString, str};

/// Pico drivers require strings as *mut i8. This converts from Rust
/// to Pico string format
pub trait ToPicoStr {
    /// Converts Rust strings to Pico null terminated `Vec<i8>` format
    fn into_pico_i8_string(self) -> Vec<i8>;
}

impl<'a> ToPicoStr for &'a str {
    fn into_pico_i8_string(self) -> Vec<i8> {
        CString::new(self)
            .expect("invalid CString")
            .into_bytes_with_nul()
            .iter()
            .map(|&x| x as i8)
            .collect()
    }
}

/// Pico drivers return strings as *i8. This converts from Pico to Rust string
/// formats
pub trait FromPicoStr {
    /// Converts from Pico null terminated `Vec<i8>` string format to Rust Strings
    fn into_string(self, buf_len: usize) -> String;
}

impl FromPicoStr for &[i8] {
    fn into_string(self, buf_len: usize) -> String {
        let vec: Vec<_> = self[..(buf_len - 1)].iter().map(|&x| x as u8).collect();

        str::from_utf8(&vec)
            .expect("invalid utf8 string")
            // This should not be required but older versions of the 5000a
            // driver return the wrong buf_len for the driver version string.
            // This trims the extra nulls that we get in the buffer
            .trim_matches(char::from(0))
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pico_strings() {
        let s1 = "something here";
        let ps = s1.into_pico_i8_string();

        assert_eq!(
            ps,
            vec![115, 111, 109, 101, 116, 104, 105, 110, 103, 32, 104, 101, 114, 101, 0]
        );

        let s2 = ps.into_string(ps.len());
        assert_eq!(s1, s2)
    }

    #[test]
    fn pico_strings_ps5000a_bug() {
        let s1 = "something here";
        // Add a load of nulls on the end
        let ps = [s1.into_pico_i8_string(), vec![0; 200]].concat();
        let s2 = ps.into_string(ps.len());
        assert_eq!(s1, s2)
    }
}
