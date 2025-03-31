use crate::quotes::get_tech_analysis::{GetTechAnalysisRequest, GetTechAnalysisResponse, IndicatorInterval, TypeOfPrice};

#[derive(Debug, Clone)]
pub enum TradeSignal {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug)]
pub struct EmaCrossStrategy {
    pub instrument_uid: String,
    pub short_ema_length: i32,
    pub long_ema_length: i32,
    pub interval: IndicatorInterval,
}

impl EmaCrossStrategy {
    pub fn new(
        instrument_uid: String,
        short_ema_length: i32,
        long_ema_length: i32,
        interval: IndicatorInterval,
    ) -> Self {
        Self {
            instrument_uid,
            short_ema_length,
            long_ema_length,
            interval,
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
        &self,
        client: &reqwest::Client,
        token: &str,
    ) -> Result<TradeSignal, Box<dyn std::error::Error>> {
        let short_ema = self.get_short_ema(client, token).await?;
        let long_ema = self.get_long_ema(client, token).await?;

        Ok(self.analyze_crossover(&short_ema, &long_ema))
    }
} 