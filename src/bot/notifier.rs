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

    pub async fn notify_signal(&self, instrument: &str, signal: &TradeSignal) {
        let message = match signal {
            TradeSignal::Buy => format!(
                "🟢 СИГНАЛ НА ПОКУПКУ\nИнструмент: {}\nРекомендация: КУПИТЬ",
                instrument
            ),
            TradeSignal::Sell => format!(
                "🔴 СИГНАЛ НА ПРОДАЖУ\nИнструмент: {}\nРекомендация: ПРОДАТЬ",
                instrument
            ),
            TradeSignal::Hold => format!(
                "⚪️ УДЕРЖАНИЕ ПОЗИЦИИ\nИнструмент: {}\nРекомендация: ДЕРЖАТЬ",
                instrument
            ),
        };

        let subs_snapshot = {
            let subs = self.subscribers.lock().await;
            subs.clone()
        };

        for chat_id in subs_snapshot {
            if let Err(err) = self.send_message(chat_id, &message).await {
                error!("Ошибка отправки сигнала в чат {}: {}", chat_id, err);
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
                                info!("Новый подписчик: {}", message.chat.id);
                            }
                            if let Err(err) = bot.send_message(
                                message.chat.id,
                                "✅ Вы подписались на получение торговых сигналов!"
                            ).await {
                                error!("Ошибка отправки приветствия: {}", err);
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