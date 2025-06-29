use crate::client;
use crate::model::{Metadata, RiskInfo};
use chrono::Utc;

pub struct RiskClient {
    http_client: client::HttpSyncClient,
    instance_id: String,
    queue: kanal::Receiver<RiskInfo>,
    url: String,
}

impl RiskClient {
    pub fn new(
        http_client: client::HttpSyncClient,
        queue: kanal::Receiver<RiskInfo>,
        url: &str,
        instance_id: &str,
    ) -> Self {
        Self {
            http_client,
            queue,
            url: url.to_owned(),
            instance_id: instance_id.to_owned(),
        }
    }

    pub fn run(&self) {
        let metadata = Metadata {
            resource_id: self.instance_id.clone(),
            timestamp: Utc::now(),
        };

        while let Ok(message) = self.queue.recv() {
            self.http_client.send(&self.url, &message, &metadata);
        }
    }
}
