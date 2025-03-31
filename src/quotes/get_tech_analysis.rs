use crate::models::structs::Quotation;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use chrono::{DateTime, Utc, Timelike, Datelike};

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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum IndicatorInterval {
    #[serde(rename = "INDICATOR_INTERVAL_UNSPECIFIED")]
    #[default]
    Unspecified,
    #[serde(rename = "INDICATOR_INTERVAL_ONE_MINUTE")]
    OneMinute,
    #[serde(rename = "INDICATOR_INTERVAL_FIVE_MINUTES")]
    FiveMinutes,
    #[serde(rename = "INDICATOR_INTERVAL_FIFTEEN_MINUTES")]
    FifteenMinutes,
    #[serde(rename = "INDICATOR_INTERVAL_ONE_HOUR")]
    OneHour,
    #[serde(rename = "INDICATOR_INTERVAL_ONE_DAY")]
    OneDay,
    #[serde(rename = "INDICATOR_INTERVAL_2_MIN")]
    TwoMin,
    #[serde(rename = "INDICATOR_INTERVAL_3_MIN")]
    ThreeMin,
    #[serde(rename = "INDICATOR_INTERVAL_10_MIN")]
    TenMin,
    #[serde(rename = "INDICATOR_INTERVAL_30_MIN")]
    ThirtyMin,
    #[serde(rename = "INDICATOR_INTERVAL_2_HOUR")]
    TwoHour,
    #[serde(rename = "INDICATOR_INTERVAL_4_HOUR")]
    FourHour,
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

    /// Создает запрос для получения EMA с указанным временным диапазоном в днях
    pub fn new_ema_with_days_back(
        instrument_uid: &str,
        interval: IndicatorInterval,
        type_of_price: TypeOfPrice,
        length: i32,
        days_back: i32,
    ) -> Self {
        let now = chrono::Utc::now();
        let from = now - chrono::Duration::days(days_back as i64);
        
        Self::new(
            IndicatorType::EMA,
            instrument_uid.to_string(),
            from.to_rfc3339(),    // передаем String напрямую, без Some()
            now.to_rfc3339(),     // передаем String напрямую, без Some()
            interval,
            type_of_price,
            length,
            None,
            None,
        )
    }

    /// Создает запрос для получения EMA с пустым временным диапазоном
    pub fn new_ema_empty_dates(
        instrument_uid: &str,
        interval: IndicatorInterval,
        type_of_price: TypeOfPrice,
        length: i32,
    ) -> Self {
        let now = chrono::Utc::now();
        let from = now - chrono::Duration::hours(1); // минимальный период - 1 час
        
        Self::new(
            IndicatorType::EMA,
            instrument_uid.to_string(),
            from.to_rfc3339(),    // передаем String напрямую, без Some()
            now.to_rfc3339(),     // передаем String напрямую, без Some()
            interval,
            type_of_price,
            length,
            None,
            None,
        )
    }

    /// Создает запрос для получения EMA с минимальным временным диапазоном
    pub fn new_ema_minimal(
        instrument_uid: &str,
        interval: IndicatorInterval,
        type_of_price: TypeOfPrice,
        length: i32,
    ) -> Self {
        let now = chrono::Utc::now();
        let from = now - chrono::Duration::days(1);
        
        Self::new(
            IndicatorType::EMA,
            instrument_uid.to_string(),
            from.to_rfc3339(),    // передаем String напрямую, без Some()
            now.to_rfc3339(),     // передаем String напрямую, без Some()
            interval,
            type_of_price,
            length,
            None,
            None,
        )
    }

    /// Создает запрос на получение технических индикаторов с автоматическим определением временных меток
    /// для текущего дня с учетом интервала
    pub fn new_for_current_day(
        indicator_type: IndicatorType,
        instrument_uid: String,
        interval: IndicatorInterval,
        type_of_price: TypeOfPrice,
        length: i32,
        deviation: Option<Deviation>,
        smoothing: Option<Smoothing>,
    ) -> Self {
        let now = chrono::Utc::now();
        let (from, to) = Self::get_time_range_for_interval(now, &interval);

        Self::new(
            indicator_type,
            instrument_uid,
            from.to_rfc3339(),
            to.to_rfc3339(),
            interval,
            type_of_price,
            length,
            deviation,
            smoothing,
        )
    }

    /// Определяет временной диапазон для заданного интервала
    fn get_time_range_for_interval(now: DateTime<Utc>, interval: &IndicatorInterval) -> (DateTime<Utc>, DateTime<Utc>) {
        let start_of_day = now
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        match interval {
            IndicatorInterval::OneMinute | IndicatorInterval::TwoMin | IndicatorInterval::ThreeMin |
            IndicatorInterval::FiveMinutes | IndicatorInterval::TenMin | IndicatorInterval::FifteenMinutes |
            IndicatorInterval::ThirtyMin => {
                // Для минутных интервалов берем текущий час
                let hour_start = now.with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();
                let hour_end = hour_start + chrono::Duration::hours(1) - chrono::Duration::seconds(1);
                (hour_start, hour_end)
            },
            IndicatorInterval::OneHour | IndicatorInterval::TwoHour | IndicatorInterval::FourHour => {
                // Для часовых интервалов берем текущий день
                let day_end = start_of_day + chrono::Duration::days(1) - chrono::Duration::seconds(1);
                (start_of_day, day_end)
            },
            IndicatorInterval::OneDay => {
                // Для дневного интервала берем текущий день
                let day_end = start_of_day + chrono::Duration::days(1) - chrono::Duration::seconds(1);
                (start_of_day, day_end)
            },
            IndicatorInterval::Week => {
                // Для недельного интервала берем текущую неделю
                let week_start = start_of_day - chrono::Duration::days(start_of_day.weekday().num_days_from_monday() as i64);
                let week_end = week_start + chrono::Duration::days(7) - chrono::Duration::seconds(1);
                (week_start, week_end)
            },
            IndicatorInterval::Month => {
                // Для месячного интервала берем текущий месяц
                let month_start = start_of_day.with_day(1).unwrap();
                let next_month = if month_start.month() == 12 {
                    month_start.with_year(month_start.year() + 1).unwrap().with_month(1).unwrap()
                } else {
                    month_start.with_month(month_start.month() + 1).unwrap()
                };
                let month_end = next_month - chrono::Duration::seconds(1);
                (month_start, month_end)
            },
            IndicatorInterval::Unspecified => {
                // По умолчанию берем текущий день
                let day_end = start_of_day + chrono::Duration::days(1) - chrono::Duration::seconds(1);
                (start_of_day, day_end)
            }
        }
    }

    /// Создает запрос для получения EMA с заданными параметрами
    pub fn new_ema(
        instrument_uid: &str,
        interval: IndicatorInterval,
        type_of_price: TypeOfPrice,
        length: i32,
    ) -> Self {
        Self::new_for_current_day(
            IndicatorType::EMA,
            instrument_uid.to_string(),
            interval,
            type_of_price,
            length,
            None,    // deviation не нужен для EMA
            None,    // smoothing не нужен для EMA
        )
    }

    /// Создает запрос для получения последних N значений EMA с заданным интервалом
    pub fn new_ema_last_n(
        instrument_uid: &str,
        interval: IndicatorInterval,
        type_of_price: TypeOfPrice,
        length: i32,
        count: i32,  // количество значений, которые хотим получить
    ) -> Self {
        let now = chrono::Utc::now();
        
        // Вычисляем начальную дату в зависимости от интервала и количества значений
        let hours_back = match interval {
            IndicatorInterval::FourHour => 4 * count,
            IndicatorInterval::TwoHour => 2 * count,
            IndicatorInterval::OneHour => count,
            // ... другие интервалы ...
            _ => 24 * count, // для дневных и более крупных интервалов
        };
        
        let from = now - chrono::Duration::hours(hours_back as i64);
        
        Self::new(
            IndicatorType::EMA,
            instrument_uid.to_string(),
            from.to_rfc3339(),    // передаем String напрямую, без Some()
            now.to_rfc3339(),     // передаем String напрямую, без Some()
            interval,
            type_of_price,
            length,
            None,
            None,
        )
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

    /// Возвращает значения EMA для каждого временного интервала
    /// Возвращает вектор кортежей (timestamp, ema_value)
    pub fn get_ema(&self) -> Vec<(String, Quotation)> {
        self.technical_indicators
            .iter()
            .filter_map(|indicator| {
                // Для EMA используем middle_band как значение EMA
                indicator.middle_band.as_ref().map(|ema| {
                    (indicator.timestamp.clone(), (*ema).clone())
                })
            })
            .collect()
    }

    /// Выводит значения EMA для каждого временного интервала
    pub fn print_ema_values(&self) {
        if self.technical_indicators.is_empty() {
            println!("Нет данных для отображения");
            return;
        }

        println!("Всего точек данных: {}", self.technical_indicators.len());
        
        for indicator in &self.technical_indicators {
            // Для EMA значение находится в поле middle_band
            if let Some(ema) = &indicator.middle_band {
                println!(
                    "Время: {}, EMA: {}.{:09}",
                    indicator.timestamp,
                    ema.units,
                    ema.nano.abs()
                );
            } else {
                println!("Время: {}, EMA: нет данных", indicator.timestamp);
            }
        }
    }

    /// Отладочный метод для просмотра всех данных индикатора
    pub fn debug_print_indicator(&self) {
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
}
