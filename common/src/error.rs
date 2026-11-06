use super::PicoStatus;
use std::fmt;
use thiserror::Error;

/// An error from a Pico driver: a [`PicoStatus`] code and, when known, the call that returned it
///
/// - It prints as `context > STATUS`, or as `STATUS` when there is no context.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub struct PicoError {
    /// The status code the driver returned
    #[source]
    pub status: PicoStatus,
    context: Option<String>,
}

impl fmt::Display for PicoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ctx) = &self.context {
            write!(f, "{} > {:?}", ctx, self.status)
        } else {
            write!(f, "{:?}", self.status)
        }
    }
}

impl PicoError {
    /// Makes an error from a status code and the name of the call or step that failed
    pub fn from_status(status: PicoStatus, context: &str) -> PicoError {
        PicoError {
            status,
            context: Some(context.to_string()),
        }
    }

    /// Makes the error for a feature that the family or the model does not have
    ///
    /// - The status is [`PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE`] and `what` is the context.
    pub fn not_supported(what: &str) -> PicoError {
        PicoError::from_status(PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE, what)
    }

    /// Whether the status is [`PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE`]
    pub fn is_not_supported(&self) -> bool {
        self.status == PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE
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

/// The result of a driver call: `Result<T, PicoError>`
pub type PicoResult<T> = Result<T, PicoError>;
