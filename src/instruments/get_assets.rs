use crate::models::enums::InstrumentType;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tracing::{info, error, debug};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum InstrumentStatus {
    #[serde(rename = "INSTRUMENT_STATUS_UNSPECIFIED")]
    #[default]
    Unspecified,
    #[serde(rename = "INSTRUMENT_STATUS_BASE")]
    Base,
    #[serde(rename = "INSTRUMENT_STATUS_ALL")]
    All,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum AssetType {
    #[serde(rename = "ASSET_TYPE_UNSPECIFIED")]
    #[default]
    Unspecified,
    #[serde(rename = "ASSET_TYPE_CURRENCY")]
    Currency,
    #[serde(rename = "ASSET_TYPE_COMMODITY")]
    Commodity,
    #[serde(rename = "ASSET_TYPE_INDEX")]
    Index,
    #[serde(rename = "ASSET_TYPE_SECURITY")]
    Security,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,
    #[serde(rename = "instrumentUid")]
    pub instrument_uid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instrument {
    pub uid: String,
    pub figi: String,
    #[serde(rename = "instrumentType")]
    pub instrument_type: String,
    pub ticker: String,
    #[serde(rename = "classCode")]
    pub class_code: String,
    pub links: Vec<Link>,
    #[serde(rename = "instrumentKind")]
    instrument_kind: InstrumentType,
    #[serde(rename = "positionUid")]
    pub position_uid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub uid: String,
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    pub name: String,
    pub instruments: Vec<Instrument>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAssetsRequest {
    #[serde(rename = "instrumentType")]
    pub instrument_type: InstrumentType,
    #[serde(rename = "instrumentStatus")]
    pub instrument_status: InstrumentStatus,
}

// Добавим реализацию Display для лучшего логирования
impl std::fmt::Display for GetAssetsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ instrumentType: {:?}, instrumentStatus: {:?} }}",
            self.instrument_type, self.instrument_status
        )
    }
}

impl GetAssetsRequest {
    pub fn new(instrument_type: InstrumentType, instrument_status: InstrumentStatus) -> Self {
        Self {
            instrument_type,
            instrument_status,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAssetsResponse {
    pub assets: Vec<Asset>,
}

impl GetAssetsResponse {
    pub async fn get_assets(
        client: &reqwest::Client,
        token: &str,
        request: GetAssetsRequest,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssets";
    
        info!("Отправка запроса GetAssets: {:?}", request);
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
    
        // Проверяем статус ответа
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|e| {
                error!("Не удалось прочитать тело ошибки: {}", e);
                "Неизвестная ошибка".to_string()
            });
            
            error!(
                "Ошибка запроса: статус {}, текст: {}",
                status,
                error_text
            );
            return Err(format!("Ошибка API: {} - {}", status, error_text).into());
        }
    
        // Пытаемся десериализовать JSON
        match response.json::<Self>().await {
            Ok(assets_response) => {
                info!("Успешно десериализован ответ с {} активами", assets_response.assets.len());
                Ok(assets_response)
            }
            Err(e) => {
                error!("Ошибка десериализации ответа: {}", e);
                Err(e.into())
            }
        }
    }
}
