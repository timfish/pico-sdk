use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::error::ConfigError;

pub fn find_fuzzy(value: &str, choices: &[String]) -> ConfigError {
    let matcher = SkimMatcherV2::default().ignore_case();
    choices
        .iter()
        .fold(None, |acc, next| {
            match (acc.clone(), matcher.fuzzy_match(next, value)) {
                (None, Some(a)) => Some((next.to_owned(), a)),
                (Some(acc), Some(b)) if b > acc.1 => Some((next.to_owned(), b)),
                _ => acc,
            }
        })
        .map_or_else(
            || ConfigError::UnknownSettingValue(value.to_string()),
            |(s, _)| ConfigError::UnknownSettingValueSuggest {
                invalid: value.to_string(),
                suggestion: s.to_string(),
            },
        )
}

fn normalize_pico_range(input: &str) -> String {
    input.replace([' ', '±'], "").to_lowercase()
}

pub fn parse_pico_range_fuzzy(input: &str, choices: &[String]) -> Result<String, ConfigError> {
    let input = normalize_pico_range(input);

    for range in choices {
        let to_cmp = normalize_pico_range(&range.to_string());

        if input == to_cmp {
            return Ok(range.clone());
        }
    }

    Err(find_fuzzy(&input, choices))
}
