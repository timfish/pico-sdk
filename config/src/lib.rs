use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use uncased::{Uncased, UncasedStr};
use value::ConfigValue;

mod error;
mod ext;
mod fuzzy;
mod value;

pub use error::ConfigError;
pub use ext::{ChannelConfigExt, DeviceConfigExt};
pub use fuzzy::{find_fuzzy, parse_pico_range_fuzzy};

type ParseFn = fn(&str, &[String]) -> Result<String, ConfigError>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Select {
    pub choices: Vec<String>,
    pub default: String,
    pub current: String,
    #[serde(skip)]
    pub parse: Option<ParseFn>,
}

impl Select {
    pub fn new<I: IntoIterator<Item = V>, V: AsRef<str>>(
        choices: I,
        default: String,
        parse: Option<ParseFn>,
    ) -> Self {
        Select {
            choices: choices.into_iter().map(|x| x.as_ref().to_owned()).collect(),
            current: default.to_string(),
            default,
            parse,
        }
    }

    pub fn set(&mut self, value: String) -> Result<(), ConfigError> {
        if self.choices.contains(&value.to_string()) {
            self.current = value.to_string();
            Ok(())
        } else if let Some(parse) = self.parse {
            self.current = parse(&value, &self.choices)?;
            Ok(())
        } else {
            Err(find_fuzzy(&value, &self.choices))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConfigType {
    Select(Select),
    Float(f64),
    Int(u64),
    Boolean(bool),
}

impl ConfigType {
    pub fn select<I: IntoIterator<Item = V>, V: AsRef<str>>(
        choices: I,
        default: &str,
        parse: Option<ParseFn>,
    ) -> Self {
        ConfigType::Select(Select::new(choices, default.to_string(), parse))
    }

    pub fn set<V: ConfigValue>(&mut self, value: V) -> Result<(), ConfigError> {
        match self {
            ConfigType::Select(choice) => {
                choice.set(value.get_string()?)?;
                Ok(())
            }
            ConfigType::Float(float) => {
                *float = value.get_float()?;
                Ok(())
            }
            ConfigType::Boolean(bool) => {
                *bool = value.get_boolean()?;
                Ok(())
            }
            ConfigType::Int(int) => {
                *int = value.get_integer()?;
                Ok(())
            }
        }
    }

    pub fn get_select(&self) -> Result<String, ConfigError> {
        match self {
            ConfigType::Select(choice) => Ok(choice.current.clone()),
            ConfigType::Float(_) => Err(ConfigError::InvalidSettingType {
                expected: "Select".to_string(),
                actual: "Float".to_string(),
            }),
            ConfigType::Boolean(_) => Err(ConfigError::InvalidSettingType {
                expected: "Select".to_string(),
                actual: "Boolean".to_string(),
            }),
            ConfigType::Int(_) => Err(ConfigError::InvalidSettingType {
                expected: "Select".to_string(),
                actual: "Int".to_string(),
            }),
        }
    }

    pub fn get_float(&self) -> Result<f64, ConfigError> {
        match self {
            ConfigType::Select(_) => Err(ConfigError::InvalidSettingType {
                expected: "Float".to_string(),
                actual: "Select".to_string(),
            }),
            ConfigType::Float(float) => Ok(*float),
            ConfigType::Boolean(_) => Err(ConfigError::InvalidSettingType {
                expected: "Float".to_string(),
                actual: "Boolean".to_string(),
            }),
            ConfigType::Int(_) => Err(ConfigError::InvalidSettingType {
                expected: "Float".to_string(),
                actual: "Int".to_string(),
            }),
        }
    }

    pub fn get_boolean(&self) -> Result<bool, ConfigError> {
        match self {
            ConfigType::Select(_) => Err(ConfigError::InvalidSettingType {
                expected: "Boolean".to_string(),
                actual: "Select".to_string(),
            }),
            ConfigType::Float(_) => Err(ConfigError::InvalidSettingType {
                expected: "Boolean".to_string(),
                actual: "Float".to_string(),
            }),
            ConfigType::Boolean(bool) => Ok(*bool),
            ConfigType::Int(_) => Err(ConfigError::InvalidSettingType {
                expected: "Boolean".to_string(),
                actual: "Int".to_string(),
            }),
        }
    }

    pub fn get_int(&self) -> Result<u64, ConfigError> {
        match self {
            ConfigType::Select(_) => Err(ConfigError::InvalidSettingType {
                expected: "Int".to_string(),
                actual: "Select".to_string(),
            }),
            ConfigType::Float(_) => Err(ConfigError::InvalidSettingType {
                expected: "Int".to_string(),
                actual: "Float".to_string(),
            }),
            ConfigType::Boolean(_) => Err(ConfigError::InvalidSettingType {
                expected: "Int".to_string(),
                actual: "Boolean".to_string(),
            }),
            ConfigType::Int(int) => Ok(*int),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub settings: HashMap<Uncased<'static>, ConfigType>,
}

impl ChannelConfig {
    pub fn new<'a, D: Iterator<Item = &'a (&'a str, ConfigType)>>(settings: D) -> Self {
        ChannelConfig {
            settings: settings
                .map(|(k, v)| (Uncased::new(k.to_string()), v.clone()))
                .collect(),
        }
    }

    pub fn set<V: ConfigValue>(
        &mut self,
        key: &str,
        value: V,
    ) -> Result<&mut ChannelConfig, ConfigError> {
        self.settings
            .get_mut(UncasedStr::new(key))
            .ok_or(ConfigError::UnknownSettingName(key.to_string()))?
            .set(value)?;
        Ok(self)
    }

    pub fn get(&self, key: &str) -> Result<&ConfigType, ConfigError> {
        self.settings
            .get(UncasedStr::new(key))
            .ok_or(ConfigError::UnknownSettingName(key.to_string()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    settings: HashMap<Uncased<'static>, ConfigType>,
    channels: HashMap<Uncased<'static>, ChannelConfig>,
}

static CHANNEL_NAMES: &[&str] = &["A", "B", "C", "D", "E", "F", "G", "H"];

impl DeviceConfig {
    pub fn new<'a, D: Iterator<Item = &'a (&'a str, ConfigType)>>(
        settings: D,
        channels: &[ChannelConfig],
    ) -> Self {
        let channels = channels
            .iter()
            .enumerate()
            .map(|(i, c)| (Uncased::new(CHANNEL_NAMES[i].to_string()), c.clone()))
            .collect();

        DeviceConfig {
            settings: settings
                .map(|(k, v)| (Uncased::new(k.to_string()), v.clone()))
                .collect(),
            channels,
        }
    }

    pub fn new_matching_channels<'a, D: Iterator<Item = &'a (&'a str, ConfigType)>>(
        settings: D,
        num_channels: usize,
        channel: ChannelConfig,
    ) -> Self {
        let channels = CHANNEL_NAMES[..num_channels]
            .iter()
            .map(|i| (Uncased::new(i.to_string()), channel.clone()))
            .collect();

        DeviceConfig {
            settings: settings
                .map(|(k, v)| (Uncased::new(k.to_string()), v.clone()))
                .collect(),
            channels,
        }
    }

    pub fn settings_iter(&self) -> impl Iterator<Item = (&Uncased<'_>, &ConfigType)> {
        self.settings.iter()
    }

    pub fn settings_iter_mut(&mut self) -> impl Iterator<Item = (&Uncased<'_>, &mut ConfigType)> {
        self.settings.iter_mut()
    }

    pub fn channels_iter(&self) -> impl Iterator<Item = (&Uncased<'_>, &ChannelConfig)> {
        self.channels.iter()
    }

    pub fn set<V: ConfigValue>(
        &mut self,
        key: &str,
        value: V,
    ) -> Result<&mut DeviceConfig, ConfigError> {
        self.settings
            .get_mut(UncasedStr::new(key))
            .ok_or(ConfigError::UnknownSettingName(key.to_string()))?
            .set(value)?;

        Ok(self)
    }

    pub fn get(&self, key: &str) -> Result<&ConfigType, ConfigError> {
        self.settings
            .get(UncasedStr::new(key))
            .ok_or(ConfigError::UnknownSettingName(key.to_string()))
    }

    pub fn channel(&mut self, channel: &str) -> Result<&mut ChannelConfig, ConfigError> {
        self.channels
            .get_mut(UncasedStr::new(channel))
            .ok_or(ConfigError::UnknownChannelName(channel.to_string()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    info: HashMap<String, String>,
}

impl DeviceInfo {
    pub fn new<'a, I: Iterator<Item = &'a (&'a str, String)>>(info: I) -> Self {
        Self {
            info: info.map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        }
    }

    pub fn info_iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.info.iter()
    }
}
