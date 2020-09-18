use pico_common::PicoChannel;
use std::collections::HashMap;

#[derive(Clone)]
/// Events returned by the `PicoStreamingDevice`
pub enum StreamingEvent {
    Start,
    Stop,
    Data {
        length: usize,
        interval: f64,
        channels: HashMap<PicoChannel, RawChannelDataBlock>,
    },
    LostConnection,
}

#[derive(Clone)]
/// A struct containing raw channel data and scaling factors to get scaled samples
pub struct RawChannelDataBlock {
    pub max_adc_value: f32,
    pub max_scaled_value: f32,
    pub samples: Vec<i16>,
}

impl RawChannelDataBlock {
    pub fn scale_samples(&self) -> Vec<f32> {
        let multiplier = self.max_scaled_value / self.max_adc_value;

        self.samples
            .iter()
            .map(|v| *v as f32 * multiplier)
            .collect()
    }

    pub fn scale_sample(&self, index: usize) -> f32 {
        let multiplier = self.max_scaled_value / self.max_adc_value;

        self.samples[index] as f32 * multiplier
    }
}
