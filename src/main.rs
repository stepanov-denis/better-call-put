use crate::config::Config;
use reqwest;
use serde::{Deserialize, Serialize};
use crate::instrument::{InstrumentResponse, InstrumentIdType};
mod config;
mod instrument;

//
// ---------------------------------------------------------------------
// Структуры для запроса активов
#[derive(Serialize, Deserialize, Debug)]
enum InstrumentType {
    #[serde(rename = "INSTRUMENT_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "INSTRUMENT_TYPE_BOND")]
    Bond,
    #[serde(rename = "INSTRUMENT_TYPE_SHARE")]
    Share,
    #[serde(rename = "INSTRUMENT_TYPE_CURRENCY")]
    Currency,
    #[serde(rename = "INSTRUMENT_TYPE_ETF")]
    Etf,
    #[serde(rename = "INSTRUMENT_TYPE_FUTURES")]
    Futures,
    #[serde(rename = "INSTRUMENT_TYPE_SP")]
    Sp,
    #[serde(rename = "INSTRUMENT_TYPE_OPTION")]
    Option,
    #[serde(rename = "INSTRUMENT_TYPE_CLEARING_CERTIFICATE")]
    ClearingCertificate,
    #[serde(rename = "INSTRUMENT_TYPE_INDEX")]
    Index,
    #[serde(rename = "INSTRUMENT_TYPE_COMMODITY")]
    Commodity,
}

#[derive(Serialize, Deserialize, Debug)]
enum InstrumentStatus {
    #[serde(rename = "INSTRUMENT_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "INSTRUMENT_STATUS_BASE")]
    Base,
    #[serde(rename = "INSTRUMENT_STATUS_ALL")]
    All,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetsRequest {
    instrumentType: InstrumentType,
    instrumentStatus: InstrumentStatus,
}

//
// ---------------------------------------------------------------------
// Структуры для десериализации ответа активов

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Link {
    instrumentUid: String,
    #[serde(rename = "type")]
    link_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Instrument {
    uid: String, // Используем это значение как instrument_uid
    classCode: String,
    instrumentType: String, // Например, "share"
    ticker: String,
    positionUid: String,
    figi: String,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    uid: String,
    name: String,
    instruments: Vec<Instrument>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetsResponse {
    assets: Vec<Asset>,
}

//
// ---------------------------------------------------------------------
// Структуры для запроса торговых статусов

#[derive(Serialize, Deserialize, Debug)]
struct TradingStatusesRequest {
    instrumentId: Vec<String>, // Принимает FIGI или instrument_uid; теперь используется instrument_uid
}

#[derive(Serialize, Deserialize, Debug)]
struct TradingStatus {
    figi: String,
    #[serde(default)]
    tradingStatus: Option<String>, // Делаем поле опциональным, если его нет в ответе
    limitOrderAvailableFlag: bool,
    marketOrderAvailableFlag: bool,
    apiTradeAvailableFlag: bool,
    instrumentUid: String,
    bestpriceOrderAvailableFlag: bool,
    onlyBestPrice: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketDataResponse {
    tradingStatuses: Vec<TradingStatus>,
}

//
// ---------------------------------------------------------------------

//
// ---------------------------------------------------------------------
// Функции обработки

// Функция фильтрации инструментов по условиям:
// classCode == "TQBR" и instrumentType == "share"
fn filter_instruments(response: &AssetsResponse) -> (Vec<Instrument>, usize) {
    let mut filtered = Vec::new();
    for asset in &response.assets {
        for instr in &asset.instruments {
            if instr.classCode == "TQBR" && instr.instrumentType == "share" {
                filtered.push(instr.clone());
            }
        }
    }
    let count = filtered.len();
    (filtered, count)
}

// Функция для печати списка инструментов в виде таблицы с разделителями.
fn print_instruments_table(instruments: &[Instrument]) {
    let col1_width = 20;
    let col2_width = 20;
    let col3_width = 20;

    let separator = format!(
        "+-{:-<col1$}-+-{:-<col2$}-+-{:-<col3$}-+",
        "",
        "",
        "",
        col1 = col1_width,
        col2 = col2_width,
        col3 = col3_width
    );

    println!("{}", separator);
    println!(
        "| {:<col1$} | {:<col2$} | {:<col3$} |",
        "instrumentType",
        "ticker",
        "classCode",
        col1 = col1_width,
        col2 = col2_width,
        col3 = col3_width
    );
    println!("{}", separator);

    for instr in instruments {
        println!(
            "| {:<col1$} | {:<col2$} | {:<col3$} |",
            instr.instrumentType,
            instr.ticker,
            instr.classCode,
            col1 = col1_width,
            col2 = col2_width,
            col3 = col3_width
        );
        println!("{}", separator);
    }
}

//
// ---------------------------------------------------------------------
// Функция main (асинхронная)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new("config.yaml")?;
    // URL запроса активов
    let assets_url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssets";

    // Формирование запроса активов
    let request_body = AssetsRequest {
        instrumentType: InstrumentType::Share,
        instrumentStatus: InstrumentStatus::Base,
    };

    // Создание HTTP-клиента
    let client = reqwest::Client::new();

    // Выполнение POST-запроса для получения активов
    let response = client
        .post(assets_url)
        .bearer_auth(&config.api_token)
        .json(&request_body)
        .send()
        .await?;

    // Десериализация JSON-ответа в структуру AssetsResponse
    let assets_response: AssetsResponse = response.json().await?;
    // println!("Полный ответ сервера (Активы):\n{:#?}", assets_response);

    // Фильтрация инструментов по условиям и вывод таблицей
    let (filtered_instruments, count) = filter_instruments(&assets_response);
    println!("\nОтфильтрованный список инструментов (classCode = \"TQBR\" и instrumentType = \"share\")");
    println!("Количество отфильтрованных инструментов: {}\n", count);
    print_instruments_table(&filtered_instruments);

    // ---------------------------------------------------------------------
    // Новый запрос: Получение торговых статусов
    let market_data_url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatuses";

    // Используем в качестве идентификаторов поле instrument_uid (uid) отфильтрованных инструментов.
    let instrument_ids: Vec<String> = filtered_instruments
        .iter()
        .map(|instr| instr.uid.clone())
        .collect();

    if instrument_ids.is_empty() {
        println!("Нет инструментов для запроса торговых статусов.");
        return Ok(());
    }

    let trading_request = TradingStatusesRequest {
        instrumentId: instrument_ids,
    };

    // Выполнение POST-запроса для получения торговых статусов
    let md_response = client
        .post(market_data_url)
        .bearer_auth(&config.api_token)
        .json(&trading_request)
        .send()
        .await?;

    // Десериализация JSON-ответа в MarketDataResponse
    let market_data: MarketDataResponse = md_response.json().await?;
    println!(
        "\nОтвет сервера на запрос торговых статусов:\n{:#?}",
        market_data
    );

    // for market_data.normal in market_data.normals {
    //     let idType = InstrumentIdType::Figi;
    //     let classCode = None;
    //     let instrument_info = InstrumentResponse::get_instrument_info(&client, idType, filtered_instrument.uid, &config.api_token, classCode).await?;
    //     println!("{:#?}", instrument_info);
    // }

    Ok(())
}
