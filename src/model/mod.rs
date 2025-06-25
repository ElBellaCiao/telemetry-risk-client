use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod unstructured;

pub use unstructured::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub resource_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RiskInfo {
    Unstructured(Unstructured),
}
