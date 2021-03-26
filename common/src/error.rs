use super::PicoStatus;
use std::fmt;
use thiserror::Error;

/// Error encapsulating `PicoStatus` error codes with context
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub struct PicoError {
    #[source]
    pub status: PicoStatus,
    context: Option<String>,
}

impl fmt::Display for PicoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ctx) = &self.context {
            write!(f, "{}", ctx)
        } else {
            Ok(())
        }
    }
}

impl PicoError {
    pub fn from_status(status: PicoStatus, context: &str) -> PicoError {
        PicoError {
            status,
            context: Some(context.to_string()),
        }
    }
}

impl From<PicoStatus> for PicoError {
    fn from(value: PicoStatus) -> Self {
        PicoError {
            status: value,
            context: None,
        }
    }
}

/// A result wrapping driver error codes: `Result<T, PicoError>`
pub type PicoResult<T> = Result<T, PicoError>;
