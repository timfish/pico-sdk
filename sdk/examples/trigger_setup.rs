//! Sets each trigger kind on a scope and reads back `trigger_enabled`,
//! then streams with an edge trigger.
//!
//! Opens the unit, turns on channels A and B at ±5 V DC and prints the
//! trigger capabilities. Then, for each case (simple edge, delay,
//! auto-trigger, advanced level with hysteresis, A AND B, A OR B,
//! window enter and exit, runt, pulse width of each type, holdoff,
//! digital and digital pulse width on MSO models, EXT and AUX):
//!
//! - a kind the capabilities lack must be refused before any driver
//!   call, with the status that names it;
//! - else `set_trigger` must return OK and `trigger_enabled` must give
//!   what the case turns on (trigger, pulse width qualifier).
//!
//! EXT and AUX are shown but not counted: the family's API has them,
//! and a model without the input gives the driver's own error. Last it
//! turns the trigger off and checks `(false, false)`, then streams A at
//! 1 MS/s for 2 s with a rising edge at 0 V and prints how many events
//! marked a trigger, and where (a signal must cross 0 V on A, such as
//! the AWG, for any to show). Exits 1 if a counted case failed.
//!
//! `cargo run --release --example trigger_setup -- psospa 13474/0004`
//! `cargo run --release --example trigger_setup -- 2000a CW356/055`

use pico_sdk::{common::*, prelude::*};
use std::{
    env,
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct Marks(Mutex<(u64, Vec<usize>)>);

impl EventHandler<OscilloscopeStreamEvent> for Marks {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        let mut marks = self.0.lock().unwrap();
        marks.0 += 1;
        if let Some(at) = event.trigger_at {
            marks.1.push(at);
        }
    }
}

struct Case {
    name: &'static str,
    config: TriggerConfig,
    /// Counted as a failure when it does not give what it should.
    counted: bool,
}

fn advanced(
    channels: Vec<ChannelTrigger>,
    conditions: TriggerConditions,
    pulse_width: Option<PulseWidthQualifier>,
) -> TriggerConfig {
    TriggerConfig::Advanced {
        channels,
        conditions,
        digital: Vec::new(),
        pulse_width,
        delay: 0,
        holdoff: None,
        auto_trigger: Some(Duration::from_millis(100)),
    }
}

