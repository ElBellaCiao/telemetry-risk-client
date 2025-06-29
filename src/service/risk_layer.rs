use crate::service::risk_client::RiskClient;
use crate::{Unstructured, client, model::RiskInfo};
use anyhow::Result;
use std::thread;
use std::time::Duration;
use tracing::field::Visit;
use tracing::{Event, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

pub struct RiskLayer {
    queue: kanal::Sender<RiskInfo>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl RiskLayer {
    pub fn new(base_url: &str) -> Result<Self> {
        // todo: Expose Key Settings Only
        let reqwest_client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .pool_max_idle_per_host(1)
            .build()?;

        let api_client = client::HttpSyncClient::new(reqwest_client.clone());

        let mut metadata_client = client::InstanceMetadataClient::new(reqwest_client)?;
        let instance_id = metadata_client.get_self_id()?;

        let (tx, rx) = kanal::unbounded();
        let risk_client = RiskClient::new(api_client, rx, base_url.to_string(), instance_id);
        let handle = thread::spawn(move || risk_client.run());

        Ok(Self {
            queue: tx,
            thread_handle: Some(handle),
        })
    }
}

impl Drop for RiskLayer {
    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

impl<S> Layer<S> for RiskLayer
where
    S: Subscriber,
{
    fn on_event(&self, _event: &Event<'_>, _ctx: Context<'_, S>) {
        if *_event.metadata().level() == tracing::Level::ERROR {
            let mut visitor = MessageVisitor::default();
            _event.record(&mut visitor);

            let metadata = _event.metadata();

            let error_report = RiskInfo::Unstructured(Unstructured {
                message: visitor.message,
                file: metadata.file().map(|s| s.to_string()),
                line: metadata.line(),
            });

            let _ = self.queue.send(error_report);
        }
    }
}

#[derive(Default)]
struct MessageVisitor {
    pub message: String,
}

impl Visit for MessageVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}
