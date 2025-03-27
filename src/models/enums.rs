use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum InstrumentType {
    #[serde(rename = "INSTRUMENT_TYPE_UNSPECIFIED")]
    #[default]
    Unspecified,
    #[serde(rename = "INSTRUMENT_TYPE_BOND")]
    Bond,
    #[serde(rename = "INSTRUMENT_TYPE_SHARE")]
    Share,
    #[serde(rename = "INSTRUMENT_TYPE_CURRENCY")]
    Currency,
    #[serde(rename = "INSTRUMENT_TYPE_ETF")]
    Etf,
    #[serde(rename = "INSTRUMENT_TYPE_FUTURES")]
    Futures,
    #[serde(rename = "INSTRUMENT_TYPE_SP")]
    Sp,
    #[serde(rename = "INSTRUMENT_TYPE_OPTION")]
    Option,
    #[serde(rename = "INSTRUMENT_TYPE_CLEARING_CERTIFICATE")]
    ClearingCertificate,
    #[serde(rename = "INSTRUMENT_TYPE_INDEX")]
    Index,
    #[serde(rename = "INSTRUMENT_TYPE_COMMODITY")]
    Commodity,
}
