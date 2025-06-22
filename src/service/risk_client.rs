use crate::{RiskReport, client, config, model::RiskInfo};
use chrono::Utc;

pub struct RiskClient {
    http_client: client::HttpSyncClient,
    metadata_client: client::InstanceMetadataClient,
    queue: kanal::Receiver<RiskInfo>,
    base_url: String,
}

impl RiskClient {
    pub fn new(
        http_client: client::HttpSyncClient,
        metadata_client: client::InstanceMetadataClient,
        queue: kanal::Receiver<RiskInfo>,
        base_url: String,
    ) -> Self {
        Self {
            http_client,
            metadata_client,
            queue,
            base_url,
        }
    }

    pub fn run(&mut self) {
        while let Ok(message) = self.queue.recv() {
            let url = format!(
                "{}/{}/{}",
                self.base_url,
                config::RISK_ENDPOINT,
                message.risk_type()
            );

            let risk_report = RiskReport {
                timestamp: Utc::now(),
                resource_id: self.metadata_client.get_self_id().ok(),
                risk_info: message,
            };

            self.http_client.send(&url, risk_report);
        }
    }
}
