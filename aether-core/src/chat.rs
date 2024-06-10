use crate::command::Command;
use crate::handle_command::handle_commands;
use crate::{msg, State};
use azalea::chat::ChatPacket;
use azalea::Client;
use log::info;
use crate::utils::parse_chat_content;

pub async fn handle_chat(client: Client, chat: ChatPacket, state: State) -> anyhow::Result<()> {
    println!("{:?}", parse_chat_content(&chat));
    let (username, content, is_whisper) = parse_chat_content(&chat);

    if let Some(ref uname) = username {
        if uname == &state.config.username || !state.config.members.contains(uname) {
            return Ok(());
        }
    } else if username.is_none() {
        if content.starts_with("Server") {
            for member in state.config.members {
                msg!(client, member, content);
            }
        } else if content == "Connected to the server.".to_string() {
            {
                info!("Connected to the Server, updating the state.");
                let mut is_connected = state.game_information.is_connected.lock();
                *is_connected = true;
            }
        }

        return Ok(());
    }

    if *state.game_information.is_connected.lock() && is_whisper {
        let command: Command = Command::parse(content.as_str()).await;
        handle_commands(command, username.unwrap(), client, chat, state).await?;
    }

    return Ok(());
}
