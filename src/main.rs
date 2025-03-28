use crate::bot::Bot;
use crate::config::Config;
use instruments::get_assets::GetAssetsRequest;
use instruments::get_assets::GetAssetsResponse;
use instruments::get_assets::InstrumentStatus;
use instruments::get_assets::IntoUid;
use models::enums::InstrumentType;
use quotes::get_trading_statuses::GetTradingStatusesResponse;
use quotes::get_trading_statuses::check_instruments_availability;
use reqwest;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
mod bot;
mod config;
mod instruments;
mod models;
mod quotes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логирования с расширенной конфигурацией
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,better_call_put=debug")),
        )
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

    let request = GetAssetsRequest::new(InstrumentType::Share, InstrumentStatus::Base);

    let assets_response =
        match GetAssetsResponse::get_assets(&client, &config.api_token, request).await {
            Ok(response) => {
                info!("Успешно получены данные об активах");
                response
            }
            Err(e) => {
                error!("Ошибка получения активов: {}", e);
                return Err(e);
            }
        };

    // Вывести все инструменты
    assets_response.print_instruments();

    let filtered_instruments =
        match assets_response.filter_instruments(&config.class_code, &config.instrument_type)
            .await
        {
            Ok(instruments) => {
                info!("активы отфильтрованы успешно");
                instruments
            }
            Err(e) => {
                error!("ошибка фильтрации активов: {}", e);
                return Err(e);
            }
        };

    GetAssetsResponse::print_filtered_instruments(&filtered_instruments);

    check_instruments_availability(&client, &config.api_token, filtered_instruments.into_uids()).await?;

    let trading_statuses = GetTradingStatusesResponse::get_trading_statuses(&client, &config.api_token, filtered_instruments.into_uids()).await?;

    Ok(())
}
