use crate::models::enums::InstrumentType;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use reqwest::Client;
use std::error::Error;
use serde_json;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum InstrumentStatus {
    #[serde(rename = "INSTRUMENT_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "INSTRUMENT_STATUS_BASE")]
    Base,
    #[serde(rename = "INSTRUMENT_STATUS_ALL")]
    All,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetType {
    #[serde(rename = "ASSET_TYPE_UNSPECIFIED")]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,
    #[serde(rename = "instrumentUid")]
    pub instrument_uid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAssetsResponse {
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Asset {
    pub uid: String,
    #[serde(rename = "type")]
    pub asset_type: String,
    pub name: String,
    pub instruments: Vec<Instrument>,
}

#[derive(Debug, Clone, Deserialize)]
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
    pub instrument_kind: String,
    #[serde(rename = "positionUid")]
    pub position_uid: String,
}

pub trait IntoUid {
    fn into_uids(&self) -> Vec<String>;
}

impl IntoUid for Vec<String> {
    fn into_uids(&self) -> Vec<String> {
        self.clone()
    }
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

impl GetAssetsResponse {
    pub async fn get_assets(
        client: &Client,
        token: &str,
        request: GetAssetsRequest,
    ) -> Result<Self, Box<dyn Error>> {
        let url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssets";
        
        info!("Отправка запроса GetAssets: {:?}", request);
        debug!("URL запроса: {}", url);

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await?;

        info!("Получен ответ от сервера, статус: {}", response.status());
        
        let response_text = response.text().await?;
        debug!("Response body: {}", response_text);
        
        match serde_json::from_str::<GetAssetsResponse>(&response_text) {
            Ok(assets_response) => Ok(assets_response),
            Err(e) => {
                error!("Ошибка десериализации ответа. Детали ошибки: {}", e);
                error!("Полученный JSON: {}", response_text);
                Err(Box::new(e))
            }
        }
    }

    /// Фильтрует инструменты по заданным параметрам
    pub async fn filter_instruments(
        &self,
        class_code: &str,
        instrument_type: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let filtered = self.assets.iter()
            .flat_map(|asset| asset.instruments.iter())
            .filter(|instrument| {
                instrument.class_code == class_code &&
                instrument.instrument_type == instrument_type
            })
            .map(|instrument| instrument.uid.clone())
            .collect();

        Ok(filtered)
    }

    /// Выводит информацию об инструментах в виде таблицы
    pub fn _print_instruments(&self) {
        let mut all_instruments = Vec::new();
        for asset in &self.assets {
            all_instruments.extend(asset.instruments.iter());
        }

        if all_instruments.is_empty() {
            info!("there are no tools to display");
            return;
        }

        // Определяем ширину для каждой колонки (включая пробелы с обеих сторон)
        let col_widths = [
            ("UID", 38),             // 36 + 2 пробела
            ("TICKER", 12),          // максимальная длина тикера + 2
            ("CLASS_CODE", 14),      // длина + 2
            ("FIGI", 14),            // длина + 2
            ("POSITION_UID", 38),    // 36 + 2 пробела
            ("INSTRUMENT_TYPE", 17), // длина + 2
        ];

        Self::print_table_separator(&col_widths);
        Self::print_table_header(&col_widths);
        Self::print_table_separator(&col_widths);

        for instrument in all_instruments {
            Self::print_table_row(
                &[
                    &instrument.uid,
                    &instrument.ticker,
                    &instrument.class_code,
                    &instrument.figi,
                    &instrument.position_uid,
                    &instrument.instrument_type,
                ],
                &col_widths,
            );
            Self::print_table_separator(&col_widths);
        }
    }

    /// Выводит отфильтрованные инструменты в виде таблицы
    pub fn print_filtered_instruments(instruments: &[String]) {
        println!("Отфильтрованные инструменты:");
        for instrument in instruments {
            println!("- {}", instrument);
        }
    }

    fn print_table_row(data: &[&str], cols: &[(&str, usize)]) {
        print!("|");
        for (value, (_, width)) in data.iter().zip(cols.iter()) {
            let space_width = width - 2; // Учитываем отступы с обеих сторон
            if value.len() > space_width {
                // Обрезаем строку и добавляем ...
                print!(" {:.width$}... |", value, width = space_width - 3);
            } else {
                // Выводим значение с выравниванием по левому краю
                print!(" {:<width$} |", value, width = space_width);
            }
        }
        println!();
    }

    fn print_table_header(cols: &[(&str, usize)]) {
        print!("|");
        for (title, width) in cols {
            print!(" {:<width$} |", title, width = width - 2);
        }
        println!();
    }

    fn print_table_separator(cols: &[(&str, usize)]) {
        print!("+");
        for (_, width) in cols {
            print!("{:-<width$}+", "", width = width);
        }
        println!();
    }

    pub fn get_instrument_ticker(&self, instrument_uid: &str) -> Option<String> {
        self.assets.iter()
            .flat_map(|asset| asset.instruments.iter())
            .find(|instrument| instrument.uid == instrument_uid)
            .map(|instrument| instrument.ticker.clone())
    }

    pub fn get_all_instruments(&self) -> Vec<&Instrument> {
        let mut all_instruments = Vec::new();
        for asset in &self.assets {
            all_instruments.extend(asset.instruments.iter());
        }
        all_instruments
    }
}
