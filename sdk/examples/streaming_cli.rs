#![forbid(unsafe_code)]

use anyhow::{anyhow, Result};
use console::{style, Style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use pico_sdk::prelude::*;
use pico_streaming::EventHandler;
use signifix::metric;
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

fn better_theme() -> ColorfulTheme {
    ColorfulTheme {
        defaults_style: Style::new(),
        inactive_item_style: Style::new(),
        active_item_style: Style::new().bold(),
        active_item_prefix: style(">".to_string()).for_stderr().bold().green(),
        ..ColorfulTheme::default()
    }
}

#[derive(Clone)]
struct RateCalc {
    queue: Arc<Mutex<VecDeque<(Instant, u64)>>>,
    window_size: Duration,
}

impl RateCalc {
    pub fn new(window_size: Duration) -> Self {
        RateCalc {
            queue: Default::default(),
            window_size,
        }
    }

    pub fn get_value(&self, latest: usize) -> u64 {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back((Instant::now(), latest as u64));

        let mut max = 0;
        let mut total = 0;
        for (index, (timestamp, value)) in queue.iter_mut().enumerate() {
            if timestamp.elapsed() > self.window_size {
                max = index;
            } else {
                total += *value;
            }
        }

        for _ in 0..max {
            queue.pop_front();
        }

        queue
            .front()
            .map(|(f, _)| (total as f64 / f.elapsed().as_secs_f64()) as u64)
            .unwrap_or(0)
    }
}

fn main() -> Result<()> {
    if std::env::args().any(|a| a.contains("--trace")) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
            .init();
    }

    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());
    let device = select_device(&enumerator)?;

    let info = device.info.clone().expect("Device should be open");

    let streaming_device = device.into_streaming_device();
    let mut config = ScopeConfig::default();

    let ch_units = configure_channels(&info, &mut config);
    config.samples_per_second = get_capture_rate();

    let capture_stats: Arc<dyn EventHandler<ScopeStreamingEvent>> = CaptureStats::new(ch_units);
    streaming_device.events.subscribe(&capture_stats);

    println!("Press Enter to stop streaming");
    streaming_device.start(config).unwrap();

    Term::stdout().read_line().unwrap();

    streaming_device.stop();

    Ok(())
}

fn select_device(enumerator: &DeviceEnumerator) -> Result<ScopeDevice> {
    loop {
        println!("Searching for devices...",);

        let devices = enumerator.enumerate();

        if devices.is_empty() {
            return Err(anyhow!("{}", style("No Pico devices found").red()));
        }

        let device_options = devices
            .iter()
            .map(|result| match result {
                Ok(d) => format!("PicoScope {} ({})", d.variant, d.serial),
                Err(EnumerationError::DriverLoadError { driver, .. }) => {
                    format!("PicoScope {} (Missing Driver)", driver)
                }
                Err(EnumerationError::DriverError { driver, error }) => {
                    format!("PicoScope {} (Driver Error - {})", driver, error)
                }
                Err(EnumerationError::KernelDriverError { driver }) => {
                    format!("PicoScope {} (Kernel Driver Missing)", driver)
                }
                Err(EnumerationError::VersionError { driver, .. }) => {
                    format!("PicoScope {} (Driver Version Error)", driver)
                }
            })
            .collect::<Vec<String>>();

        let device_selection = Select::with_theme(&better_theme())
            .with_prompt(&format!(
                "{}",
                style("Select a device").blue().underlined().bold()
            ))
            .default(0)
            .items(&device_options[..])
            .interact()
            .unwrap();

        println!();

        match &devices[device_selection] {
            Ok(d) => return Ok(d.open().unwrap()),
            Err(error) => match error {
                EnumerationError::DriverLoadError { driver, .. }
                | EnumerationError::VersionError {
                    driver,
                    found: _,
                    required: _,
                } => {
                    println!("Downloading {} driver", driver);
                    let _ = download_drivers_to_cache(&[*driver]);
                    println!("Download complete");
                }
                _ => {}
            },
        }
    }
}

