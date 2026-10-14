use pico_common::{PicoError, PicoResult, TriggerConfig};

/// Triggers for block, rapid block and streaming. Each family turns
/// [`TriggerConfig`] into its own trigger model.
pub trait TriggerDriver {
    /// Sets the trigger, or turns it off with `TriggerConfig::None`.
    fn set_trigger(&self, _handle: i16, _trigger: &TriggerConfig) -> PicoResult<()> {
        Err(PicoError::not_supported("set_trigger"))
    }
    /// Whether the trigger and the pulse width qualifier are on.
    fn trigger_enabled(&self, _handle: i16) -> PicoResult<(bool, bool)> {
        Err(PicoError::not_supported("trigger_enabled"))
    }
}
