use thiserror::Error;

#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub enum ConfigError {
    #[error("Setting named '{0}' not found")]
    UnknownSettingName(String),
    #[error("Channel named '{0}' not found")]
    UnknownChannelName(String),
    #[error("Value '{0}' is not valid")]
    UnknownSettingValue(String),
    #[error("Value '{invalid}' is not valid. Did you mean '{suggestion}'?")]
    UnknownSettingValueSuggest { invalid: String, suggestion: String },
    #[error("Invalid value. Could not convert '{input}' to '{target_type}'")]
    ValueConversionFailed { input: String, target_type: String },
    #[error("Invalid setting type. Expected '{expected}' but found '{actual}'")]
    InvalidSettingType { expected: String, actual: String },
}
