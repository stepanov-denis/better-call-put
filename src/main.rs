use crate::bot::Bot;
use crate::config::Config;
use instruments::get_assets::GetAssetsRequest;
use instruments::get_assets::GetAssetsResponse;
use instruments::get_assets::InstrumentStatus;
use models::enums::InstrumentType;
use reqwest;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;
mod bot;
mod config;
mod instruments;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
            // Инициализация логирования с расширенной конфигурацией
            tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,better_call_put=debug")))
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    
        info!("Запуск приложения");

        let config = Config::new("config.yaml")?;
    
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30)) // Увеличиваем таймаут
            .build()?;
    
        info!("HTTP клиент создан");

    // tracing_subscriber::fmt::init();
    
    

    // // Создание HTTP-клиента
    // let client = reqwest::Client::new();

    let request = GetAssetsRequest::new(InstrumentType::Share, InstrumentStatus::Base);

    let assets_response = match GetAssetsResponse::get_assets(&client, &config.api_token, request).await {
        Ok(response) => {
            info!("Успешно получены данные об активах");
            response
        }
        Err(e) => {
            error!("Ошибка получения активов: {}", e);
            return Err(e);
        }
    };

    // println!("All asets:\n{:#?}", assets_response);

    let filtered_instruments =
        match Bot::filter_instruments(assets_response, &config.class_code, &config.instrument_type).await {
            Ok(instruments) => {
                info!("активы отфильтрованы успешно");
                instruments
            }
            Err(e) => {
                error!("ошибка фильтрации активов: {}", e);
                return Err(e);
            }
        };

    Bot::print_instruments(&filtered_instruments);



















    

    //
    // ---------------------------------------------------------------------

    //
    // ---------------------------------------------------------------------
    // Структуры для запроса торговых статусов

    // #[derive(Serialize, Deserialize, Debug)]
    // struct TradingStatusesRequest {
    //     instrumentId: Vec<String>, // Принимает FIGI или instrument_uid; теперь используется instrument_uid
    // }

    // #[derive(Serialize, Deserialize, Debug)]
    // struct TradingStatus {
    //     figi: String,
    //     #[serde(default)]
    //     tradingStatus: Option<String>, // Делаем поле опциональным, если его нет в ответе
    //     limitOrderAvailableFlag: bool,
    //     marketOrderAvailableFlag: bool,
    //     apiTradeAvailableFlag: bool,
    //     instrumentUid: String,
    //     bestpriceOrderAvailableFlag: bool,
    //     onlyBestPrice: bool,
    // }

    // #[derive(Serialize, Deserialize, Debug)]
    // struct MarketDataResponse {
    //     tradingStatuses: Vec<TradingStatus>,
    // }

    // ---------------------------------------------------------------------
    // Новый запрос: Получение торговых статусов
    // let market_data_url = "https://invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatuses";

    // // Используем в качестве идентификаторов поле instrument_uid (uid) отфильтрованных инструментов.
    // let instrument_ids: Vec<String> = filtered_instruments
    //     .iter()
    //     .map(|instr| instr.uid.clone())
    //     .collect();

    // if instrument_ids.is_empty() {
    //     println!("Нет инструментов для запроса торговых статусов.");
    //     return Ok(());
    // }

    // let trading_request = TradingStatusesRequest {
    //     instrumentId: instrument_ids,
    // };

    // // Выполнение POST-запроса для получения торговых статусов
    // let md_response = client
    //     .post(market_data_url)
    //     .bearer_auth(&config.api_token)
    //     .json(&trading_request)
    //     .send()
    //     .await?;

    // // Десериализация JSON-ответа в MarketDataResponse
    // let market_data: MarketDataResponse = md_response.json().await?;
    // println!(
    //     "\nОтвет сервера на запрос торговых статусов:\n{:#?}",
    //     market_data
    // );

    Ok(())
}
