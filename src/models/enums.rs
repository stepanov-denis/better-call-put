use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentType {
    #[serde(rename = "INSTRUMENT_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "INSTRUMENT_TYPE_BOND")]
    Bond,
    #[serde(rename = "INSTRUMENT_TYPE_CURRENCY")]
    Currency,
    #[serde(rename = "INSTRUMENT_TYPE_ETF")]
    Etf,
    #[serde(rename = "INSTRUMENT_TYPE_FUTURES")]
    Futures,
    #[serde(rename = "INSTRUMENT_TYPE_SHARE")]
    Share,
    #[serde(rename = "INSTRUMENT_TYPE_OPTION")]
    Option,
    #[serde(rename = "INSTRUMENT_TYPE_INDEX")]
    Index,
    #[serde(rename = "INSTRUMENT_TYPE_COMMODITY")]
    Commodity,
    #[serde(rename = "INSTRUMENT_TYPE_CRYPTOCURRENCY")]
    CryptoCurrency,
    #[serde(rename = "INSTRUMENT_TYPE_SP")]
    Sp,
    #[serde(rename = "INSTRUMENT_TYPE_CLEARING_CERTIFICATE")]
    ClearingCertificate,
}

// Реализуем Display для InstrumentType
impl fmt::Display for InstrumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstrumentType::Unspecified => write!(f, "INSTRUMENT_TYPE_UNSPECIFIED"),
            InstrumentType::Bond => write!(f, "INSTRUMENT_TYPE_BOND"),
            InstrumentType::Currency => write!(f, "INSTRUMENT_TYPE_CURRENCY"),
            InstrumentType::Etf => write!(f, "INSTRUMENT_TYPE_ETF"),
            InstrumentType::Futures => write!(f, "INSTRUMENT_TYPE_FUTURES"),
            InstrumentType::Share => write!(f, "INSTRUMENT_TYPE_SHARE"),
            InstrumentType::Option => write!(f, "INSTRUMENT_TYPE_OPTION"),
            InstrumentType::Index => write!(f, "INSTRUMENT_TYPE_INDEX"),
            InstrumentType::Commodity => write!(f, "INSTRUMENT_TYPE_COMMODITY"),
            InstrumentType::CryptoCurrency => write!(f, "INSTRUMENT_TYPE_CRYPTOCURRENCY"),
            InstrumentType::Sp => write!(f, "INSTRUMENT_TYPE_SP"),
            InstrumentType::ClearingCertificate => write!(f, "INSTRUMENT_TYPE_CLEARING_CERTIFICATE"),
        }
    }
}

impl InstrumentType {
    pub fn as_str(&self) -> &str {
        match self {
            InstrumentType::Unspecified => "unspecified",
            InstrumentType::Bond => "bond",
            InstrumentType::Currency => "currency",
            InstrumentType::Etf => "etf",
            InstrumentType::Futures => "futures",
            InstrumentType::Share => "share",
            InstrumentType::Option => "option",
            InstrumentType::Index => "index",
            InstrumentType::Commodity => "commodity",
            InstrumentType::CryptoCurrency => "cryptocurrency",
            InstrumentType::Sp => "sp",
            InstrumentType::ClearingCertificate => "clearing_certificate",
        }
    }
}
