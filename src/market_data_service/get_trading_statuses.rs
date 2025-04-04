use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum TradingStatus {
    #[serde(rename = "SECURITY_TRADING_STATUS_UNSPECIFIED")]
    #[default]
    Unspecified,
    #[serde(rename = "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING")]
    NotAvailableForTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_OPENING_PERIOD")]
    OpeningPeriod,
    #[serde(rename = "SECURITY_TRADING_STATUS_CLOSING_PERIOD")]
    ClosingPeriod,
    #[serde(rename = "SECURITY_TRADING_STATUS_BREAK_IN_TRADING")]
    BreakInTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_NORMAL_TRADING")]
    NormalTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_CLOSING_AUCTION")]
    ClosingAuction,
    #[serde(rename = "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION")]
    DarkPoolAuction,
    #[serde(rename = "SECURITY_TRADING_STATUS_DISCRETE_AUCTION")]
    DiscreteAuction,
    #[serde(rename = "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD")]
    OpeningAuctionPeriod,
    #[serde(rename = "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE")]
    TradingAtClosingAuctionPrice,
    #[serde(rename = "SECURITY_TRADING_STATUS_SESSION_ASSIGNED")]
    SessionAssigned,
    #[serde(rename = "SECURITY_TRADING_STATUS_SESSION_CLOSE")]
    SessionClose,
    #[serde(rename = "SECURITY_TRADING_STATUS_SESSION_OPEN")]
    SessionOpen,
    #[serde(rename = "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING")]
    DealerNormalTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING")]
    DealerBreakInTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING")]
    DealerNotAvailableForTrading,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTradingStatusesRequest {
    #[serde(rename = "instrumentId")]
    pub instrument_id: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingStatusResponse {
    pub figi: String,
    #[serde(rename = "tradingStatus")]
    pub trading_status: Option<TradingStatus>,
    #[serde(rename = "limitOrderAvailableFlag")]
    pub limit_order_available_flag: bool,
    #[serde(rename = "marketOrderAvailableFlag")]
    pub market_order_available_flag: bool,
    #[serde(rename = "apiTradeAvailableFlag")]
    pub api_trade_available_flag: bool,
    #[serde(rename = "instrumentUid")]
    pub instrument_uid: String,
    #[serde(rename = "bestpriceOrderAvailableFlag")]
    pub bestprice_order_available_flag: bool,
    #[serde(rename = "onlyBestPrice")]
    pub only_best_price: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTradingStatusesResponse {
    #[serde(rename = "tradingStatuses")]
    pub trading_statuses: Vec<TradingStatusResponse>,
}

impl GetTradingStatusesResponse {
    pub async fn get_trading_statuses(
        client: &reqwest::Client,
        token: &str,
        instrument_ids: Vec<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatuses";

        let request = GetTradingStatusesRequest {
            instrument_id: instrument_ids,
        };

        info!("Sending GetTradingStatuses request: {:?}", request);
        debug!("Request URL: {}", url);

        let response = match client
            .post(url)
            .bearer_auth(token)
            .json(&request)
            .send()
            .await
        {
            Ok(resp) => {
                info!("Received response from server, status: {}", resp.status());
                resp
            }
            Err(e) => {
                error!("Error sending request: {}", e);
                return Err(e.into());
            }
        };

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|e| {
                error!("Unable to read error body: {}", e);
                "Unknown error".to_string()
            });

            error!("Request failed with status: {}", status);
            return Err(format!("Error API: {} - {}", status, error_text).into());
        }

        match response.json::<Self>().await {
            Ok(statuses_response) => {
                info!(
                    "Successfully received statuses for {} instruments",
                    statuses_response.trading_statuses.len()
                );
                Ok(statuses_response)
            }
            Err(e) => {
                error!("Error deserializing response: {}", e);
                Err(e.into())
            }
        }
    }

    /// Checks if the instrument is available for trading
    /// Returns true if:
    /// 1. Instrument found
    /// 2. API trading available (api_trade_available_flag)
    /// 3. Trading status NORMAL_TRADING
    pub fn _is_instrument_available(&self, instrument_uid: &str) -> bool {
        self.trading_statuses
            .iter()
            .find(|status| status.instrument_uid == instrument_uid)
            .map(|status| {
                status.api_trade_available_flag
                    && matches!(status.trading_status, Some(TradingStatus::NormalTrading))
            })
            .unwrap_or(false)
    }

    /// Gets full information about the instrument status
    pub fn _get_instrument_status(&self, instrument_uid: &str) -> Option<&TradingStatusResponse> {
        self.trading_statuses
            .iter()
            .find(|status| status.instrument_uid == instrument_uid)
    }

    /// Returns list of instrument UIDs that are available for trading
    pub fn get_available_instruments(&self) -> Vec<String> {
        self.trading_statuses
            .iter()
            .filter(|status| {
                status.api_trade_available_flag
                    && matches!(status.trading_status, Some(TradingStatus::NormalTrading))
            })
            .map(|status| status.instrument_uid.clone())
            .collect()
    }
}

// Example usage:
pub async fn _check_instruments_availability(
    client: &reqwest::Client,
    token: &str,
    instrument_ids: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response =
        GetTradingStatusesResponse::get_trading_statuses(client, token, instrument_ids).await?;

    for status in &response.trading_statuses {
        println!("Instrument {}: ", status.instrument_uid);
        println!(
            "  Available for trading: {}",
            response._is_instrument_available(&status.instrument_uid)
        );
        println!(
            "  API trading available: {}",
            status.api_trade_available_flag
        );
        println!(
            "  Limit orders available: {}",
            status.limit_order_available_flag
        );
        println!(
            "  Market orders available: {}",
            status.market_order_available_flag
        );
        println!("  Trading status: {:?}", status.trading_status);
        println!();
    }

    Ok(())
}
