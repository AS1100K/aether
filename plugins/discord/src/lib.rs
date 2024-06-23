#![doc = include_str!("../README.md")]

#[cfg(feature = "chat-bridge")]
pub mod chat_bridge;

use azalea::app::{Plugin, Update};
use azalea::ecs::prelude::*;
use azalea::prelude::*;
use bevy_tasks::IoTaskPool;
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
    pub webhook: &'static str,
    pub contents: &'static str,
    pub username: Option<&'static str>,
    pub avatar_url: Option<&'static str>,
}

#[derive(Serialize)]
struct Context {
    contents: &'static str,
    username: Option<&'static str>,
    avatar_url: Option<&'static str>,
}

fn handle_send_discord_message(
    mut events: EventReader<SendDiscordMessage>
) {
    for event in events.read() {
        let webhook = event.webhook;

        let context = Context {
            contents: event.contents,
            username: event.username,
            avatar_url: event.avatar_url,
        };

        let thread_pool = IoTaskPool::get();

        thread_pool
            .spawn(async move {
                let client = reqwest::Client::new();
                let res = client.post(webhook).json(&context).send().await;

                if let Ok(response) = res {
                    if response.status() != 204 {
                        warn!("Unable to send message");
                    }
                    return;
                }
            })
            .detach();
    }
}

pub trait DiscordExt {
    fn send_discord_message(&self, contex: SendDiscordMessage);
}

impl DiscordExt for Client {
    fn send_discord_message(&self, contex: SendDiscordMessage) {
        let mut  ecs = self.ecs.lock();
        ecs.send_event(contex);
    }
}