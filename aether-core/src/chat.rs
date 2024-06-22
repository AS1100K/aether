use crate::command::Command;
use crate::handle_command::handle_commands;
use crate::utils::parse_chat_content;
use azalea::chat::ChatPacket;
use azalea::Client;
use log::{info, warn};
use azalea_anti_afk::AntiAFKClientExt;
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
            client.set_anti_afk(true);
        } else if content == "You have lost connection to the server" {
            {
                info!("Lost Connection to the server, back to queue");
                state.set_connection_state(false);
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
