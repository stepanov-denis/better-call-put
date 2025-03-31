use crate::models::enums::InstrumentType;
use crate::models::structs::Quotation;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum InstrumentIdType {
    #[serde(rename = "INSTRUMENT_ID_UNSPECIFIED")]
    #[default]
    Unspecified,
    #[serde(rename = "INSTRUMENT_ID_TYPE_FIGI")]
    Figi,
    #[serde(rename = "INSTRUMENT_ID_TYPE_TICKER")]
    Ticker,
    #[serde(rename = "INSTRUMENT_ID_TYPE_UID")]
    Uid,
    #[serde(rename = "INSTRUMENT_ID_TYPE_POSITION_UID")]
    PositionUid,
}

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

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum RealExchange {
    #[serde(rename = "REAL_EXCHANGE_UNSPECIFIED")]
    #[default]
    Unspecified,
    #[serde(rename = "REAL_EXCHANGE_MOEX")]
    Moex,
    #[serde(rename = "REAL_EXCHANGE_RTS")]
    Rts,
    #[serde(rename = "REAL_EXCHANGE_OTC")]
    Otc,
    #[serde(rename = "REAL_EXCHANGE_DEALER")]
    Dealer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrandData {
    #[serde(rename = "logoName")]
    pub logo_name: String,
    #[serde(rename = "logoBaseColor")]
    pub logo_base_color: String,
    #[serde(rename = "textColor")]
    pub text_color: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub figi: Option<String>,
    pub ticker: Option<String>,
    pub class_code: Option<String>,
    pub isin: Option<String>,
    pub lot: Option<i32>,
    pub currency: Option<String>,
    pub klong: Option<Quotation>,
    pub kshort: Option<Quotation>,
    pub dlong: Option<Quotation>,
    pub dshort: Option<Quotation>,
    pub dlong_min: Option<Quotation>,
    pub dshort_min: Option<Quotation>,
    pub short_enabled_flag: Option<bool>,
    pub name: Option<String>,
    pub exchange: Option<String>,
    pub country_of_risk: Option<String>,
    pub country_of_risk_name: Option<String>,
    pub instrument_type: Option<String>,
    pub trading_status: Option<TradingStatus>,
    pub otc_flag: Option<bool>,
    pub buy_available_flag: Option<bool>,
    pub sell_available_flag: Option<bool>,
    pub min_price_increment: Option<Quotation>,
    pub api_trade_available_flag: Option<bool>,
    pub uid: Option<String>,
    pub real_exchange: Option<RealExchange>,
    pub position_uid: Option<String>,
    pub asset_uid: Option<String>,
    pub for_iis_flag: Option<bool>,
    pub for_qual_investor_flag: Option<bool>,
    pub weekend_flag: Option<bool>,
    pub blocked_tca_flag: Option<bool>,
    pub instrument_kind: Option<InstrumentType>,
    pub first_1min_candle_date: Option<String>,
    pub first_1day_candle_date: Option<String>,
    pub brand: Option<BrandData>,
    pub dlong_client: Option<Quotation>,
    pub dshort_client: Option<Quotation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInstrumentByRequest {
    #[serde(rename = "idType")]
    pub id_type: InstrumentIdType,
    #[serde(rename = "classCode")]
    pub class_code: Option<String>,
    pub id: String,
}

impl GetInstrumentByRequest {
    pub fn new(id_type: InstrumentIdType, class_code: Option<String>, id: String) -> Self {
        Self {
            id_type,
            class_code,
            id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentResponse {
    pub instrument: Instrument,
}

impl InstrumentResponse {
    pub async fn get_instrument_by(
        client: &reqwest::Client,
        api_token: &str,
        request: GetInstrumentByRequest,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let instrument_url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetInstrumentBy";

        info!("send request: {:?}", request);

        let response = client
            .post(instrument_url)
            .bearer_auth(api_token)
            .json(&request)
            .send()
            .await?;

        // Проверяем статус ответа
        if !response.status().is_success() {
            error!("request error: {}", response.status());
            let error_text = response.text().await?;
            error!("text of error: {}", error_text);
            return Err("api request error".into());
        }

        let instrument_response: InstrumentResponse = response.json().await?;

        Ok(instrument_response)
    }
}
