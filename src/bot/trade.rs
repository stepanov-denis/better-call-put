use crate::bot::signal::{CrossoverSignal, Signal, TradeSignal};
use crate::market_data_service::get_tech_analysis::{GetTechAnalysisRequest, GetTechAnalysisResponse, IndicatorInterval, TypeOfPrice};
use crate::market_data_service::get_last_prices::{GetLastPricesRequest, GetLastPricesResponse, LastPriceType, InstrumentStatus};
use reqwest::Client;
use tracing::info;

pub struct EmaCrossStrategy {
    instrument_uid: String,
    instrument_ticker: String,
    short_ema_length: i32,
    long_ema_length: i32,
    interval: IndicatorInterval,
    signal_generator: CrossoverSignal,
    last_short_ema: f64,
    last_long_ema: f64,
    last_price: f64,
}

impl EmaCrossStrategy {
    pub fn new(
        instrument_uid: String,
        instrument_ticker: String,
        short_ema_length: i32,
        long_ema_length: i32,
        interval: IndicatorInterval,
        hysteresis_percentage: f64,
        hysteresis_periods: u32,
    ) -> Self {
        Self {
            instrument_uid,
            instrument_ticker,
            short_ema_length,
            long_ema_length,
            interval,
            signal_generator: CrossoverSignal::new(hysteresis_percentage, hysteresis_periods),
            last_short_ema: 0.0,
            last_long_ema: 0.0,
            last_price: 0.0,
        }
    }

    /// Gets short EMA values
    pub async fn get_short_ema(
        &self,
        client: &reqwest::Client,
        token: &str,
    ) -> Result<GetTechAnalysisResponse, Box<dyn std::error::Error>> {
        let request = GetTechAnalysisRequest::new_ema_auto_period(
            &self.instrument_uid,
            self.interval.clone(),
            TypeOfPrice::Close,
            self.short_ema_length,
        );

        GetTechAnalysisResponse::get_tech_analysis(client, token, request).await
    }

    /// Gets long EMA values
    pub async fn get_long_ema(
        &self,
        client: &reqwest::Client,
        token: &str,
    ) -> Result<GetTechAnalysisResponse, Box<dyn std::error::Error>> {
        let request = GetTechAnalysisRequest::new_ema_auto_period(
            &self.instrument_uid,
            self.interval.clone(),
            TypeOfPrice::Close,
            self.long_ema_length,
        );

        GetTechAnalysisResponse::get_tech_analysis(client, token, request).await
    }

    /// Analyzes EMA crossover and returns trading signal
    pub fn analyze_crossover(
        &self,
        short_ema: &GetTechAnalysisResponse,
        long_ema: &GetTechAnalysisResponse,
    ) -> TradeSignal {
        // Get last two points for each EMA
        let short_values: Vec<_> = short_ema.technical_indicators
            .iter()
            .filter_map(|i| i.middle_band.as_ref())
            .rev()
            .take(2)
            .collect();

        let long_values: Vec<_> = long_ema.technical_indicators
            .iter()
            .filter_map(|i| i.middle_band.as_ref())
            .rev()
            .take(2)
            .collect();

        if short_values.len() < 2 || long_values.len() < 2 {
            return TradeSignal::Hold;
        }

        // Convert values to numbers for comparison
        let short_current = short_values[0].units.parse::<f64>().unwrap_or(0.0) 
            + (short_values[0].nano as f64 / 1_000_000_000.0);
        let short_previous = short_values[1].units.parse::<f64>().unwrap_or(0.0)
            + (short_values[1].nano as f64 / 1_000_000_000.0);
        
        let long_current = long_values[0].units.parse::<f64>().unwrap_or(0.0)
            + (long_values[0].nano as f64 / 1_000_000_000.0);
        let long_previous = long_values[1].units.parse::<f64>().unwrap_or(0.0)
            + (long_values[1].nano as f64 / 1_000_000_000.0);

        // Check for crossover
        if short_current > long_current && short_previous <= long_previous {
            TradeSignal::Buy
        } else if short_current < long_current && short_previous >= long_previous {
            TradeSignal::Sell
        } else {
            TradeSignal::Hold
        }
    }

