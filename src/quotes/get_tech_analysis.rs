use crate::models::structs::Quotation;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use chrono;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum IndicatorType {
    #[serde(rename = "INDICATOR_TYPE_UNSPECIFIED")]
    #[default]
    Unspecified,
    #[serde(rename = "INDICATOR_TYPE_BB")]
    BB,
    #[serde(rename = "INDICATOR_TYPE_EMA")]
    EMA,
    #[serde(rename = "INDICATOR_TYPE_RSI")]
    RSI,
    #[serde(rename = "INDICATOR_TYPE_MACD")]
    MACD,
    #[serde(rename = "INDICATOR_TYPE_SMA")]
    SMA,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum IndicatorInterval {
    #[serde(rename = "INDICATOR_INTERVAL_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "INDICATOR_INTERVAL_ONE_MINUTE")]
    OneMinute,
    #[serde(rename = "INDICATOR_INTERVAL_2_MIN")]
    TwoMinutes,
    #[serde(rename = "INDICATOR_INTERVAL_3_MIN")]
    ThreeMinutes,
    #[serde(rename = "INDICATOR_INTERVAL_FIVE_MINUTES")]
    FiveMinutes,
    #[serde(rename = "INDICATOR_INTERVAL_10_MIN")]
    TenMinutes,
    #[serde(rename = "INDICATOR_INTERVAL_FIFTEEN_MINUTES")]
    FifteenMinutes,
    #[serde(rename = "INDICATOR_INTERVAL_30_MIN")]
    ThirtyMin,
    #[serde(rename = "INDICATOR_INTERVAL_ONE_HOUR")]
    Hour,
    #[serde(rename = "INDICATOR_INTERVAL_2_HOUR")]
    TwoHours,
    #[serde(rename = "INDICATOR_INTERVAL_4_HOUR")]
    FourHour,
    #[serde(rename = "INDICATOR_INTERVAL_ONE_DAY")]
    Day,
    #[serde(rename = "INDICATOR_INTERVAL_WEEK")]
    Week,
    #[serde(rename = "INDICATOR_INTERVAL_MONTH")]
    Month,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum TypeOfPrice {
    #[serde(rename = "TYPE_OF_PRICE_UNSPECIFIED")]
    #[default]
    Unspecified,
    #[serde(rename = "TYPE_OF_PRICE_CLOSE")]
    Close,
    #[serde(rename = "TYPE_OF_PRICE_OPEN")]
    Open,
    #[serde(rename = "TYPE_OF_PRICE_HIGH")]
    High,
    #[serde(rename = "TYPE_OF_PRICE_LOW")]
    Low,
    #[serde(rename = "TYPE_OF_PRICE_AVG")]
    Avg,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deviation {
    #[serde(rename = "deviationMultiplier")]
    pub deviation_multiplier: Quotation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Smoothing {
    #[serde(rename = "fastLength")]
    pub fast_length: i32,
    #[serde(rename = "slowLength")]
    pub slow_length: i32,
    #[serde(rename = "signalSmoothing")]
    pub signal_smoothing: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTechAnalysisRequest {
    #[serde(rename = "indicatorType")]
    pub indicator_type: IndicatorType,
    #[serde(rename = "instrumentUid")]
    pub instrument_uid: String,
    pub from: String,      // обязательное поле
    pub to: String,        // обязательное поле
    pub interval: IndicatorInterval,
    #[serde(rename = "typeOfPrice")]
    pub type_of_price: TypeOfPrice,
    pub length: i32,
    pub deviation: Option<Deviation>,
    pub smoothing: Option<Smoothing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TechnicalIndicator {
    pub timestamp: String,
    #[serde(rename = "middleBand")]
    pub middle_band: Option<Quotation>,
    #[serde(rename = "upperBand")]
    pub upper_band: Option<Quotation>,
    #[serde(rename = "lowerBand")]
    pub lower_band: Option<Quotation>,
    pub signal: Option<Quotation>,
    pub macd: Option<Quotation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTechAnalysisResponse {
    #[serde(rename = "technicalIndicators")]
    pub technical_indicators: Vec<TechnicalIndicator>,
}

impl GetTechAnalysisRequest {
    pub fn new(
        indicator_type: IndicatorType,
        instrument_uid: String,
        from: String,
        to: String,
        interval: IndicatorInterval,
        type_of_price: TypeOfPrice,
        length: i32,
        deviation: Option<Deviation>,
        smoothing: Option<Smoothing>,
    ) -> Self {
        Self {
            indicator_type,
            instrument_uid,
            from,
            to,
            interval,
            type_of_price,
            length,
            deviation,
            smoothing,
        }
    }

    /// Создает запрос для получения EMA с автоматическим расчетом временного диапазона
    pub fn new_ema_auto_period(
        instrument_uid: &str,
        interval: IndicatorInterval,
        type_of_price: TypeOfPrice,
        length: i32,
    ) -> Self {
        let now = chrono::Utc::now();
        
        // Для EMA нам нужно как минимум 2.5 * length точек для формирования индикатора
        // Добавляем еще 50% для отображения трендов
        let required_points = (length as f64 * 3.75).ceil() as i32;
        
        // Минимальное количество дней для разных интервалов с учетом required_points
        let days = match interval {
            IndicatorInterval::OneMinute => {
                let points_per_day = 24 * 60; // точек в день
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(1)
            },
            IndicatorInterval::TwoMinutes => {
                let points_per_day = 24 * 30; // точек в день
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(1)
            },
            IndicatorInterval::ThreeMinutes => {
                let points_per_day = 24 * 20;
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(1)
            },
            IndicatorInterval::FiveMinutes => {
                let points_per_day = 24 * 12;
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(1)
            },
            IndicatorInterval::TenMinutes => {
                let points_per_day = 24 * 6;
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(1)
            },
            IndicatorInterval::FifteenMinutes => {
                let points_per_day = 24 * 4;
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(1)
            },
            IndicatorInterval::ThirtyMin => {
                let points_per_day = 24 * 2;
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(2)
            },
            IndicatorInterval::Hour => {
                let points_per_day = 24;
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(3)
            },
            IndicatorInterval::TwoHours => {
                let points_per_day = 12;
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(4)
            },
            IndicatorInterval::FourHour => {
                let points_per_day = 6;
                ((required_points as f64 / points_per_day as f64).ceil() as i64).max(8)
            },
            IndicatorInterval::Day => required_points.max(30) as i64,
            IndicatorInterval::Week => (required_points * 7).max(90) as i64,
            IndicatorInterval::Month => (required_points * 30).max(180) as i64,
            IndicatorInterval::Unspecified => required_points.max(30) as i64,
        };

        // Устанавливаем начало периода на начало дня
        let from = (now - chrono::Duration::days(days))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        
        Self::new(
            IndicatorType::EMA,
            instrument_uid.to_string(),
            from.to_rfc3339(),
            now.to_rfc3339(),
            interval,
            type_of_price,
            length,
            None,
            None,
        )
    }

    fn _calculate_required_hours(&self, required_points: i32) -> i64 {
        match self.interval {
            IndicatorInterval::OneMinute => {
                (required_points as f64 * 1.0 / 60.0).ceil() as i64
            }
            IndicatorInterval::TwoMinutes => {
                (required_points as f64 * 2.0 / 60.0).ceil() as i64
            }
            IndicatorInterval::ThreeMinutes => {
                (required_points as f64 * 3.0 / 60.0).ceil() as i64
            }
            IndicatorInterval::FiveMinutes => {
                (required_points as f64 * 5.0 / 60.0).ceil() as i64
            }
            IndicatorInterval::TenMinutes => {
                (required_points as f64 * 10.0 / 60.0).ceil() as i64
            }
            IndicatorInterval::FifteenMinutes => {
                (required_points as f64 * 15.0 / 60.0).ceil() as i64
            }
            IndicatorInterval::ThirtyMin => {
                (required_points as f64 * 30.0 / 60.0).ceil() as i64
            }
            IndicatorInterval::Hour => {
                required_points as i64
            }
            IndicatorInterval::TwoHours => {
                required_points as i64 * 2
            }
            IndicatorInterval::FourHour => {
                required_points as i64 * 4
            }
            IndicatorInterval::Day => required_points.max(30) as i64 * 24,
            IndicatorInterval::Week => required_points.max(30) as i64 * 24 * 7,
            IndicatorInterval::Month => required_points.max(30) as i64 * 24 * 30,
            IndicatorInterval::Unspecified => required_points.max(30) as i64 * 24,
        }
    }
}

impl GetTechAnalysisResponse {
    pub async fn get_tech_analysis(
        client: &reqwest::Client,
        token: &str,
        request: GetTechAnalysisRequest,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTechAnalysis";

        info!("Отправка запроса GetTechAnalysis: {:?}", request);
        debug!("URL запроса: {}", url);

        let response = match client
            .post(url)
            .bearer_auth(token)
            .json(&request)
            .send()
            .await
        {
            Ok(resp) => {
                info!("Получен ответ от сервера, статус: {}", resp.status());
                resp
            }
            Err(e) => {
                error!("Ошибка отправки запроса: {}", e);
                return Err(e.into());
            }
        };

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|e| {
                error!("Не удалось прочитать тело ошибки: {}", e);
                "Неизвестная ошибка".to_string()
            });

            error!("Ошибка запроса: статус {}, текст: {}", status, error_text);
            return Err(format!("Ошибка API: {} - {}", status, error_text).into());
        }

        match response.json::<Self>().await {
            Ok(tech_analysis_response) => {
                info!(
                    "Успешно получены технические индикаторы для {} временных точек",
                    tech_analysis_response.technical_indicators.len()
                );
                Ok(tech_analysis_response)
            }
            Err(e) => {
                error!("Ошибка десериализации ответа: {}", e);
                Err(e.into())
            }
        }
    }

    /// Отладочный метод для просмотра всех данных индикатора
    pub fn _debug_print_indicator(&self) {
        for (i, indicator) in self.technical_indicators.iter().enumerate() {
            println!("Индикатор #{}", i + 1);
            println!("  Timestamp: {}", indicator.timestamp);
            println!("  Middle Band: {:?}", indicator.middle_band);
            println!("  Upper Band: {:?}", indicator.upper_band);
            println!("  Lower Band: {:?}", indicator.lower_band);
            println!("  Signal: {:?}", indicator.signal);
            println!("  MACD: {:?}", indicator.macd);
            println!("---");
        }
    }

    /// Выводит значения EMA для каждого временного интервала
    pub fn _print_ema_values(&self) {
        if self.technical_indicators.is_empty() {
            println!("Нет данных для отображения");
            return;
        }

        println!("Всего точек данных: {}", self.technical_indicators.len());
        
        // Добавляем отладочный вывод первой точки
        if let Some(first) = self.technical_indicators.first() {
            println!("Отладка первой точки:");
            println!("  middle_band: {:?}", first.middle_band);
            println!("  upper_band: {:?}", first.upper_band);
            println!("  lower_band: {:?}", first.lower_band);
            println!("  signal: {:?}", first.signal);
            println!("  macd: {:?}", first.macd);
            println!("---");
        }

        for indicator in &self.technical_indicators {
            // Пробуем искать EMA в разных полях
            let ema_value = indicator.middle_band.as_ref()
                .or(indicator.signal.as_ref())  // пробуем signal если middle_band пусто
                .or(indicator.macd.as_ref());   // пробуем macd если signal пусто

            match ema_value {
                Some(ema) => {
                    println!(
                        "Время: {}, EMA: {}.{:09}",
                        indicator.timestamp,
                        ema.units,
                        ema.nano.abs()
                    );
                }
                None => {
                    println!("Время: {}, EMA: нет данных", indicator.timestamp);
                }
            }
        }
    }
}
