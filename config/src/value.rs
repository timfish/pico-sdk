use crate::error::ConfigError;

pub trait ConfigValue {
    fn get_string(&self) -> Result<String, ConfigError>;
    fn get_float(&self) -> Result<f64, ConfigError>;
    fn get_boolean(&self) -> Result<bool, ConfigError>;
    fn get_integer(&self) -> Result<u64, ConfigError>;
}

impl ConfigValue for f64 {
    fn get_string(&self) -> Result<String, ConfigError> {
        Ok(self.to_string())
    }

    fn get_float(&self) -> Result<f64, ConfigError> {
        Ok(*self)
    }

    fn get_boolean(&self) -> Result<bool, ConfigError> {
        Ok(*self != 0.0)
    }

    fn get_integer(&self) -> Result<u64, ConfigError> {
        Ok(*self as u64)
    }
}

impl ConfigValue for i32 {
    fn get_string(&self) -> Result<String, ConfigError> {
        Ok(self.to_string())
    }

    fn get_float(&self) -> Result<f64, ConfigError> {
        Ok(*self as f64)
    }

    fn get_boolean(&self) -> Result<bool, ConfigError> {
        Ok(*self != 0)
    }

    fn get_integer(&self) -> Result<u64, ConfigError> {
        Ok(*self as u64)
    }
}

impl ConfigValue for &str {
    fn get_string(&self) -> Result<String, ConfigError> {
        Ok(self.to_string())
    }

    fn get_float(&self) -> Result<f64, ConfigError> {
        self.parse::<f64>()
            .map_err(|_| ConfigError::ValueConversionFailed {
                input: self.to_string(),
                target_type: "f64".to_string(),
            })
    }

    fn get_boolean(&self) -> Result<bool, ConfigError> {
        self.parse::<bool>()
            .map_err(|_| ConfigError::ValueConversionFailed {
                input: self.to_string(),
                target_type: "bool".to_string(),
            })
    }

    fn get_integer(&self) -> Result<u64, ConfigError> {
        self.parse::<u64>()
            .map_err(|_| ConfigError::ValueConversionFailed {
                input: self.to_string(),
                target_type: "u64".to_string(),
            })
    }
}

impl ConfigValue for String {
    fn get_string(&self) -> Result<String, ConfigError> {
        Ok(self.to_string())
    }

    fn get_float(&self) -> Result<f64, ConfigError> {
        self.parse::<f64>()
            .map_err(|_| ConfigError::ValueConversionFailed {
                input: self.to_string(),
                target_type: "f64".to_string(),
            })
    }

    fn get_boolean(&self) -> Result<bool, ConfigError> {
        self.parse::<bool>()
            .map_err(|_| ConfigError::ValueConversionFailed {
                input: self.to_string(),
                target_type: "bool".to_string(),
            })
    }

    fn get_integer(&self) -> Result<u64, ConfigError> {
        self.parse::<u64>()
            .map_err(|_| ConfigError::ValueConversionFailed {
                input: self.to_string(),
                target_type: "u64".to_string(),
            })
    }
}

impl ConfigValue for bool {
    fn get_string(&self) -> Result<String, ConfigError> {
        Ok(self.to_string())
    }

    fn get_float(&self) -> Result<f64, ConfigError> {
        Err(ConfigError::ValueConversionFailed {
            input: self.to_string(),
            target_type: "f64".to_string(),
        })
    }

    fn get_boolean(&self) -> Result<bool, ConfigError> {
        Ok(*self)
    }

    fn get_integer(&self) -> Result<u64, ConfigError> {
        Err(ConfigError::ValueConversionFailed {
            input: self.to_string(),
            target_type: "u64".to_string(),
        })
    }
}
