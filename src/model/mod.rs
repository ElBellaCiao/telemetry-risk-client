use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

mod unstructured;

pub use unstructured::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct RiskReport {
    pub resource_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub risk_info: RiskInfo,
}

#[derive(Debug, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum RiskType {
    Unstructured,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RiskInfo {
    Unstructured(Unstructured),
}

impl RiskInfo {
    pub fn risk_type(&self) -> RiskType {
        match self {
            RiskInfo::Unstructured(_) => RiskType::Unstructured,
        }
    }
}
