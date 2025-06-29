mod risk_client;
mod risk_layer;

use crate::client;
use crate::config::Settings;
use crate::service::risk_client::RiskClient;
use crate::service::risk_layer::RiskLayer;
use anyhow::Result;
use cloud_util::get_config;
use std::thread;
use std::time::Duration;

pub fn get_risk_layer() -> Result<RiskLayer> {
    // config
    let config = get_config::<Settings>()?;

    // helper clients
    let reqwest_client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .pool_max_idle_per_host(1)
        .build()?;
    let mut metadata_client = client::InstanceMetadataClient::new(reqwest_client.clone())?;
    let api_client = client::HttpSyncClient::new(reqwest_client);

    // metadata
    let instance_id = metadata_client.get_self_id()?;

    // RiskClient
    let (tx, rx) = kanal::unbounded();
    let risk_client = RiskClient::new(
        api_client,
        rx,
        &config.telemetry_risk_service_url,
        &instance_id,
    );
    let handle = thread::spawn(move || risk_client.run());

    // RiskLayer
    RiskLayer::new(tx, Some(handle))
}
