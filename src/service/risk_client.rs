use crate::model::ErrorReport;
use crate::{client, model};

pub struct RiskClient {
    client: client::HttpSyncClient,
    queue: kanal::Receiver<ErrorReport>,
    base_url: String,
}

impl RiskClient {
    pub fn new(
        client: client::HttpSyncClient,
        queue: kanal::Receiver<ErrorReport>,
        base_url: String,
    ) -> Self {
        Self {
            client,
            queue,
            base_url,
        }
    }

    pub fn run(&self) {
        let url = format!("{}/{}", self.base_url, model::RISK_ENDPOINT);
        while let Ok(message) = self.queue.recv() {
            self.client.send(&url, message);
        }
    }
}
