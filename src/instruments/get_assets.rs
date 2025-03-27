use crate::models::enums::InstrumentType;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tracing::{debug, error, info};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,
    #[serde(rename = "instrumentUid")]
    pub instrument_uid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub trait IntoUid {
    fn into_uids(&self) -> Vec<String>;
}

impl IntoUid for Vec<Instrument> {
    fn into_uids(&self) -> Vec<String> {
        self.iter()
            .map(|instrument| instrument.uid.clone())
            .collect()
    }
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

            error!("Ошибка запроса: статус {}, текст: {}", status, error_text);
            return Err(format!("Ошибка API: {} - {}", status, error_text).into());
        }

        // Пытаемся десериализовать JSON
        match response.json::<Self>().await {
            Ok(assets_response) => {
                info!(
                    "Успешно десериализован ответ с {} активами",
                    assets_response.assets.len()
                );
                Ok(assets_response)
            }
            Err(e) => {
                error!("Ошибка десериализации ответа: {}", e);
                Err(e.into())
            }
        }
    }

    /// Фильтрует инструменты по заданным параметрам
    pub async fn filter_instruments(
        self,
        class_code: &str,
        instrument_type: &str,
    ) -> Result<Vec<Instrument>, Box<dyn Error>> {
        let mut filtered_instruments = Vec::new();

        for asset in self.assets {
            for instrument in asset.instruments {
                if instrument.class_code == class_code
                    && instrument.instrument_type == instrument_type
                {
                    filtered_instruments.push(instrument);
                }
            }
        }

        info!(
            "filtered instruments: {} with class_code: '{}' and instrument_type: '{}'",
            filtered_instruments.len(),
            class_code,
            instrument_type
        );

        filtered_instruments.sort_by(|a, b| a.ticker.cmp(&b.ticker));

        Ok(filtered_instruments)
    }

    /// Выводит информацию об инструментах в виде таблицы
    pub fn print_instruments(&self) {
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
    pub fn print_filtered_instruments(instruments: &Vec<Instrument>) {
        if instruments.is_empty() {
            info!("there are no tools to display");
            return;
        }

        let col_widths = [
            ("UID", 38),
            ("TICKER", 12),
            ("CLASS_CODE", 14),
            ("FIGI", 14),
            ("POSITION_UID", 38),
            ("INSTRUMENT_TYPE", 17),
        ];

        Self::print_table_separator(&col_widths);
        Self::print_table_header(&col_widths);
        Self::print_table_separator(&col_widths);

        for instrument in instruments {
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
}
