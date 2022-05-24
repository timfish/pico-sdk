use num_derive::{ToPrimitive, FromPrimitive};

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoWaveType {
    Sine = 0,
    Square = 1,
    Triangle = 2,
    RampUp = 3,
    RampDown = 4,
    Sinc = 5,
    Gaussian = 6,
    HalfSine = 7,
    DCVoltage = 8,
}

impl Default for PicoWaveType {
    fn default() -> Self {
        PicoWaveType::Sine
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoSweepType {
    Up = 0,
    Down = 1,
    UpDown = 2,
    DownUp = 3,
}

impl Default for PicoSweepType {
    fn default() -> Self {
        PicoSweepType::Up
    }
}

#[derive(Debug, Clone)]
pub struct SigGenArbitraryMinMaxValues {
    pub min_value: i16,
    pub max_value: i16,
    pub min_size: u32,
    pub max_size: u32,
}

// Rust addition: encode the potential values of sweeps and shots, to avoid invalid states
// like >0 & >0
#[derive(Debug, Clone)]
pub enum SweepShotCount {
    None,
    Sweeps(u32),
    Shots(u32),
    ContinuousSweeps,
    ContinuousShots,
}

impl Default for SweepShotCount {
    fn default() -> Self {
        SweepShotCount::None
    }
}

// TODO: this value is copied from sys/src/ps2000a - should we import it here?
// should there be a crate for identical symbols?
// should build.rs check for identity?
const COPY_PS2000A_SHOT_SWEEP_TRIGGER_CONTINUOUS_RUN: u32 = 4294967295;

impl SweepShotCount {
    pub fn to_sweeps(&self) -> u32 {
        match self {
            SweepShotCount::Sweeps(sweeps) => *sweeps,
            SweepShotCount::ContinuousSweeps => COPY_PS2000A_SHOT_SWEEP_TRIGGER_CONTINUOUS_RUN,
            _ => 0,
        }
    }

    pub fn to_shots(&self) -> u32 {
        match self {
            SweepShotCount::Shots(shots) => *shots,
            SweepShotCount::ContinuousShots => COPY_PS2000A_SHOT_SWEEP_TRIGGER_CONTINUOUS_RUN,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoExtraOperations {
    /// <summary>
    /// Normal signal generator operation specified by wavetype.
    /// </summary>
    Off = 0,
    /// <summary>
    /// The signal generator produces white noise and ignores all settings except pkToPk and offsetVoltage.
    /// </summary>
    WhiteNoise = 1,
    /// <summary>
    /// produces a pseudorandom random binary sequence with a bit rate
    /// specified by the start and stop frequency.
    /// </summary>
    PRBS = 2, // Pseudo-Random Bit Stream
}

impl Default for PicoExtraOperations {
    fn default() -> Self {
        PicoExtraOperations::Off
    }
}

/// <summary>
/// AWG index modes
/// </summary>
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoIndexMode {
    /// <summary>
    /// The generator outputs the raw contents of the buffer repeatedly .
    /// </summary>
    Single = 0,
    /// <summary>
    /// The generator outputs the contents of the buffer from beginning to end, and then does a second pass in the reverse
    /// direction through the buffer
    /// </summary>
    Dual = 1,
    /// <summary>
    /// This is similiar to the Dual but passes through the buffer four time inverting, and inverting reversed
    /// </summary>
    Quad = 2,
}

/// <summary>
/// The type of trigger that will be applied to the signal generator
/// </summary>
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoSigGenTrigType {
    /// <summary>
    /// Trigger on rising edge
    /// </summary>
    Rising = 0,
    /// <summary>
    /// Trigger on falling edge
    /// </summary>
    Falling = 1,
    /// <summary>
    /// Run while trigger is high
    /// </summary>
    GateHigh = 2,
    /// <summary>
    /// Run while trigger is low
    /// </summary>
    GateLow = 3,
}

impl Default for PicoSigGenTrigType {
    fn default() -> Self {
        PicoSigGenTrigType::Rising
    }
}

/// <summary>
/// The source that will trigger the signal generator
/// </summary>
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq)]
pub enum PicoSigGenTrigSource {
    /// <summary>
    /// Run without waiting for trigger
    /// </summary>
    None = 0,
    /// <summary>
    /// Use scope trigger
    /// </summary
    ScopeTrig = 1,
    /// <summary>
    /// Use AUXIO input
    /// </summary>
    AuxIn = 2,
    /// <summary>
    /// Use external input
    /// </summary>
    ExtIn = 3,
    /// <summary>
    /// Wait for software trigger
    /// </summary>
    SoftTrig = 4,
}

impl Default for PicoSigGenTrigSource {
    fn default() -> Self {
        PicoSigGenTrigSource::None
    }
}

#[derive(Default, Debug)]
pub struct SetSigGenBuiltInV2Properties {
    pub offset_voltage: i32, /* microvolts */
    pub pk_to_pk: u32,       /* microvolts */
    pub wave_type: PicoWaveType,
    pub start_frequency: f64, /* Hertz */
    pub stop_frequency: f64,  /* Hertz */
    pub increment: f64,       /* delta frequency jumps in Hertz */
    pub dwell_time: f64,      /* amount to stay at each frequency in seconds */
    pub sweep_type: PicoSweepType,
    pub extra_operations: PicoExtraOperations,
    pub sweeps_shots: SweepShotCount,
    pub trig_type: PicoSigGenTrigType,
    pub trig_source: PicoSigGenTrigSource,
    pub ext_in_threshold: i16,
}
