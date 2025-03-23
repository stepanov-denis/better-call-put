use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum InstrumentIdType {
    #[serde(rename = "INSTRUMENT_ID_UNSPECIFIED")]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Quotation {
    pub units: String,
    pub nano: i32,
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

#[derive(Serialize, Deserialize, Debug)]
struct Instrument {
    #[serde(default)]
    figi: Option<String>,
    #[serde(default)]
    ticker: Option<String>,
    #[serde(rename = "classCode", default)]
    class_code: Option<String>,
    #[serde(default)]
    isin: Option<String>,
    #[serde(default)]
    lot: Option<i32>,
    #[serde(default)]
    currency: Option<String>,
    #[serde(default)]
    klong: Option<Quotation>,
    #[serde(default)]
    kshort: Option<Quotation>,
    #[serde(default)]
    dlong: Option<Quotation>,
    #[serde(default)]
    dshort: Option<Quotation>,
    #[serde(rename = "dlongMin", default)]
    dlong_min: Option<Quotation>,
    #[serde(rename = "dshortMin", default)]
    dshort_min: Option<Quotation>,
    #[serde(rename = "shortEnabledFlag", default)]
    short_enabled_flag: Option<bool>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    exchange: Option<String>,
    #[serde(rename = "countryOfRisk", default)]
    country_of_risk: Option<String>,
    #[serde(rename = "countryOfRiskName", default)]
    country_of_risk_name: Option<String>,
    #[serde(rename = "instrumentType", default)]
    instrument_type: Option<String>,
    #[serde(rename = "tradingStatus", default)]
    trading_status: Option<String>,
    #[serde(rename = "otcFlag", default)]
    otc_flag: Option<bool>,
    #[serde(rename = "buyAvailableFlag", default)]
    buy_available_flag: Option<bool>,
    #[serde(rename = "sellAvailableFlag", default)]
    sell_available_flag: Option<bool>,
    #[serde(rename = "minPriceIncrement", default)]
    min_price_increment: Option<Quotation>,
    #[serde(rename = "apiTradeAvailableFlag", default)]
    api_trade_available_flag: Option<bool>,
    #[serde(default)]
    uid: Option<String>,
    #[serde(rename = "realExchange", default)]
    real_exchange: Option<String>,
    #[serde(rename = "positionUid", default)]
    position_uid: Option<String>,
    #[serde(rename = "assetUid", default)]
    asset_uid: Option<String>,
    #[serde(rename = "forIisFlag", default)]
    for_iis_flag: Option<bool>,
    #[serde(rename = "forQualInvestorFlag", default)]
    for_qual_investor_flag: Option<bool>,
    #[serde(rename = "weekendFlag", default)]
    weekend_flag: Option<bool>,
    #[serde(rename = "blockedTcaFlag", default)]
    blocked_tca_flag: Option<bool>,
    #[serde(rename = "instrumentKind", default)]
    instrument_kind: Option<String>,
    #[serde(rename = "first1minCandleDate", default)]
    first_1min_candle_date: Option<String>,
    #[serde(rename = "first1dayCandleDate", default)]
    first_1day_candle_date: Option<String>,
    #[serde(default)]
    brand: Option<BrandData>,
    #[serde(rename = "dlongClient", default)]
    dlong_client: Option<Quotation>,
    #[serde(rename = "dshortClient", default)]
    dshort_client: Option<Quotation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentResponse {
    pub instrument: Instrument,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInstrumentByRequest {
    #[serde(rename = "idType")]
    pub id_type: InstrumentIdType,
    pub id: String,
    #[serde(rename = "classCode")]
    pub class_code: Option<String>,
}

impl InstrumentResponse {
    pub async fn get_instrument_info(
        client: &reqwest::Client,
        id_type: InstrumentIdType,
        id: String,
        api_token: &str,
        class_code: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let instrument_url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetInstrumentBy";

        let request_body = GetInstrumentByRequest {
            id_type,
            id,
            class_code,
        };

        let response = client
            .post(instrument_url)
            .bearer_auth(api_token)
            .json(&request_body)
            .send()
            .await?;

        // Проверяем статус ответа
        if !response.status().is_success() {
            println!("Ошибка запроса: {}", response.status());
            let error_text = response.text().await?;
            println!("Текст ошибки: {}", error_text);
            return Err("Ошибка запроса к API".into());
        }

        let instrument_response: InstrumentResponse = response.json().await?;

        Ok(instrument_response)
    }
}
