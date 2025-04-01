use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Copy)]
pub enum TradeSignal {
    Buy,
    Sell,
    Hold,
}

impl From<Signal> for TradeSignal {
    fn from(signal: Signal) -> Self {
        match signal {
            Signal::Buy => Self::Buy,
            Signal::Sell => Self::Sell,
            Signal::Hold => Self::Hold,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Above,
    Below,
    Between,
}

#[derive(Debug)]
pub struct CrossoverSignal {
    hysteresis_percentage: f64,
    hysteresis_periods: u32,
    state: State,
    time_in_state: u32,
    last_signal: Option<Signal>,
}

impl CrossoverSignal {
    pub fn new(hysteresis_percentage: f64, hysteresis_periods: u32) -> Self {
        Self {
            hysteresis_percentage,
            hysteresis_periods,
            state: State::Between,
            time_in_state: 0,
            last_signal: None,
        }
    }

    pub fn update(&mut self, short_ema: f64, long_ema: f64) -> Signal {
        if long_ema == 0.0 {
            return Signal::Hold;
        }

        let ema_diff = short_ema - long_ema;
        let ema_percentage = ema_diff / long_ema * 100.0;
        
        info!(
            "EMA анализ: короткая={}, длинная={}, разница={}%, порог={}%",
            short_ema, long_ema, ema_percentage, self.hysteresis_percentage
        );

        let new_state = if ema_percentage > self.hysteresis_percentage {
            State::Above
        } else if ema_percentage < -self.hysteresis_percentage {
            State::Below
        } else {
            State::Between
        };

        if new_state == self.state {
            self.time_in_state += 1;
        } else {
            info!("Смена состояния: {:?} -> {:?}", self.state, new_state);
            self.state = new_state;
            self.time_in_state = 1;
        }

        match self.state {
            State::Above => {
                if self.time_in_state >= self.hysteresis_periods && self.last_signal != Some(Signal::Buy) {
                    info!("Сформирован сигнал на покупку после {} периодов", self.time_in_state);
                    self.last_signal = Some(Signal::Buy);
                    Signal::Buy
                } else {
                    Signal::Hold
                }
            }
            State::Below => {
                if self.time_in_state >= self.hysteresis_periods && self.last_signal != Some(Signal::Sell) {
                    info!("Сформирован сигнал на продажу после {} периодов", self.time_in_state);
                    self.last_signal = Some(Signal::Sell);
                    Signal::Sell
                } else {
                    Signal::Hold
                }
            }
            State::Between => Signal::Hold,
        }
    }
} 