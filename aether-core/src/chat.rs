use std::time::Instant;

use azalea::app::{App, Plugin};
use azalea::chat::{ChatReceivedEvent, SendChatEvent};
use azalea::ecs::prelude::*;
use azalea::entity::metadata::Player;
use azalea::entity::LocalEntity;
use azalea::prelude::*;
use azalea_anti_afk::config::AntiAFKConfig;
use azalea_anti_afk::AntiAFK;
use azalea_utility::auto_totem::AutoTotem;
use bevy_discord::bot::serenity::all::ChannelId;
use bevy_discord::bot::DiscordBotRes;
use bevy_discord::runtime::tokio_runtime;
use serde_json::json;
use tracing::error;

use crate::command::AetherCommand;
use crate::commands::pearl::load::LoadPearl;
use crate::config::{Bot, Config};
use crate::discord::{DiscordChannelId, DiscordChatRelay};
use crate::utils::{parse_chat_content, InWorld};

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(GameTick, handle_chat);
    }
}

#[allow(clippy::complexity)]
pub fn handle_chat(
    mut events: EventReader<ChatReceivedEvent>,
    query: Query<(&Bot, Option<&InWorld>), (With<Player>, With<LocalEntity>)>,
    config: Res<Config>,
    discord_bot_res: Option<Res<DiscordBotRes>>,
    mut commands: Commands,
    mut send_chat_event: EventWriter<SendChatEvent>,
    mut load_pearl: EventWriter<LoadPearl>,
) {
    for ChatReceivedEvent { entity, packet } in events.read() {
        for (state, in_world) in query.iter() {
            let (username, content, is_whisper) = parse_chat_content(packet);

            if let Some(uname) = username {
                if uname != state.username
                    && config.members.contains(&uname)
                    && in_world.is_some()
                    && is_whisper
                {
                    // Parse the command and execute it
                    // TODO: Notify Command received in Discord
                    let aether_command = AetherCommand::parse(&content, state);
                    match aether_command {
                        AetherCommand::PearlLoad => {
                            load_pearl.send(LoadPearl {
                                entity: *entity,
                                username: uname,
                            });
                        }
                        AetherCommand::Help => {
                            send_chat_event.send(SendChatEvent {
                                        entity: *entity,
                                        content: format!("/w {} Hi, I am {} a part of Project Aether. For more information see https://github.com/as1100k/aether.git or have a talk with AS1100K. Tnx!", uname, state.username),
                                    });
                        }
                        AetherCommand::Unknown => {
                            send_chat_event.send(SendChatEvent {
                                entity: *entity,
                                content: format!(
                                    "/w {} Oops! unknown command idk what to do. Ohh god!",
                                    uname
                                ),
                            });
                        }
                    }
                }
            } else if content == "Connected to the server." {
                commands.entity(*entity).insert(InWorld);
                commands.entity(*entity).insert(AntiAFK {
                    last_afk_tick: Instant::now(),
                    has_moved: None,
                    config: AntiAFKConfig {
                        jump: true,
                        sneak: false,
                        walk: false,
                        flip_lever: true,
                        central_afk_location: None,
                    },
                });
                commands.entity(*entity).insert(AutoTotem);

                if let Some(discord_bot) = &discord_bot_res {
                    if let Some(chat_relay_channel_id) = state.chat_relay_channel_id {
                        // Add `DiscordChatRelay` Component
                        commands.entity(*entity).insert(DiscordChatRelay {
                            channel_id: ChannelId::new(chat_relay_channel_id),
                        });
                    }

                    if let Some(channel_id) = state.channel_id
                        && let Some(http) = discord_bot.get_http()
                    {
                        // Add `DiscordChannelId` Component
                        commands.entity(*entity).insert(DiscordChannelId {
                            channel_id: ChannelId::new(channel_id),
                        });

                        // Send a Notification that bot has joined
                        let channel_id_clone = channel_id.clone();
                        let state_clone = state.clone();
                        tokio_runtime().spawn(async move {
                            if let Err(e) = http
                                .send_message(
                                    ChannelId::new(channel_id_clone),
                                    Vec::new(),
                                    &json!({
                                        "embeds": [
                                            {
                                                "title": "Bot Connected!",
                                                "description": format!("Username: {} \n Role: {:?}", &state_clone.username, &state_clone.role)
                                            }
                                        ]
                                    }),
                                )
                                .await
                            {
                                error!("Unable to send joined notification to discord, {:?}", e);
                            }
                        });
                    }
                } else if content == "You have lost connection to the server" {
                    commands.entity(*entity).remove::<InWorld>();
                    commands.entity(*entity).remove::<AntiAFK>();
                    commands.entity(*entity).remove::<AutoTotem>();
                    commands.entity(*entity).remove::<DiscordChatRelay>();
                    commands.entity(*entity).remove::<DiscordChannelId>();
                }
            }
        }
    }
}
