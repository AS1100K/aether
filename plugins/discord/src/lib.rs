#![doc = include_str!("../README.md")]

#[cfg(feature = "chat-bridge")]
pub mod chat_bridge;

#[cfg(feature = "log-bridge")]
pub mod log_bridge;

use azalea::app::{Plugin, Update};
use azalea::ecs::prelude::*;
use azalea::prelude::*;
use serde::Serialize;
use tracing::warn;

pub struct DiscordPlugin;

impl Plugin for DiscordPlugin {
    fn build(&self, app: &mut azalea::app::App) {
        app.add_event::<SendDiscordMessage>()
            .add_systems(Update, handle_send_discord_message);
    }
}

#[derive(Event)]
pub struct SendDiscordMessage {
    pub webhook: String,
    pub contents: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize)]
struct Context {
    content: String,
    username: Option<String>,
    avatar_url: Option<String>,
}

fn handle_send_discord_message(mut events: EventReader<SendDiscordMessage>) {
    for event in events.read() {
        let webhook = event.webhook.to_owned();

        let content = event.contents.to_owned();
        let username = event.username.to_owned();
        let avatar_url = event.avatar_url.to_owned();

        let context = Context {
            content,
            username,
            avatar_url,
        };

        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let res = client
                .post(format!("{}?wait=true", webhook))
                .json(&context)
                .send()
                .await;

            if let Ok(response) = res {
                if response.status() != 200 {
                    warn!("Unable to send message");
                }
            }
        });
    }
}

pub trait DiscordExt {
    fn send_discord_message(&self, context: SendDiscordMessage);
}

impl DiscordExt for Client {
    fn send_discord_message(&self, context: SendDiscordMessage) {
        let mut ecs = self.ecs.lock();
        ecs.send_event(context);
    }
}
