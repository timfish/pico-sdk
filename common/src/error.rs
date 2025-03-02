use super::PicoStatus;
use std::fmt;
use thiserror::Error;

/// Error encapsulating `PicoStatus` error codes with context
#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub struct PicoDriverError {
    #[source]
    pub status: PicoStatus,
    context: Option<String>,
}

impl fmt::Display for PicoDriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ctx) = &self.context {
            write!(f, "{} > {:?}", ctx, self.status)
        } else {
            write!(f, "{:?}", self.status)
        }
    }
}

impl PicoDriverError {
    pub fn from_status(status: PicoStatus, context: &str) -> PicoDriverError {
        PicoDriverError {
            status,
            context: Some(context.to_string()),
        }
    }
}

impl From<PicoStatus> for PicoDriverError {
    fn from(value: PicoStatus) -> Self {
        PicoDriverError {
            status: value,
            context: None,
        }
    }
}

/// A result wrapping driver error codes: `Result<T, PicoError>`
pub type PicoDriverResult<T> = Result<T, PicoDriverError>;
