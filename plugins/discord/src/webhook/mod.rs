use crate::common::DiscordMessage;
use crate::runtime::tokio_runtime;
use crate::DiscordSet;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use reqwest::StatusCode;
use std::collections::HashMap;
use tracing::{error, trace};

#[derive(Clone)]
pub struct DiscordWebhookPlugin(DiscordWebhookRes);

#[derive(Resource, Clone, Default)]
pub struct DiscordWebhookRes {
    channels: HashMap<&'static str, Channel<'static>>,
}

#[derive(Clone)]
pub struct Channel<'a> {
    /// Prefix in every message of this channel. Mainly you would use mention here, like `@everyone`
    /// NOTE: When text are joined with prefix, it automatically adds a space.
    pub message_prefix: &'a str,
    /// Similar to `Channel::message_prefix` but at the end of the message.
    pub message_suffix: &'a str,
    pub webhook_url: &'a str,
}

impl DiscordWebhookPlugin {
    /// Create a new discord Plugin
    pub fn new(discord_webhook_res: DiscordWebhookRes) -> Self {
        Self(discord_webhook_res)
    }
}

/// Discord Plugin Resource
impl DiscordWebhookRes {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    /// Adds a new channel
    /// To know more about its fields see, [`Channel`]
    pub fn add_channel(
        mut self,
        name_identifier: &'static str,
        webhook_url: &'static str,
        message_prefix: &'static str,
        message_suffix: &'static str,
    ) -> Self {
        self.channels.insert(
            name_identifier,
            Channel {
                message_prefix,
                message_suffix,
                webhook_url,
            },
        );
        self
    }
}

impl Plugin for DiscordWebhookPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone())
            .add_event::<SendMessageEvent>()
            .add_systems(Update, handle_send_message.in_set(DiscordSet));
    }
}

/// Sending this event will send a message on the channel.
#[derive(Event, Clone)]
pub struct SendMessageEvent {
    name_identifier: &'static str,
    message: DiscordMessage,
}

impl SendMessageEvent {
    /// Create a new [`SendMessageEvent`]
    pub fn new(name_identifier: &'static str, message: DiscordMessage) -> Self {
        Self {
            name_identifier,
            message,
        }
    }
}

fn handle_send_message(
    mut events: EventReader<SendMessageEvent>,
    discord_webhook_res: Res<DiscordWebhookRes>,
) {
    for event in events.read() {
        if let Some(channel) = discord_webhook_res.channels.get(event.name_identifier) {
            let channel_clone = channel.clone();
            let event_clone = event.clone();

            tokio_runtime().spawn(async move {
                let client = reqwest::Client::new();
                trace!("body => {:?}", serde_json::to_string(&event_clone.message));

                let res = client.post(channel_clone.webhook_url)
                    .query(&[("wait", true)])
                    .json(&event_clone.message)
                    .send()
                    .await;

                match res {
                    Ok(response) => {
                        if response.status() != StatusCode::OK {
                            error!("Got response code {}. The message might contains problem in body. Make sure messages are compliant with discord webhook API. Learn more at https://discord.com/developers/docs/resources/webhook#execute-webhook", response.status())
                        }
                    }
                    Err(err) => {
                        error!("Unable to send message to discord webhook, error => {:?}", err.without_url())
                    }
                }
            });
        } else {
            error!("Unable to find discord channel.");
        }
    }
}
