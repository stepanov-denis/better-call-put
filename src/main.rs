use crate::config::Config;
use crate::bot::MarketScanner;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;
use tokio::signal;

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
    let mut scanner = MarketScanner::new(config)?;
    
    // Создаем канал для отправки сигнала завершения
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    
    // Запускаем обработчик Ctrl+C в отдельной задаче
    tokio::spawn(async move {
        if let Err(e) = signal::ctrl_c().await {
            error!("Ошибка при обработке Ctrl+C: {}", e);
            return;
        }
        info!("Получен сигнал завершения Ctrl+C");
        let _ = shutdown_tx.send(());
    });

    // Запускаем сканер с поддержкой graceful shutdown
    scanner.start_with_shutdown(shutdown_rx).await?;

    info!("Программа успешно завершена");
    Ok(())
}