fn configure_channels(info: &ScopeInfo, config: &mut ScopeConfig) -> HashMap<PicoChannel, String> {
    loop {
        let mut channels = info
            .get_channels()
            .iter()
            .map(|c| (*c, info.valid_channel_ranges.get(c), config.channels.get(c)))
            .collect::<Vec<_>>();

        channels.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

        let mut channel_options = channels
            .iter()
            .map(|(ch, ranges, config)| {
                if let Some(ranges) = ranges {
                    if let Some(config) = config {
                        format!(
                            "Channel {} - {}",
                            ch,
                            style(format!("{} / {:?}", config.range, config.coupling)).green()
                        )
                    } else if ranges.is_empty() {
                        format!(
                            "Channel {} - {}",
                            ch,
                            style("No Probe connected").red().bold()
                        )
                    } else {
                        format!("Channel {} - {}", ch, style("Disabled"))
                    }
                } else {
                    format!(
                        "Channel {} - {}",
                        ch,
                        style("Disabled due to power constraints").red().bold()
                    )
                }
            })
            .collect::<Vec<String>>();

        if channels.iter().any(|(_, _, config)| config.is_some()) {
            channel_options.push("Channel configuration complete".to_string());
        }

        let ch_selection = Select::with_theme(&better_theme())
            .with_prompt(&format!(
                "{}",
                style("Configure channels").blue().underlined().bold()
            ))
            .default(0)
            .items(&channel_options[..])
            .interact()
            .unwrap();

        if ch_selection >= channels.len() {
            return channels
                .iter()
                .filter_map(|(ch, _, config)| config.map(|c| (*ch, c.range.get_units().short)))
                .collect();
        }

        let (edit_channel, ranges, _) = channels[ch_selection];

        if let Some(ranges) = ranges {
            if ranges.is_empty() {
                println!(
                    "{} cannot be configured with no probe connected",
                    edit_channel
                );

                continue;
            }

            if let Some(range) = select_range(ranges) {
                config.enable_channel(edit_channel, range, PicoCoupling::DC);
            } else {
                config.disable_channel(edit_channel);
            }
        } else {
            println!(
                "{} cannot be configured as it's disabled due to power constraints",
                edit_channel
            );
        }
    }
}

fn get_colour(ch: PicoChannel) -> Style {
    match ch {
        PicoChannel::A => Style::new().blue(),
        PicoChannel::B => Style::new().red(),
        PicoChannel::C => Style::new().green(),
        PicoChannel::D => Style::new().yellow(),
        PicoChannel::E => Style::new().magenta(),
        PicoChannel::F => Style::new().white(),
        PicoChannel::G => Style::new().cyan(),
        _ => Style::new().white(),
    }
}

struct CaptureStats {
    term: Term,
    rate_calc: RateCalc,
    ch_units: HashMap<PicoChannel, String>,
}

impl CaptureStats {
    pub fn new(ch_units: HashMap<PicoChannel, String>) -> Arc<Self> {
        Arc::new(CaptureStats {
            term: Term::stdout(),
            rate_calc: RateCalc::new(Duration::from_secs(5)),
            ch_units,
        })
    }
}

impl EventHandler<ScopeStreamingEvent> for CaptureStats {
    #[tracing::instrument(level = "trace", skip(self, event))]
    fn new_data(&self, event: &ScopeStreamingEvent) {
        let mut data: Vec<(PicoChannel, usize, f64, String)> = event
            .channels
            .iter()
            .map(|(ch, v)| {
                (
                    *ch,
                    v.samples.len(),
                    v.scale_sample(0),
                    self.ch_units.get(ch).unwrap_or(&"".to_string()).to_string(),
                )
            })
            .collect();

        data.sort_by(|a, b| a.0.cmp(&b.0));

        self.term.clear_last_lines(data.len() + 1).unwrap();

        println!(
            "{} @ {}",
            style("Streaming").bold(),
            style(format!(
                "{}S/s",
                match metric::Signifix::try_from(self.rate_calc.get_value(event.length)) {
                    Ok(v) => format!("{}", v),
                    Err(metric::Error::OutOfLowerBound(_)) => "0".to_string(),
                    _ => panic!("unknown error"),
                }
            ))
            .bold()
        );

        for (ch, _, first, unit) in data {
            let ch_col = get_colour(ch);

            let value = match metric::Signifix::try_from(first) {
                Ok(v) => format!("{}", v),
                Err(metric::Error::OutOfLowerBound(_)) => "0".to_string(),
                _ => panic!("unknown error"),
            };

            println!(
                "  {} - {}",
                ch_col.apply_to(ch).bold(),
                style(format!("{} {}", value, unit)).bold()
            );
        }
    }
}

fn get_capture_rate() -> u32 {
    let rates: Vec<u32> = vec![
        1_000,
        10_000,
        100_000,
        1_000_000,
        5_000_000,
        10_000_000,
        20_000_000,
        50_000_000,
        100_000_000,
    ];

    let rate_options: Vec<String> = rates
        .iter()
        .map(|r| {
            let sig = metric::Signifix::try_from(*r).unwrap();
            format!(
                "{:>8}",
                format!("{} {}S/s", sig.integer(), sig.symbol().unwrap_or(""))
            )
        })
        .collect();

    let rate_selection = Select::with_theme(&better_theme())
        .with_prompt(&format!(
            "{}",
            style("Select capture rate").blue().underlined().bold()
        ))
        .default(0)
        .items(&rate_options[..])
        .interact()
        .unwrap();

    rates[rate_selection]
}

fn select_range(ranges: &[PicoRange]) -> Option<PicoRange> {
    let mut range_options: Vec<String> = ranges.iter().map(|r| format!("{}", r)).collect();
    range_options.push("Disabled".to_string());

    let range_selection = Select::with_theme(&better_theme())
        .with_prompt(&format!(
            "{}",
            style("Select Range").blue().underlined().bold()
        ))
        .default(0)
        .items(&range_options[..])
        .interact()
        .unwrap();

    if range_selection >= ranges.len() {
        None
    } else {
        Some(ranges[range_selection])
    }
}
