use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub telemetry_risk_service_url: String,
}
