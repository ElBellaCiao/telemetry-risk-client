use crate::{Unstructured, model::RiskInfo};
use anyhow::Result;
use std::thread;
use tracing::field::Visit;
use tracing::{Event, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

pub struct RiskLayer {
    queue: kanal::Sender<RiskInfo>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl RiskLayer {
    pub fn new(
        tx: kanal::Sender<RiskInfo>,
        handler: Option<thread::JoinHandle<()>>,
    ) -> Result<Self> {
        Ok(Self {
            queue: tx,
            thread_handle: handler,
        })
    }
}

impl Drop for RiskLayer {
    fn drop(&mut self) {
        let _ = self.queue.close();
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

impl<S> Layer<S> for RiskLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        if event.metadata().level() == &tracing::Level::ERROR {
            let mut visitor = MessageVisitor::default();
            event.record(&mut visitor);

            let metadata = event.metadata();

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
