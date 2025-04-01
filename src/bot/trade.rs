use crate::bot::signal::{CrossoverSignal, Signal, TradeSignal};
use crate::quotes::get_tech_analysis::{GetTechAnalysisRequest, GetTechAnalysisResponse, IndicatorInterval, TypeOfPrice};
use reqwest::Client;
use tracing::info;

pub struct EmaCrossStrategy {
    instrument_uid: String,
    short_ema_length: i32,
    long_ema_length: i32,
    interval: IndicatorInterval,
    signal_generator: CrossoverSignal,
}

impl EmaCrossStrategy {
    pub fn new(
        instrument_uid: String,
        short_ema_length: i32,
        long_ema_length: i32,
        interval: IndicatorInterval,
        hysteresis_percentage: f64,
        hysteresis_periods: u32,
    ) -> Self {
        Self {
            instrument_uid,
            short_ema_length,
            long_ema_length,
            interval,
            signal_generator: CrossoverSignal::new(hysteresis_percentage, hysteresis_periods),
        }
    }

    /// Получает значения короткой EMA
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

    /// Получает значения длинной EMA
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

    /// Анализирует пересечение EMA и возвращает торговый сигнал
    pub fn analyze_crossover(
        &self,
        short_ema: &GetTechAnalysisResponse,
        long_ema: &GetTechAnalysisResponse,
    ) -> TradeSignal {
        // Получаем последние две точки для каждой EMA
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

        // Конвертируем значения в числа для сравнения
        let short_current = short_values[0].units.parse::<f64>().unwrap_or(0.0) 
            + (short_values[0].nano as f64 / 1_000_000_000.0);
        let short_previous = short_values[1].units.parse::<f64>().unwrap_or(0.0)
            + (short_values[1].nano as f64 / 1_000_000_000.0);
        
        let long_current = long_values[0].units.parse::<f64>().unwrap_or(0.0)
            + (long_values[0].nano as f64 / 1_000_000_000.0);
        let long_previous = long_values[1].units.parse::<f64>().unwrap_or(0.0)
            + (long_values[1].nano as f64 / 1_000_000_000.0);

        // Проверяем пересечение
        if short_current > long_current && short_previous <= long_previous {
            TradeSignal::Buy
        } else if short_current < long_current && short_previous >= long_previous {
            TradeSignal::Sell
        } else {
            TradeSignal::Hold
        }
    }

    /// Получает и анализирует торговый сигнал
    pub async fn get_trade_signal(
        &mut self,
        client: &Client,
        token: &str,
    ) -> Result<Signal, Box<dyn std::error::Error>> {
        let short_ema = self.get_ema_values(client, token, self.short_ema_length).await?;
        let long_ema = self.get_ema_values(client, token, self.long_ema_length).await?;

        if short_ema.technical_indicators.is_empty() || long_ema.technical_indicators.is_empty() {
            info!("Нет данных индикаторов");
            return Ok(Signal::Hold);
        }

        // Добавляем отладочный вывод всех полей последнего индикатора
        if let Some(last_indicator) = short_ema.technical_indicators.last() {
            info!(
                "Последний индикатор короткой EMA:\n\
                 timestamp: {:?}\n\
                 signal: {:?}",
                last_indicator.timestamp,
                last_indicator.signal,
            );
        }

        let last_short = match short_ema.technical_indicators.last() {
            Some(indicator) => {
                match &indicator.signal {
                    Some(value) => {
                        let units = value.units.parse::<f64>().unwrap_or_else(|e| {
                            info!("Ошибка парсинга units для короткой EMA: {}", e);
                            0.0
                        });
                        let nanos = value.nano as f64 / 1_000_000_000.0;
                        units + nanos
                    }
                    None => {
                        info!("Отсутствует значение signal для короткой EMA");
                        0.0
                    }
                }
            }
            None => {
                info!("Не удалось получить последний индикатор для короткой EMA");
                0.0
            }
        };

        let last_long = match long_ema.technical_indicators.last() {
            Some(indicator) => {
                match &indicator.signal {
                    Some(value) => {
                        let units = value.units.parse::<f64>().unwrap_or_else(|e| {
                            info!("Ошибка парсинга units для длинной EMA: {}", e);
                            0.0
                        });
                        let nanos = value.nano as f64 / 1_000_000_000.0;
                        units + nanos
                    }
                    None => {
                        info!("Отсутствует значение signal для длинной EMA");
                        0.0
                    }
                }
            }
            None => {
                info!("Не удалось получить последний индикатор для длинной EMA");
                0.0
            }
        };

        info!(
            "Последние значения EMA - короткая: {:.6}, длинная: {:.6}",
            last_short, last_long
        );

        Ok(self.signal_generator.update(last_short, last_long))
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
} 