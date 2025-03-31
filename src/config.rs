use crate::models::enums::InstrumentType;
use crate::quotes::get_tech_analysis::IndicatorInterval;
use crate::instruments::get_assets::InstrumentStatus;
use serde::Deserialize;
use std::error::Error;
use std::fs;

// Конфигурация
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub api_token: String,
    pub telegram_token: String,
    pub class_code: String,
    pub instrument_type: InstrumentType,
    pub scan_interval_seconds: u64,
    pub strategy: StrategyConfig,
    pub assets: AssetsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetsConfig {
    pub instrument_type: InstrumentType,
    pub instrument_status: InstrumentStatus,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StrategyConfig {
    pub short_ema_length: i32,
    pub long_ema_length: i32,
    pub interval: IndicatorInterval,
}

impl Config {
    pub fn new(_path: &str) -> Result<Self, Box<dyn Error>> {
        // Чтение файла конфигурации
        let config_data = fs::read_to_string("config.yaml")?;
        let config: Config = serde_yaml::from_str(&config_data)?;

        Ok(config)
    }
}
