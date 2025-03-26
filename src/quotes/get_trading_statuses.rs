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

        info!("Отправка запроса GetTradingStatuses: {:?}", request);
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
            Ok(statuses_response) => {
                info!(
                    "Успешно получены статусы для {} инструментов",
                    statuses_response.trading_statuses.len()
                );
                Ok(statuses_response)
            }
            Err(e) => {
                error!("Ошибка десериализации ответа: {}", e);
                Err(e.into())
            }
        }
    }

    /// Проверяет доступность инструмента для торговли
    /// Возвращает true если:
    /// 1. Инструмент найден
    /// 2. API торговля доступна (api_trade_available_flag)
    /// 3. Торговый статус NORMAL_TRADING
    pub fn is_instrument_available(&self, instrument_uid: &str) -> bool {
        self.trading_statuses
            .iter()
            .find(|status| status.instrument_uid == instrument_uid)
            .map(|status| {
                status.api_trade_available_flag
                    && matches!(status.trading_status, Some(TradingStatus::NormalTrading))
            })
            .unwrap_or(false)
    }

    /// Получает полную информацию о статусе инструмента
    pub fn get_instrument_status(&self, instrument_uid: &str) -> Option<&TradingStatusResponse> {
        self.trading_statuses
            .iter()
            .find(|status| status.instrument_uid == instrument_uid)
    }
}

// Пример использования:
pub async fn check_instruments_availability(
    client: &reqwest::Client,
    token: &str,
    instrument_ids: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response =
        GetTradingStatusesResponse::get_trading_statuses(client, token, instrument_ids).await?;

    for status in &response.trading_statuses {
        println!("Инструмент {}: ", status.instrument_uid);
        println!(
            "  Доступен для торговли: {}",
            response.is_instrument_available(&status.instrument_uid)
        );
        println!(
            "  Доступен для API торговли: {}",
            status.api_trade_available_flag
        );
        println!(
            "  Доступны лимитные заявки: {}",
            status.limit_order_available_flag
        );
        println!(
            "  Доступны рыночные заявки: {}",
            status.market_order_available_flag
        );
        println!("  Торговый статус: {:?}", status.trading_status);
        println!();
    }

    Ok(())
}
