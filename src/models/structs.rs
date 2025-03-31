use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    pub units: String,
    pub nano: i32,
}
