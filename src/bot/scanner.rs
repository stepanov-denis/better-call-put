use std::time::Duration;
use std::collections::HashMap;
use crate::config::Config;
use crate::instruments::get_assets::{GetAssetsRequest, GetAssetsResponse, IntoUid};
use crate::bot::trade::EmaCrossStrategy;
use crate::bot::notifier::SignalNotifier;
use tracing::{error, info};
use tokio::sync::oneshot;
use tokio::select;
use crate::market_data_service::get_trading_statuses::GetTradingStatusesResponse;
use crate::market_data_service::get_trading_statuses::_check_instruments_availability;

pub struct MarketScanner {
    client: reqwest::Client,
    config: Config,
    notifier: SignalNotifier,
    scan_interval: Duration,
    strategies: HashMap<String, EmaCrossStrategy>,
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
            scan_interval: Duration::from_secs(config.scan_interval_seconds),
            strategies: HashMap::new(),
        })
    }

    pub async fn start_with_shutdown(
        &mut self,
        mut shutdown: oneshot::Receiver<()>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.notifier.start_listener().await;
        
        info!("Starting continuous market scanning with interval {:?}", self.scan_interval);
        
        loop {
            select! {
                // Check termination signal
                _ = &mut shutdown => {
                    info!("Received termination signal, stopping scanning");
                    break;
                }
                // Perform scanning
                _ = async {
                    match self.scan_market().await {
                        Ok(_) => {
                            info!("Scanning cycle completed successfully. Pause {:?}", self.scan_interval);
                        }
                        Err(e) => {
                            error!("Error during market scanning: {}. Pause {:?}", e, self.scan_interval);
                        }
                    }
                    tokio::time::sleep(self.scan_interval).await;
                } => {}
            }
        }

        info!("Market scanning stopped");
        Ok(())
    }

    /// Scans the market and returns trading signals for available instruments
    async fn scan_market(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting market scanning cycle");

        let request = GetAssetsRequest::new(
            self.config.assets.instrument_type.clone(),
            self.config.assets.instrument_status,
        );

        let assets_response = match GetAssetsResponse::get_assets(&self.client, &self.config.t_token, request).await {
            Ok(response) => {
                info!("Successfully received asset data");
                response
            }
            Err(e) => {
                error!("Error getting assets: {}", e);
                return Err(e);
            }
        };

        let assets_response_clone = assets_response.clone();

        let filtered_instruments = match assets_response
            .filter_instruments(&self.config.filter.class_code, &self.config.filter.instrument_type.as_str())
            .await
        {
            Ok(instruments) => {
                info!("Assets filtered successfully");
                instruments
            }
            Err(e) => {
                error!("Error filtering assets: {}", e);
                return Err(e);
            }
        };

        GetAssetsResponse::print_filtered_instruments(&filtered_instruments);

        _check_instruments_availability(&self.client, &self.config.t_token, filtered_instruments.clone().into_uids())
            .await?;

        let trading_statuses = GetTradingStatusesResponse::get_trading_statuses(
            &self.client,
            &self.config.t_token,
            filtered_instruments.into_uids(),
        )
        .await?;

        let available_instruments = trading_statuses.get_available_instruments();
        info!("Found {} available instruments for trading", available_instruments.len());

        for available_instrument in available_instruments {
            let ticker = assets_response_clone.get_instrument_ticker(&available_instrument)
                .unwrap_or_else(|| available_instrument.clone());

            let strategy = self.strategies.entry(available_instrument.clone()).or_insert_with(|| {
                EmaCrossStrategy::new(
                    available_instrument.clone(),
                    ticker.clone(),
                    self.config.strategy.short_ema_length,
                    self.config.strategy.long_ema_length,
                    self.config.strategy.interval,
                    self.config.strategy.hysteresis_percentage,
                    self.config.strategy.hysteresis_periods,
                )
            });

            match strategy.get_trade_signal(&self.client, &self.config.t_token).await {
                Ok(signal) => {
                    info!("Received signal {:?} for instrument {} ({})", 
                        signal, 
                        strategy.get_ticker(), 
                        available_instrument
                    );
                    let trade_signal = crate::bot::signal::TradeSignal::from(signal);
                    self.notifier.notify_signal(
                        &format!("{} ({})", strategy.get_ticker(), available_instrument),
                        &trade_signal,
                        strategy.get_last_short(),
                        strategy.get_last_long(),
                        strategy.get_last_price()
                    ).await;
                }
                Err(e) => {
                    error!("Error getting signal for {} ({}): {}", 
                        strategy.get_ticker(), 
                        available_instrument, 
                        e
                    );
                }
            }
        }

        Ok(())
    }
} 