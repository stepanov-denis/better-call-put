impl GetTradingStatusesResponse {
    // ... остальные методы ...

    /// Проверяет доступность инструмента для торговли
    /// Возвращает true если:
    /// 1. Инструмент найден
    /// 2. API торговля доступна (api_trade_available_flag)
    /// 3. Торговый статус NORMAL_TRADING
    pub fn is_instrument_available(&self, instrument_uid: &str) -> bool {
        self.trading_statuses
            .iter()
            .find(|status| status.instrument_uid == instrument_uid)
            .map(|status| {
                status.api_trade_available_flag && 
                matches!(status.trading_status, Some(TradingStatus::NormalTrading))
            })
            .unwrap_or(false)
    }

    /// Получает полную информацию о статусе инструмента
    pub fn get_instrument_status(&self, instrument_uid: &str) -> Option<&TradingStatusResponse> {
        self.trading_statuses
            .iter()
            .find(|status| status.instrument_uid == instrument_uid)
    }
}

// Обновляем пример использования для более подробного вывода
pub async fn check_instruments_availability(
    client: &reqwest::Client,
    token: &str,
    instrument_ids: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = GetTradingStatusesResponse::get_trading_statuses(
        client,
        token,
        instrument_ids,
    ).await?;

    for status in &response.trading_statuses {
        println!("Инструмент {}: ", status.instrument_uid);
        println!("  Доступен для торговли: {}", response.is_instrument_available(&status.instrument_uid));
        println!("  Доступен для API торговли: {}", status.api_trade_available_flag);
        println!("  Доступны лимитные заявки: {}", status.limit_order_available_flag);
        println!("  Доступны рыночные заявки: {}", status.market_order_available_flag);
        println!("  Торговый статус: {:?}", status.trading_status);
        println!();
    }

    Ok(())
} 