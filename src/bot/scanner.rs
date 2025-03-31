use std::time::Duration;
use crate::config::Config;
use crate::instruments::get_assets::{GetAssetsRequest, GetAssetsResponse, IntoUid};
use crate::bot::trade::EmaCrossStrategy;
use crate::bot::notifier::SignalNotifier;
use tracing::{error, info};
use tokio::sync::oneshot;
use tokio::select;
use crate::quotes::get_trading_statuses::GetTradingStatusesResponse;

pub struct MarketScanner {
    client: reqwest::Client,
    config: Config,
    notifier: SignalNotifier,
    scan_interval: Duration,
}

impl MarketScanner {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let notifier = SignalNotifier::new(&config.telegram_token);

        Ok(Self { 
            client, 
            config: config.clone(),
            notifier,
            scan_interval: Duration::from_secs(config.scan_interval_seconds),  // Используем значение из конфига
        })
    }

    pub async fn start_with_shutdown(
        &self,
        mut shutdown: oneshot::Receiver<()>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.notifier.start_listener().await;
        
        info!("Запуск непрерывного сканирования рынка с интервалом {:?}", self.scan_interval);
        
        loop {
            select! {
                // Проверяем сигнал завершения
                _ = &mut shutdown => {
                    info!("Получен сигнал завершения, останавливаем сканирование");
                    break;
                }
                // Выполняем сканирование
                _ = async {
                    match self.scan_market().await {
                        Ok(_) => {
                            info!("Цикл сканирования завершен успешно. Пауза {:?}", self.scan_interval);
                        }
                        Err(e) => {
                            error!("Ошибка при сканировании рынка: {}. Пауза {:?}", e, self.scan_interval);
                        }
                    }
                    tokio::time::sleep(self.scan_interval).await;
                } => {}
            }
        }

        info!("Сканирование рынка остановлено");
        Ok(())
    }

    /// Сканирует рынок и возвращает торговые сигналы для доступных инструментов
    async fn scan_market(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Начало цикла сканирования рынка");

        let request = GetAssetsRequest::new(
            self.config.assets.instrument_type.clone(),
            self.config.assets.instrument_status,
        );

        let assets_response = match GetAssetsResponse::get_assets(&self.client, &self.config.api_token, request).await {
            Ok(response) => {
                info!("Успешно получены данные об активах");
                response
            }
            Err(e) => {
                error!("Ошибка получения активов: {}", e);
                return Err(e);
            }
        };

        let filtered_instruments = match assets_response
            .filter_instruments(
                &self.config.class_code,
                &self.config.instrument_type.as_str()
            )
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

        let trading_statuses = GetTradingStatusesResponse::get_trading_statuses(
            &self.client,
            &self.config.api_token,
            filtered_instruments.into_uids(),
        )
        .await?;

        let available_instruments = trading_statuses.get_available_instruments();

        info!("Доступные инструменты: {:?}", available_instruments);

        for available_instrument in available_instruments {
            let strategy = EmaCrossStrategy::new(
                available_instrument.clone(),
                self.config.strategy.short_ema_length,
                self.config.strategy.long_ema_length,
                self.config.strategy.interval,
            );

            match strategy.get_trade_signal(&self.client, &self.config.api_token).await {
                Ok(signal) => {
                    info!("Получен сигнал {:?} для инструмента {}", signal, available_instrument);
                    self.notifier.notify_signal(&available_instrument, &signal).await;
                }
                Err(e) => {
                    error!("Ошибка получения сигнала для {}: {}", available_instrument, e);
                }
            }
        }

        Ok(())
    }
} 