use super::PicoStatus;
use std::fmt;
use thiserror::Error;

/// Error encapsulating `PicoStatus` error codes with context
#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub struct PicoError {
    #[source]
    pub status: PicoStatus,
    context: Option<String>,
}

impl fmt::Display for PicoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ctx) = &self.context {
            write!(f, "{}: {}", self.status, ctx)
        } else {
            write!(f, "{}", self.status)
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
