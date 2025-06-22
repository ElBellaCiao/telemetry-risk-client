use anyhow::Result;
use std::fmt::Display;
use std::time::{Duration, Instant};

pub struct InstanceMetadataClient {
    client: reqwest::blocking::Client,
    last_update: Instant,
    token: String,
}

impl InstanceMetadataClient {
    const TTL: Duration = Duration::from_secs(6 * 60 * 60); // 6 hours
    const REFRESH_BUFFER: Duration = Duration::from_secs(30 * 60); // 30 min

    pub fn new(client: reqwest::blocking::Client) -> Result<Self> {
        let token = Self::fetch_token(&client, Self::TTL)?;
        Ok(Self {
            client,
            token,
            last_update: Instant::now(),
        })
    }

    pub fn get_self_id(&mut self) -> Result<String> {
        self.request("instance-id")
    }

    fn fetch_token(client: &reqwest::blocking::Client, ttl: Duration) -> Result<String> {
        Ok(client
            .put("http://169.254.169.254/latest/api/token")
            .header(
                "X-aws-ec2-metadata-token-ttl-seconds",
                ttl.as_secs().to_string(),
            )
            .send()?
            .text()?)
    }

    fn request(&mut self, request: impl Display) -> Result<String> {
        if self.last_update.elapsed() > (Self::TTL - Self::REFRESH_BUFFER) {
            self.token = Self::fetch_token(&self.client, Self::TTL)?;
            self.last_update = Instant::now();
        }

        let response = self
            .client
            .get(format!(
                "http://169.254.169.254/latest/meta-data/{}",
                &request
            ))
            .header("X-aws-ec2-metadata-token", &self.token)
            .send()?
            .text()?;

        Ok(response)
    }
}
