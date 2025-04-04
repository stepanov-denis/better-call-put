use crate::models::structs::Quotation;
use serde::{Deserialize, Serialize};
use std::fmt;
use tracing;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum LastPriceType {
    #[serde(rename = "LAST_PRICE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "LAST_PRICE_EXCHANGE")]
    Exchange,
    #[serde(rename = "LAST_PRICE_DEALER")]
    Dealer,
}

impl fmt::Display for LastPriceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LastPriceType::Unspecified => write!(f, "Unspecified"),
            LastPriceType::Exchange => write!(f, "Exchange"),
            LastPriceType::Dealer => write!(f, "Dealer"),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum InstrumentStatus {
    #[serde(rename = "INSTRUMENT_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "INSTRUMENT_STATUS_BASE")]
    Base,
    #[serde(rename = "INSTRUMENT_STATUS_ALL")]
    All,
}

#[derive(Debug, Serialize)]
pub struct GetLastPricesRequest {
    #[serde(rename = "instrumentId")]
    pub instrument_id: Vec<String>,
    #[serde(rename = "lastPriceType")]
    pub last_price_type: LastPriceType,
    #[serde(rename = "instrumentStatus")]
    pub instrument_status: InstrumentStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastPrice {
    pub figi: String,
    pub price: Quotation,
    pub time: String,
    #[serde(rename = "instrumentUid")]
    pub instrument_uid: String,
    #[serde(rename = "lastPriceType")]
    pub last_price_type: LastPriceType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLastPricesResponse {
    #[serde(rename = "lastPrices")]
    pub last_prices: Vec<LastPrice>,
}

impl GetLastPricesRequest {
    pub fn new(
        instrument_id: Vec<String>,
        last_price_type: LastPriceType,
        instrument_status: InstrumentStatus,
    ) -> Self {
        Self {
            instrument_id,
            last_price_type,
            instrument_status,
        }
    }
}

impl GetLastPricesResponse {
    pub async fn get_last_prices(
        client: &reqwest::Client,
        token: &str,
        request: GetLastPricesRequest,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.MarketDataService/GetLastPrices";
        
        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        tracing::info!("Received response from server, status: {}", response.status());

        if !response.status().is_success() {
            let error_text = response.text().await?;
            tracing::error!("Server returned error: {}", error_text);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Server error: {}", error_text)
            )));
        }

        let response_text = response.text().await?;
        let response: GetLastPricesResponse = serde_json::from_str(&response_text)?;

        // Логируем информацию о полученных ценах в более читаемом формате
        tracing::info!("Received last prices for {} instruments", response.last_prices.len());
        for price in &response.last_prices {
            let price_value = price.price.units.parse::<f64>().unwrap_or(0.0) + 
                            (price.price.nano as f64 / 1_000_000_000.0);
            tracing::info!(
                "Instrument: {} ({}), Price: {:.3}, Time: {}, Type: {}",
                price.figi,
                price.instrument_uid,
                price_value,
                price.time,
                price.last_price_type
            );
        }

        Ok(response)
    }
} 