fn cases(adc: i16) -> Vec<Case> {
    use ThresholdDirection::*;
    use TriggerSource::{Aux, Channel, Digital, External, PulseWidthQualifier as Pwq};
    use TriggerState::True;
    let a = Channel(PicoChannel::A);
    let b = Channel(PicoChannel::B);
    let level = |source, direction| ChannelTrigger::level(source, direction, adc, 256);
    let window = |direction| {
        ChannelTrigger::window(
            a,
            direction,
            Threshold::new(-adc, 256),
            Threshold::new(adc, 256),
        )
    };
    let pwq = |kind| PulseWidthQualifier {
        conditions: vec![vec![(a, True)]],
        direction: Rising,
        lower: 100,
        upper: 1000,
        kind,
        digital: Vec::new(),
    };
    let simple = |source, delay, auto| TriggerConfig::Simple {
        source,
        threshold_adc: adc,
        direction: Rising,
        delay,
        auto_trigger: auto,
    };
    let mut digital = advanced(Vec::new(), vec![vec![(Digital, True)]], None);
    if let TriggerConfig::Advanced { digital: bits, .. } = &mut digital {
        bits.push(DigitalBit::new(0, DigitalDirection::Rising));
        bits.push(DigitalBit::new(9, DigitalDirection::High));
    }
    let digital_pwq = advanced(
        Vec::new(),
        vec![vec![(Pwq, True)]],
        Some(PulseWidthQualifier {
            conditions: vec![vec![(Digital, True)]],
            direction: Rising,
            lower: 100,
            upper: 0,
            kind: PulseWidthType::GreaterThan,
            digital: vec![DigitalBit::new(3, DigitalDirection::High)],
        }),
    );
    let mut holdoff = advanced(vec![level(a, Rising)], vec![vec![(a, True)]], None);
    if let TriggerConfig::Advanced { holdoff: h, .. } = &mut holdoff {
        *h = Some(1000);
    }
    let mut delayed = advanced(vec![level(a, Falling)], vec![vec![(a, True)]], None);
    if let TriggerConfig::Advanced { delay, .. } = &mut delayed {
        *delay = 5000;
    }
    let with_pwq = |kind| {
        advanced(
            vec![level(a, Rising)],
            vec![vec![(a, True), (Pwq, True)]],
            Some(pwq(kind)),
        )
    };
    let counted = |name, config| Case {
        name,
        config,
        counted: true,
    };
    vec![
        counted(
            "simple rising A",
            TriggerConfig::edge(PicoChannel::A, adc, Rising),
        ),
        counted(
            "simple, auto-trigger 10 ms",
            simple(a, 0, Some(Duration::from_millis(10))),
        ),
        counted("simple, delay 1000 samples", simple(a, 1000, None)),
        counted(
            "advanced level A, hysteresis 256",
            advanced(vec![level(a, RisingOrFalling)], vec![vec![(a, True)]], None),
        ),
        counted("advanced, delay 5000 samples", delayed),
        counted(
            "A above AND B below",
            TriggerConfig::all_of(vec![level(a, Above), level(b, Below)]),
        ),
        counted(
            "A rising OR B falling",
            TriggerConfig::any_of(vec![level(a, Rising), level(b, Falling)]),
        ),
        counted("window enter A", TriggerConfig::any_of(vec![window(Enter)])),
        counted("window exit A", TriggerConfig::any_of(vec![window(Exit)])),
        counted(
            "window inside A",
            TriggerConfig::any_of(vec![window(Inside)]),
        ),
        counted(
            "positive runt A",
            TriggerConfig::any_of(vec![window(PositiveRunt)]),
        ),
        counted(
            "negative runt A",
            TriggerConfig::any_of(vec![window(NegativeRunt)]),
        ),
        counted("pulse width < 100", with_pwq(PulseWidthType::LessThan)),
        counted("pulse width > 100", with_pwq(PulseWidthType::GreaterThan)),
        counted(
            "pulse width in 100..1000",
            with_pwq(PulseWidthType::InRange),
        ),
        counted(
            "pulse width out of 100..1000",
            with_pwq(PulseWidthType::OutOfRange),
        ),
        counted("holdoff 1000 samples", holdoff),
        counted("digital D0 rising AND D9 high", digital),
        counted("digital pulse width on D3", digital_pwq),
        Case {
            name: "simple rising EXT",
            config: simple(External, 0, None),
            counted: false,
        },
        Case {
            name: "simple rising AUX",
            config: simple(Aux, 0, None),
            counted: false,
        },
    ]
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let mut args = env::args().skip(1);
    let driver_type = Driver::from_str(&args.next().unwrap_or_else(|| "psospa".into()))
        .expect("a driver type such as 2000a or psospa");
    let serial = args.next();

    let PicoDriver::Oscilloscope(driver) = driver_type
        .load(&cache_resolution())
        .unwrap_or_else(|e| panic!("could not load the {driver_type} driver: {e}"))
    else {
        panic!("{driver_type} is not a scope driver");
    };
    let device = driver
        .open_device(serial.as_deref())
        .unwrap_or_else(|e| panic!("could not open a {driver_type} unit: {e}"));
    let info = device.info.clone().expect("an open device has info");
    let handle = *info.handle;
    let caps = info.capabilities.triggers;
    println!("scope {} {}", device.variant, device.serial);
    println!("trigger capabilities: {caps:#?}");

    let channel = OscilloscopeChannelConfig {
        coupling: PicoCoupling::DC,
        range: PicoRange::X1_PROBE_5V,
        offset: 0.0,
        bandwidth: None,
    };
    for ch in [PicoChannel::A, PicoChannel::B] {
        device
            .driver
            .enable_channel(handle, ch, &channel)
            .unwrap_or_else(|e| panic!("could not turn on {ch}: {e}"));
    }

    // A tenth of full scale: 0.5 V on ±5 V.
    let adc = info.max_adc_value / 10;
    let mut failures = 0;
    for case in cases(adc) {
        let refusal = case
            .config
            .kinds()
            .into_iter()
            .find(|kind| !caps.has(*kind));
        let result = device.driver.set_trigger(handle, &case.config);
        let outcome = match (refusal, result) {
            (Some(kind), Err(e)) => Ok(format!("refused, no {}: {e}", kind.name())),
            (Some(kind), Ok(())) => Err(format!("set, though the unit has no {}", kind.name())),
            (None, Err(e)) => Err(format!("set_trigger: {e}")),
            (None, Ok(())) => match device.driver.trigger_enabled(handle) {
                Ok(on) if on == case.config.enables() => {
                    Ok(format!("set; enabled (trigger, pwq) = {on:?}"))
                }
                Ok(on) => Err(format!(
                    "enabled {on:?}, expected {:?}",
                    case.config.enables()
                )),
                Err(e) => Err(format!("trigger_enabled: {e}")),
            },
        };
        match (outcome, case.counted) {
            (Ok(text), _) => println!("OK    {:<32} {text}", case.name),
            (Err(text), true) => {
                failures += 1;
                println!("FAIL  {:<32} {text}", case.name);
            }
            (Err(text), false) => println!("INFO  {:<32} {text}", case.name),
        }
    }

    match device
        .driver
        .set_trigger(handle, &TriggerConfig::None)
        .and_then(|()| device.driver.trigger_enabled(handle))
    {
        Ok((false, false)) => println!("OK    {:<32} off", "trigger off"),
        other => {
            failures += 1;
            println!("FAIL  {:<32} {other:?}", "trigger off");
        }
    }

    let runner = device.into_streaming_device();
    let marks = Arc::new(Marks(Mutex::new((0, Vec::new()))));
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = marks.clone();
    runner.events.subscribe(&handler);
    let mut config = OscilloscopeConfig {
        trigger: TriggerConfig::edge(PicoChannel::A, 0, ThresholdDirection::Rising),
        ..Default::default()
    };
    config.set_sample_rate(1_000_000);
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_5V, PicoCoupling::DC);
    match runner.start(config) {
        Ok(()) => {
            println!(
                "streaming with an edge trigger; enabled = {:?}",
                runner.trigger_enabled()
            );
            thread::sleep(Duration::from_secs(2));
            runner.stop();
            let marks = marks.0.lock().unwrap();
            println!(
                "{} events, {} marked a trigger, at {:?}",
                marks.0,
                marks.1.len(),
                &marks.1[..marks.1.len().min(10)]
            );
        }
        Err(e) if !caps.streaming => println!("stream with a trigger refused: {e}"),
        Err(e) => {
            failures += 1;
            println!("FAIL  stream with a trigger: {e}");
        }
    }
    runner.close();
    println!("{failures} counted cases failed");
    if failures > 0 {
        std::process::exit(1);
    }
}
