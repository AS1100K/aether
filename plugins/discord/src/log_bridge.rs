use reqwest::Client;
use std::sync::Arc;
use serde_json::json;
use tracing::{Event, Subscriber};
use tracing::field::Field;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::EnvFilter;
pub use tracing::Level;
use tracing_subscriber::field::Visit;

struct DiscordLayer {
    discord_webhook_url: String,
    http_client: Arc<Client>,
    log_level: Level,
}

impl<S> Layer<S> for DiscordLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event, _ctx: Context<S>) {
        if event.metadata().level() <= &self.log_level {
            let mut visitor = JsonVisitor::new();

            event.record(&mut visitor);

            let log_message = format!("{}", visitor.0["message"]);
            let client = self.http_client.clone();
            let webhook_url = self.discord_webhook_url.clone();

            tokio::spawn(async move {
                // Create the payload for the Discord webhook
                let payload = serde_json::json!({
                    "content": log_message,
                });

                // Send the payload to the Discord webhook
                let _ = client.post(&webhook_url).json(&payload).send().await;
            });
        }
    }
}

impl DiscordLayer {
    fn new(discord_webhook_url: String, log_level: Level) -> Self {
        DiscordLayer {
            discord_webhook_url,
            http_client: Arc::new(Client::new()),
            log_level,
        }
    }
}

struct JsonVisitor(serde_json::Value);

impl JsonVisitor {
    fn new() -> Self {
        JsonVisitor(json!({}))
    }
}

impl Visit for JsonVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0[field.name()] = json!(format!("{:?}", value));
    }
}

pub struct DiscordLogBridge;

impl DiscordLogBridge {
    /// All the logs will be bridged to discord after running this function.
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use azalea_discord::log_bridge::{DiscordLogBridge, Level};
    /// use tracing::info;
    ///
    /// #[derive(Default)]
    /// struct Config {
    ///     log_bridge: Option<String>
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = Config::default();
    ///
    ///     if let Some(webhook) = config.log_bridge {
    ///         DiscordLogBridge::init(webhook, Level::INFO);
    ///     }
    ///
    ///     info!("This message will be shown in discord.");
    /// }
    /// ```
    pub fn init(webhook: String, log_level: Level) {
        let discord_layer = DiscordLayer::new(webhook, log_level);

        tracing_subscriber::registry()
            .with(discord_layer)
            .with(EnvFilter::new(format!("{}", log_level)))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}