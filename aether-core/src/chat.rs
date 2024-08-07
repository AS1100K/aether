use std::time::Instant;

use azalea::app::{App, Plugin};
use azalea::chat::{ChatReceivedEvent, SendChatEvent};
use azalea::entity::metadata::Player;
use azalea::entity::LocalEntity;
use azalea::prelude::*;
use azalea::ecs::prelude::*;
use azalea_anti_afk::config::AntiAFKConfig;
use azalea_anti_afk::AntiAFK;
use azalea_utility::auto_eat::{StartAutoEat, StopAutoEat};
use azalea_utility::auto_totem::AutoTotem;

use crate::command::AetherCommand;
use crate::commands::pearl::load::LoadPearl;
use crate::config::{Bot, Config};
use crate::utils::{parse_chat_content, InWorld};

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(GameTick, handle_chat);
    }
}

#[allow(clippy::complexity)]
fn handle_chat(
    mut events: EventReader<ChatReceivedEvent>,
    query: Query<(&Bot, Option<&InWorld>), (With<Player>, With<LocalEntity>)>,
    config: Res<Config>,
    mut commands: Commands,
    mut send_chat_event: EventWriter<SendChatEvent>,
    mut load_pearl: EventWriter<LoadPearl>,
    mut start_auto_eat: EventWriter<StartAutoEat>,
    mut stop_auto_eat: EventWriter<StopAutoEat>
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
                start_auto_eat.send(StartAutoEat {
                    use_inventory: true
                });
            } else if content == "You have lost connection to the server" {
                commands.entity(*entity).remove::<InWorld>();
                commands.entity(*entity).remove::<AntiAFK>();
                commands.entity(*entity).remove::<AutoTotem>();
                stop_auto_eat.send(StopAutoEat);
            }
        }
    }
}
