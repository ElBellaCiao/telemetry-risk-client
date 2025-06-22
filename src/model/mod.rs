use serde::{Deserialize, Serialize};

mod unstructured;

pub use unstructured::*;

#[derive(Debug, Deserialize, Serialize)]
pub enum RiskReport {
    Unstructured(UnstructuredReport),
}
