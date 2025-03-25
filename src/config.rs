use serde::Deserialize;
use std::error::Error;
use std::fs;

// Конфигурация
#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_token: String,
    pub class_code: String,
    pub instrument_type: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        // Чтение файла конфигурации
        let config_data = fs::read_to_string("config.yaml")?;
        let config: Config = serde_yaml::from_str(&config_data)?;

        Ok(config)
    }
}
