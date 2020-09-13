#![forbid(unsafe_code)]

use anyhow::{anyhow, Result};
use console::{style, Style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use pico_sdk::{
    common::{PicoChannel, PicoRange},
    device::{ChannelDetails, PicoDevice},
    download::{cache_resolution, download_drivers_to_cache},
    enumeration::{DeviceEnumerator, EnumerationError},
    streaming::{StreamingEvent, SubscribeToReader, ToStreamDevice},
};
use signifix::metric;
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    sync::{Arc, Mutex},
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
    queue: Arc<Mutex<VecDeque<u64>>>,
}

impl RateCalc {
    pub fn new() -> Self {
        RateCalc {
            queue: Default::default(),
        }
    }

    pub fn get_value(&self, latest: usize) -> u64 {
        let mut calc = self.queue.lock().unwrap();
        calc.push_back(latest as u64);

        while calc.len() > 20 {
            calc.pop_front();
        }

        (calc.iter().sum::<u64>() * 10) / calc.len() as u64
    }
}

fn main() -> Result<()> {
    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());
    let device = select_device(&enumerator)?;
    let ch_units = configure_channels(&device);

    let streaming_device = device.to_streaming_device();
    streaming_device.set_samples_per_second(get_capture_rate())?;

    let term = Term::stdout();
    let rate_calc = RateCalc::new();

    let _sub = streaming_device
        .events
        .subscribe_on_thread(Box::new(move |event| {
            display_capture_stats(event, &term, &rate_calc, &ch_units);
        }));

    println!("Press Enter to stop streaming");

    streaming_device.start().unwrap();

    Term::stdout().read_line().unwrap();

    streaming_device.stop();

    Ok(())
}

fn select_device(enumerator: &DeviceEnumerator) -> Result<PicoDevice> {
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
                Err(EnumerationError::DriverLoadError { driver }) => {
                    format!("PicoScope {} (Missing Driver)", driver)
                }
                Err(EnumerationError::DriverError { driver, error }) => {
                    format!("PicoScope {} (Driver Error - {})", driver, error)
                }
                Err(EnumerationError::VersionError {
                    driver,
                    found: _,
                    required: _,
                }) => format!("PicoScope {} (Driver Version Error)", driver),
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
            Ok(d) => return Ok(d.clone()),
            Err(error) => match error {
                EnumerationError::DriverLoadError { driver }
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

fn configure_channels(device: &PicoDevice) -> HashMap<PicoChannel, String> {
    loop {
        let mut channels: Vec<_> = {
            device
                .channels
                .read()
                .iter()
                .map(|(ch, cfg)| (*ch, cfg.clone()))
                .collect()
        };

        channels.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut channel_options = channels
            .iter()
            .map(|(ch, details)| {
                if let Some(ranges) = device.get_valid_ranges(*ch) {
                    if details.configuration.enabled {
                        format!(
                            "Channel {} - {}",
                            ch,
                            style(format!(
                                "{} / {:?}",
                                details.configuration.range, details.configuration.coupling
                            ))
                            .green()
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

        if channels
            .iter()
            .any(|(_, details)| details.configuration.enabled)
        {
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
                .map(|(ch, details)| (*ch, details.configuration.range.get_units().short))
                .collect();
        }

        let (edit_channel, edit_config) = channels[ch_selection].clone();

        if let Some(ranges) = device.get_valid_ranges(edit_channel) {
            if ranges.is_empty() {
                println!(
                    "{} cannot be configured with no probe connected",
                    edit_channel
                );

                continue;
            }

            let channel = display_channel(edit_config.clone(), &ranges);

            if channel.configuration.enabled {
                device.enable_channel(
                    edit_channel,
                    channel.configuration.range,
                    channel.configuration.coupling,
                );
            } else {
                device.disable_channel(edit_channel);
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

fn display_capture_stats(
    event: StreamingEvent,
    term: &Term,
    rate_calc: &RateCalc,
    ch_units: &HashMap<PicoChannel, String>,
) {
    if let StreamingEvent::Data {
        length: _,
        interval: _,
        channels,
    } = event
    {
        let mut data: Vec<(PicoChannel, usize, f32, String)> = channels
            .iter()
            .map(|(ch, v)| {
                (
                    *ch,
                    v.samples.len(),
                    v.scale_sample(0),
                    ch_units.get(&ch).unwrap_or(&"".to_string()).to_string(),
                )
            })
            .collect();

        let (_, samples, _, _) = data[0];

        data.sort_by(|a, b| a.0.cmp(&b.0));

        term.clear_last_lines(data.len() + 1).unwrap();

        println!(
            "{} @ {}",
            format!("{}", style("Streaming").bold()),
            format!(
                "{}",
                style(format!(
                    "{}S/s",
                    metric::Signifix::try_from(rate_calc.get_value(samples)).unwrap()
                ))
                .bold()
            )
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
                format!("{}", ch_col.apply_to(ch).bold()),
                format!("{}", style(format!("{} {}", value, unit)).bold())
            );
        }
    };
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

fn display_channel(channel: ChannelDetails, ranges: &[PicoRange]) -> ChannelDetails {
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

    let mut channel = channel;

    channel.configuration.enabled = range_selection < ranges.len();

    if channel.configuration.enabled {
        channel.configuration.range = ranges[range_selection];
    }

    channel
}
