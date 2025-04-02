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
    // Initialize logging with extended configuration
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

    info!("Starting application");

    let config = Config::new("config.yaml")?;
    let mut scanner = MarketScanner::new(config)?;
    
    // Create channel for sending termination signal
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    
    // Run Ctrl+C handler in a separate task
    tokio::spawn(async move {
        if let Err(e) = signal::ctrl_c().await {
            error!("Error handling Ctrl+C: {}", e);
            return;
        }
        info!("Received Ctrl+C termination signal");
        let _ = shutdown_tx.send(());
    });

    // Run scanner with graceful shutdown support
    scanner.start_with_shutdown(shutdown_rx).await?;

    info!("Program completed successfully");
    Ok(())
}