    pub fn get_last_short(&self) -> f64 {
        self.last_short_ema
    }

    pub fn get_last_long(&self) -> f64 {
        self.last_long_ema
    }

    pub fn get_last_price(&self) -> f64 {
        self.last_price
    }

    /// Gets and analyzes trading signal
    pub async fn get_trade_signal(
        &mut self,
        client: &Client,
        token: &str,
    ) -> Result<Signal, Box<dyn std::error::Error>> {
        // Get last price
        let last_prices_request = GetLastPricesRequest::new(
            vec![self.instrument_uid.clone()],
            LastPriceType::Unspecified,
            InstrumentStatus::Base,
        );

        let last_prices = GetLastPricesResponse::get_last_prices(client, token, last_prices_request).await?;
        
        if let Some(last_price) = last_prices.last_prices.first() {
            self.last_price = last_price.price.units.parse::<f64>().unwrap_or(0.0) 
                + (last_price.price.nano as f64 / 1_000_000_000.0);
        }

        let short_ema = self.get_ema_values(client, token, self.short_ema_length).await?;
        let long_ema = self.get_ema_values(client, token, self.long_ema_length).await?;

        if short_ema.technical_indicators.is_empty() || long_ema.technical_indicators.is_empty() {
            info!("No data found for indicators");
            return Ok(Signal::Hold);
        }

        // Add debug output for all fields of the last indicator
        if let Some(last_indicator) = short_ema.technical_indicators.last() {
            info!(
                "Last indicator for short EMA:\n\
                 timestamp: {:?}\n\
                 signal: {:?}",
                last_indicator.timestamp,
                last_indicator.signal,
            );
        }

        self.last_short_ema = match short_ema.technical_indicators.last() {
            Some(indicator) => {
                match &indicator.signal {
                    Some(value) => {
                        let units = value.units.parse::<f64>().unwrap_or_else(|e| {
                            info!("Error parsing units for short EMA: {}", e);
                            0.0
                        });
                        let nanos = value.nano as f64 / 1_000_000_000.0;
                        units + nanos
                    }
                    None => {
                        info!("Missing signal value for short EMA");
                        0.0
                    }
                }
            }
            None => {
                info!("Failed to get last indicator for short EMA");
                0.0
            }
        };

        self.last_long_ema = match long_ema.technical_indicators.last() {
            Some(indicator) => {
                match &indicator.signal {
                    Some(value) => {
                        let units = value.units.parse::<f64>().unwrap_or_else(|e| {
                            info!("Error parsing units for long EMA: {}", e);
                            0.0
                        });
                        let nanos = value.nano as f64 / 1_000_000_000.0;
                        units + nanos
                    }
                    None => {
                        info!("Missing signal value for long EMA");
                        0.0
                    }
                }
            }
            None => {
                info!("Failed to get last indicator for long EMA");
                0.0
            }
        };

        info!(
            "Last EMA values - short: {:.6}, long: {:.6}",
            self.last_short_ema, self.last_long_ema
        );

        Ok(self.signal_generator.update(self.last_short_ema, self.last_long_ema))
    }

    async fn get_ema_values(
        &self,
        client: &Client,
        token: &str,
        length: i32,
    ) -> Result<GetTechAnalysisResponse, Box<dyn std::error::Error>> {
        let request = GetTechAnalysisRequest::new_ema_auto_period(
            &self.instrument_uid,
            self.interval,
            TypeOfPrice::Close,
            length,
        );

        GetTechAnalysisResponse::get_tech_analysis(client, token, request).await
    }

    pub fn get_ticker(&self) -> &str {
        &self.instrument_ticker
    }
} 