use pico_config::*;

fn normalize(input: &str) -> String {
    input.replace([' ', '±'], "").to_lowercase()
}

pub fn parse_ranges(input: &str, choices: &[String]) -> Result<String, ConfigError> {
    let input = normalize(input);

    for range in choices {
        let to_cmp = normalize(&range.to_string());

        if input == to_cmp {
            return Ok(range.clone());
        }
    }

    Err(find_fuzzy(&input, choices))
}

fn get_device() -> DeviceConfig {
    let ch = ChannelConfig::new(
        [
            ("Coupling", ConfigType::select(["AC", "DC"], "DC", None)),
            (
                "Range",
                ConfigType::select(
                    ["±100 mV", "±200 mV", "±500 mV", "±1 V"],
                    "±1 V",
                    Some(parse_ranges),
                ),
            ),
            ("Offset", ConfigType::Float(0.0)),
        ]
        .iter(),
    );

    let device = DeviceConfig::new_matching_channels(
        [(
            "Resolution",
            ConfigType::select(["8 bit", "10 bit", "12 bit"], "10 bit", None),
        )]
        .iter(),
        4,
        ch,
    );

    device
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device_config = get_device();
    device_config
        .set("resolution", "10 bit")?
        .channel("a")?
        .set("range", "100mV")?
        .set("offset", 1.234)?;

    let json: Result<String, serde_json::Error> = serde_json::to_string_pretty(&device_config);

    println!("{}", json.unwrap());

    Ok(())
}
