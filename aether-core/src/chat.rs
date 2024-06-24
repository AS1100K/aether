use crate::command::Command;
use crate::handle_command::handle_commands;
use crate::utils::parse_chat_content;
use azalea::chat::ChatPacket;
use azalea::Client;
use log::{info, warn};
use azalea_anti_afk::AntiAFKClientExt;
use azalea_anti_afk::config::AntiAFKConfig;
use azalea_discord::chat_bridge::DiscordChatBridgeExt;
use azalea_discord::{DiscordExt, SendDiscordMessage};
use crate::config::{Bot, Config};

pub async fn handle_chat(client: Client, chat: ChatPacket, mut state: Bot) -> anyhow::Result<()> {
    let (username, content, is_whisper) = parse_chat_content(&chat);

    if let Some(ref uname) = username {
        if uname == &state.username || !client.ecs.lock().resource::<Config>().members.contains(uname) {
            return Ok(());
        }
    } else if username.is_none() {
        if content.starts_with("Server") {
            warn!("{}", content);
        } else if content == "Connected to the server.".to_string() {
            info!("Connected to the Server, updating the state.");
            state.set_connection_state(true);

            if state.chat_bridge.is_some() {
                client.set_discord_chat_bridge(true, "2b2t Server", state.chat_bridge)
            } else if state.queue_bridge.is_some() {
                client.set_discord_chat_bridge(false, "", None)
            }

            let central_afk_location = if let Some(afk_location) = state.afk_location {
                Some(afk_location.to_vec3_floored())
            } else {
                None
            };

            let anti_afk_config = AntiAFKConfig {
                jump: true,
                sneak: true,
                walk: true,
                flip_lever: true,
                central_afk_location
            };

            client.set_anti_afk(true, Some(anti_afk_config));
        } else if content == "You have lost connection to the server" {
            {
                info!("Lost Connection to the server, back to queue");
                state.set_connection_state(false);
                if state.log_bridge.is_some() {
                    client.send_discord_message(SendDiscordMessage {
                        webhook: state.log_bridge.unwrap(),
                        contents: "Lost Connection to the server, back to queue. Aww".to_string(),
                        username: Some(state.username),
                        avatar_url: Some(format!("https://crafatar.com/avatars/{}", client.uuid())),
                    });
                }

                if state.queue_bridge.is_some() {
                    client.set_discord_chat_bridge(true, "2b2t Server", state.queue_bridge)
                } else if state.chat_bridge.is_some() {
                    client.set_discord_chat_bridge(false, "", None)
                }
            }
        }

        return Ok(());
    }

    if *state.is_connected.lock() && is_whisper {
        let command: Command = Command::parse(content.as_str(), &state).await;
        handle_commands(command, username.unwrap(), client, chat, state).await?;
    }

    return Ok(());
}
