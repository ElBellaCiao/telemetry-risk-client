use serde::Serialize;

pub struct HttpSyncClient {
    client: reqwest::blocking::Client,
}

impl HttpSyncClient {
    pub fn new(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }

    pub fn send(&self, url: &str, body: &impl Serialize, query: &impl Serialize) {
        let _ = self.client.post(url).query(query).json(body).send();
    }
}
