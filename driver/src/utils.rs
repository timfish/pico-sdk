use crate::EnumerationResult;
use pico_common::FromPicoStr;

pub fn parse_enum_result(buffer: &[i8], len: usize) -> Vec<EnumerationResult> {
    let serials_list = buffer.from_pico_i8_string(len);

    serials_list
        .split(',')
        .map(|device| {
            let mut parts = device.split('[');

            EnumerationResult {
                serial: parts
                    .next()
                    .expect("Failed to parse enumeration result")
                    .to_string(),
                variant: parts
                    .next()
                    .expect("Failed to parse enumeration result")
                    .trim_end_matches(']')
                    .to_string(),
            }
        })
        .collect()
}

pub fn parse_version_string(input: &str) -> String {
    input
        .split([' ', ','])
        .next_back()
        .expect("Failed to parse driver version string")
        .to_string()
}
