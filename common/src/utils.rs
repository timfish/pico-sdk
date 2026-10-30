use std::ffi::CString;

/// Pico drivers require strings as *mut i8. This converts from Rust
/// to Pico string format
pub trait ToPicoStr {
    /// Converts Rust strings to Pico null terminated Vec<i8> format
    fn into_pico_i8_string(self) -> Vec<i8>;
}

impl ToPicoStr for &str {
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
    /// Reads a C string from the first `buf_len` bytes of the buffer.
    ///
    /// The string ends at the first NUL, or at `buf_len` or the end of the
    /// buffer when there is none, so a length that counts the NUL, misses it
    /// or runs past the buffer gives the same string. Bytes that are not
    /// UTF-8 become U+FFFD.
    #[allow(clippy::wrong_self_convention)]
    fn from_pico_i8_string(self, buf_len: usize) -> String;
}

impl FromPicoStr for &[i8] {
    fn from_pico_i8_string(self, buf_len: usize) -> String {
        let bytes: Vec<u8> = self[..buf_len.min(self.len())]
            .iter()
            .take_while(|&&b| b != 0)
            .map(|&b| b as u8)
            .collect();

        String::from_utf8_lossy(&bytes).into_owned()
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

        let s2 = ps.from_pico_i8_string(ps.len());
        assert_eq!(s1, s2)
    }

    #[test]
    fn pico_strings_ps5000a_bug() {
        let s1 = "something here";
        // Add a load of nulls on the end
        let ps = [s1.into_pico_i8_string(), vec![0; 200]].concat();
        let s2 = ps.from_pico_i8_string(ps.len());
        assert_eq!(s1, s2)
    }

    fn pico(bytes: &[u8]) -> Vec<i8> {
        bytes.iter().map(|&b| b as i8).collect()
    }

    #[test]
    fn ends_at_first_nul_within_length() {
        let buf = pico(b"ES283/108\0");
        assert_eq!(buf.from_pico_i8_string(buf.len()), "ES283/108");
        assert_eq!(buf.from_pico_i8_string(10), "ES283/108");
    }

    #[test]
    fn no_nul_takes_the_length() {
        let buf = pico(b"AB123/0045");
        assert_eq!(buf.from_pico_i8_string(buf.len()), "AB123/0045");
        assert_eq!(buf.from_pico_i8_string(4), "AB12");
    }

    #[test]
    fn length_zero_is_empty() {
        assert_eq!(pico(b"abc\0").from_pico_i8_string(0), "");
        assert_eq!(Vec::<i8>::new().from_pico_i8_string(0), "");
    }

    #[test]
    fn length_past_buffer_stops_at_buffer() {
        assert_eq!(pico(b"abc").from_pico_i8_string(300), "abc");
        assert_eq!(pico(b"abc\0junk").from_pico_i8_string(300), "abc");
    }

    #[test]
    fn invalid_utf8_is_replaced() {
        assert_eq!(pico(b"a\xffb\0").from_pico_i8_string(4), "a\u{fffd}b");
    }
}
