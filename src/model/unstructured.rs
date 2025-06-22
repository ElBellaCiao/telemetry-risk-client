use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Unstructured {
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}
