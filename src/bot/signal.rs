use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    last_short_ema: Option<f64>,
    last_long_ema: Option<f64>,
    has_crossed: bool,
}

impl CrossoverSignal {
    pub fn new(hysteresis_percentage: f64, hysteresis_periods: u32) -> Self {
        Self {
            hysteresis_percentage,
            hysteresis_periods,
            state: State::Between,
            time_in_state: 0,
            last_signal: None,
            last_short_ema: None,
            last_long_ema: None,
            has_crossed: false,
        }
    }

    pub fn update(&mut self, short_ema: f64, long_ema: f64) -> Signal {
        if long_ema == 0.0 {
            return Signal::Hold;
        }

        let ema_diff = short_ema - long_ema;
        let ema_percentage = ema_diff / long_ema * 100.0;
        
        info!(
            "EMA analysis: short={}, long={}, difference={}%, threshold={}%",
            short_ema, long_ema, ema_percentage, self.hysteresis_percentage
        );

        // First check for crossover
        if let (Some(last_short), Some(last_long)) = (self.last_short_ema, self.last_long_ema) {
            // Check for golden cross (buy signal)
            if short_ema > long_ema && last_short <= last_long {
                info!("Golden cross detected");
                self.has_crossed = true;
                self.state = State::Above;
                self.time_in_state = 0;
            }
            // Check for death cross (sell signal)
            else if short_ema < long_ema && last_short >= last_long {
                info!("Death cross detected");
                self.has_crossed = true;
                self.state = State::Below;
                self.time_in_state = 0;
            }
        }

        // If we have crossed, increment the counter and check for hysteresis threshold
        if self.has_crossed {
            self.time_in_state += 1;
            
            // Check for buy signal (short EMA above long EMA)
            if self.state == State::Above && ema_percentage > self.hysteresis_percentage {
                if self.time_in_state >= self.hysteresis_periods && self.last_signal != Some(Signal::Buy) {
                    info!("Buy signal generated after crossing and exceeding hysteresis threshold");
                    self.last_signal = Some(Signal::Buy);
                    self.time_in_state = 0;
                    self.has_crossed = false;
                    self.last_short_ema = Some(short_ema);
                    self.last_long_ema = Some(long_ema);
                    return Signal::Buy;
                }
            }
            // Check for sell signal (short EMA below long EMA)
            else if self.state == State::Below && ema_percentage < -self.hysteresis_percentage {
                if self.time_in_state >= self.hysteresis_periods && self.last_signal != Some(Signal::Sell) {
                    info!("Sell signal generated after crossing and exceeding hysteresis threshold");
                    self.last_signal = Some(Signal::Sell);
                    self.time_in_state = 0;
                    self.has_crossed = false;
                    self.last_short_ema = Some(short_ema);
                    self.last_long_ema = Some(long_ema);
                    return Signal::Sell;
                }
            }
        }

        // Update last values
        self.last_short_ema = Some(short_ema);
        self.last_long_ema = Some(long_ema);

        Signal::Hold
    }
} 