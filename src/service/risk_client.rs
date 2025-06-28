use crate::model::{Metadata, RiskInfo};
use crate::{client, config};
use chrono::Utc;

pub struct RiskClient {
    http_client: client::HttpSyncClient,
    instance_id: String,
    queue: kanal::Receiver<RiskInfo>,
    base_url: String,
}

impl RiskClient {
    pub fn new(
        http_client: client::HttpSyncClient,
        queue: kanal::Receiver<RiskInfo>,
        base_url: String,
        instance_id: String,
    ) -> Self {
        Self {
            http_client,
            queue,
            base_url,
            instance_id,
        }
    }

    pub fn run(&self) {
        let url = format!("{}/{}", self.base_url, config::RISK_ENDPOINT);
        let metadata = Metadata {
            resource_id: self.instance_id.clone(),
            timestamp: Utc::now(),
        };

        while let Ok(message) = self.queue.recv() {
            self.http_client.send(&url, &message, &metadata);
        }
    }
}
