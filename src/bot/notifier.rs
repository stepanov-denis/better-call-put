use std::collections::HashSet;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use tokio::sync::Mutex;
use crate::bot::signal::TradeSignal;
use tracing::{info, error};

pub type Subscribers = Arc<Mutex<HashSet<ChatId>>>;

pub struct SignalNotifier {
    bot: Bot,
    subscribers: Subscribers,
}

impl SignalNotifier {
    pub fn new(token: &str) -> Self {
        SignalNotifier {
            bot: Bot::new(token),
            subscribers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    async fn send_message(&self, chat_id: ChatId, text: &str) -> Result<(), teloxide::RequestError> {
        self.bot.send_message(chat_id, text).await?;
        Ok(())
    }

    pub async fn notify_signal(&self, instrument: &str, signal: &TradeSignal, short_ema: f64, long_ema: f64, last_price: f64) {
        let ema_diff = short_ema - long_ema;
        let ema_percentage = ema_diff / long_ema * 100.0;

        // Format price with appropriate precision
        let price_str = if last_price >= 1000.0 {
            format!("{:.2}", last_price)
        } else if last_price >= 100.0 {
            format!("{:.3}", last_price)
        } else if last_price >= 10.0 {
            format!("{:.4}", last_price)
        } else {
            format!("{:.6}", last_price)
        };

        let message = match signal {
            TradeSignal::Buy => format!(
                "🟢 BUY SIGNAL\n\
                Instrument: {}\n\
                Last Price: {}\n\
                Recommendation: BUY\n\
                Short EMA: {:.6}\n\
                Long EMA: {:.6}\n\
                Difference: {:.6}%",
                instrument, price_str, short_ema, long_ema, ema_percentage
            ),
            TradeSignal::Sell => format!(
                "🔴 SELL SIGNAL\n\
                Instrument: {}\n\
                Last Price: {}\n\
                Recommendation: SELL\n\
                Short EMA: {:.6}\n\
                Long EMA: {:.6}\n\
                Difference: {:.6}%",
                instrument, price_str, short_ema, long_ema, ema_percentage
            ),
            TradeSignal::Hold => {
                info!(
                    "HOLD POSITION\n\
                    Instrument: {}\n\
                    Last Price: {}\n\
                    Short EMA: {:.6}\n\
                    Long EMA: {:.6}\n\
                    Difference: {:.6}%",
                    instrument, price_str, short_ema, long_ema, ema_percentage
                );
                return; // Don't send Hold messages to Telegram
            }
        };

        let subs_snapshot = {
            let subs = self.subscribers.lock().await;
            subs.clone()
        };

        for chat_id in subs_snapshot {
            if let Err(err) = self.send_message(chat_id, &message).await {
                error!("Error sending signal to chat {}: {}", chat_id, err);
            }
        }
    }

    pub async fn start_listener(&self) {
        let bot = self.bot.clone();
        let subscribers = self.subscribers.clone();
        
        tokio::spawn(async move {
            teloxide::repl(bot, move |message: Message, bot: Bot| {
                let subscribers = subscribers.clone();
                async move {
                    if let Some(text) = message.text() {
                        if text == "/start" {
                            {
                                let mut subs = subscribers.lock().await;
                                subs.insert(message.chat.id);
                                info!("New subscriber: {}", message.chat.id);
                            }
                            if let Err(err) = bot.send_message(
                                message.chat.id,
                                "✅ You have subscribed to trading signals!"
                            ).await {
                                error!("Error sending welcome message: {}", err);
                            }
                        }
                    }
                    respond(())
                }
            })
            .await;
        });
    }
} 