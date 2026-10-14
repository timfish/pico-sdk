use super::PSOSPADriver;
use crate::oscilloscope::AuxIoDriver;
use pico_common::{AuxIoMode, PicoResult, PicoStatus};
use pico_sys_dynamic::psospa::{
    enPicoAuxIoMode_PICO_AUXIO_HIGH_OUT, enPicoAuxIoMode_PICO_AUXIO_INPUT,
    enPicoAuxIoMode_PICO_AUXIO_LOW_OUT, enPicoAuxIoMode_PICO_AUXIO_TRIGGER_OUT, PICO_AUXIO_MODE,
};

fn aux_io_mode(mode: AuxIoMode) -> PICO_AUXIO_MODE {
    match mode {
        AuxIoMode::Input => enPicoAuxIoMode_PICO_AUXIO_INPUT,
        AuxIoMode::High => enPicoAuxIoMode_PICO_AUXIO_HIGH_OUT,
        AuxIoMode::Low => enPicoAuxIoMode_PICO_AUXIO_LOW_OUT,
        AuxIoMode::TriggerOut => enPicoAuxIoMode_PICO_AUXIO_TRIGGER_OUT,
    }
}

impl AuxIoDriver for PSOSPADriver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_aux_io_mode(&self, handle: i16, mode: AuxIoMode) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaSetAuxIoMode(handle, aux_io_mode(mode)) })
            .to_result((), "set_aux_io_mode")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modes_map_to_the_header_values() {
        assert_eq!(aux_io_mode(AuxIoMode::default()), 0);
        assert_eq!(aux_io_mode(AuxIoMode::High), 1);
        assert_eq!(aux_io_mode(AuxIoMode::Low), 2);
        assert_eq!(aux_io_mode(AuxIoMode::TriggerOut), 3);
    }
}
