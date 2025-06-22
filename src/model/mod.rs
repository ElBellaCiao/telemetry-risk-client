use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod unstructured;

pub use unstructured::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct RiskReport {
    pub resource_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub risk_info: RiskInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RiskInfo {
    Unstructured(Unstructured),
}
