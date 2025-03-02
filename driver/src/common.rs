use auto_enums::auto_enum;
use pico_common::PicoDriverError;
use pico_config::{ConfigError, DeviceConfig, DeviceInfo};
use std::{any::Any, collections::HashMap, fmt};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum PicoError {
    #[error(transparent)]
    DriverError(#[from] PicoDriverError),
    #[error("DriverLoadError: {0}")]
    DriverLoadError(String),
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}

impl From<libloading::Error> for PicoError {
    fn from(value: libloading::Error) -> Self {
        PicoError::DriverLoadError(format!("{:?}", value))
    }
}

/// Serial and variant pairs returned by driver enumeration
#[derive(Clone, Debug)]
pub struct EnumerationResult {
    pub variant: String,
    pub serial: String,
}

#[derive(Clone, Debug)]
pub struct OpenResult {
    pub handle: i16,
    pub serial: String,
}

pub type StreamingState = Box<dyn Any + Send + Sync>;

#[derive(Debug)]
pub enum StreamingChannelResult {
    Float(Vec<f64>),
    Buffer { data: Vec<i16>, multiplier: f64 },
}

impl StreamingChannelResult {
    #[auto_enum(Iterator)]
    pub fn iter(&self) -> impl Iterator<Item = f64> + '_ {
        match self {
            StreamingChannelResult::Float(data) => data.iter().copied(),
            StreamingChannelResult::Buffer {
                data, multiplier, ..
            } => data.iter().map(move |x| *x as f64 * multiplier),
        }
    }
}

#[derive(Debug)]
pub struct StreamingResult {
    pub channels: HashMap<String, StreamingChannelResult>,
    pub nano_seconds_interval: u64,
}

pub trait PicoDriver: fmt::Debug + Send + Sync {
    fn enumerate_units(&self) -> Result<Vec<EnumerationResult>, PicoError>;
    fn open_device(&self, serial: Option<&str>) -> Result<OpenResult, PicoError>;
    fn get_device_info(&self, handle: i16) -> Result<DeviceInfo, PicoError>;
    fn get_device_config(&self, handle: i16) -> Result<DeviceConfig, PicoError>;
    fn configure_device(&self, handle: i16, config: &DeviceConfig) -> Result<(), PicoError>;
    fn start_streaming(
        &self,
        handle: i16,
        config: &DeviceConfig,
    ) -> Result<StreamingState, PicoError>;
    fn get_streaming_values(
        &self,
        handle: i16,
        state: &StreamingState,
    ) -> Result<StreamingResult, PicoError>;
    fn update_streaming_buffers(
        &self,
        handle: i16,
        state: &StreamingState,
    ) -> Result<StreamingState, PicoError>;
    fn stop(&self, handle: i16) -> Result<(), PicoError>;
    fn close_device(&self, handle: i16) -> Result<(), PicoError>;
}
