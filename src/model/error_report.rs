use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorReport {
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub timestamp: DateTime<Utc>,
}
