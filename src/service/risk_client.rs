use crate::model::{Metadata, RiskInfo};
use crate::{client, config};
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
        let url = format!("{}/{}", self.base_url, config::RISK_ENDPOINT);
        let metadata = Metadata {
            resource_id: self.metadata_client.get_self_id().ok(),
            timestamp: Utc::now(),
        };

        while let Ok(message) = self.queue.recv() {
            self.http_client.send(&url, &message, &metadata);
        }
    }
}
