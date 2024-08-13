use azalea::app::{Plugin, Update};
use azalea::chat::{ChatPacketKind, ChatReceivedEvent, SendChatKindEvent};
use azalea::ecs::prelude::*;
use azalea::entity::metadata::Player;
use azalea::entity::LocalEntity;
use azalea::prelude::*;
use bevy_discord::bot::events::BMessage;
use bevy_discord::bot::serenity::all::ChannelId;
use bevy_discord::bot::DiscordBotRes;
use bevy_discord::runtime::tokio_runtime;
use serde_json::json;
use tracing::error;

use crate::chat::handle_chat;
use crate::config::Bot;

pub struct AetherDiscordPlugin;

/// Component present when `chat_relay` is turned on
#[derive(Component)]
pub struct DiscordChatRelay {
    pub channel_id: ChannelId,
}

/// Component present when `channel_id` is passed in `config.json`
#[derive(Component)]
pub struct DiscordChannelId {
    pub channel_id: ChannelId,
}

impl Plugin for AetherDiscordPlugin {
    fn build(&self, app: &mut azalea::app::App) {
        app.add_systems(
            Update,
            (handle_chat_relay, handle_discord_bridge)
                .chain()
                .after(handle_chat),
        );
    }
}

#[allow(clippy::complexity)]
fn handle_chat_relay(
    mut events: EventReader<ChatReceivedEvent>,
    query: Query<(&DiscordChatRelay, &Bot), (With<Player>, With<LocalEntity>)>,
    discord_bot_res: Res<DiscordBotRes>,
) {
    for ChatReceivedEvent { entity: _, packet } in events.read() {
        for (DiscordChatRelay { channel_id }, state) in query.iter() {
            let (sender, message) = packet.split_sender_and_content();
            let sender = sender.unwrap_or("Server".to_string());

            if sender != state.username
                && let Some(http) = discord_bot_res.get_http()
            {
                let channel_id_clone = channel_id.clone();
                tokio_runtime().spawn(async move {
                    if http
                        .send_message(
                            channel_id_clone,
                            Vec::new(),
                            &json!({
                                "content": format!("{} -> {}", sender, message)
                            }),
                        )
                        .await
                        .is_err()
                    {
                        error!("Unable to send message on discord");
                    }
                });
            }
        }
    }
}

fn handle_discord_bridge(
    mut events: EventReader<BMessage>,
    query: Query<(&DiscordChatRelay, Entity), (With<Player>, With<LocalEntity>)>,
    mut send_chat_kind_event: EventWriter<SendChatKindEvent>,
) {
    for BMessage {
        ctx: _,
        new_message,
    } in events.read()
    {
        for (DiscordChatRelay { channel_id }, entity) in query.iter() {
            if !new_message.author.bot && &new_message.channel_id == channel_id {
                send_chat_kind_event.send(SendChatKindEvent {
                    entity,
                    content: new_message.content.to_owned(),
                    kind: ChatPacketKind::Message,
                });
            }
        }
    }
}
