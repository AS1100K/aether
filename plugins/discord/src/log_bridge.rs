use reqwest::Client;
use std::sync::Arc;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::layer::{Context, SubscriberExt};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

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
            let mut visitor = serde_json::json!({});
            event.record(&mut |field, value| {
                visitor[field.name()] = serde_json::json!(value.to_string());
            });

            let log_message = format!("{}", visitor["message"]);

            let client = self.http_client.clone();
            let webhook_url = self.discord_webhook_url.clone();

            // Spawn an async task to send the log message to Discord
            tokio::spawn(async move {
                let payload = serde_json::json!({
                    "content": log_message,
                });

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

struct DiscordLogBridge;

impl DiscordLogBridge {
    pub fn init(webhook: String, log_level: Level) {
        let discord_layer = DiscordLayer::new(webhook, log_level);

        tracing_subscriber::registry()
            .with(discord_layer)
            .with(EnvFilter::new(format!("{}", log_level)))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}
