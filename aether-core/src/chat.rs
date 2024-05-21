use crate::command::Command;
use crate::handle_command::handle_commands;
use crate::State;
use azalea::chat::ChatPacket;
use azalea::Client;

pub async fn handle_chat(client: Client, chat: ChatPacket, state: State) -> anyhow::Result<()> {
    let (username, content) = chat.split_sender_and_content();

    if let Some(ref uname) = username {
        if uname == &state.config.username || !state.config.members.contains(uname) {
            return Ok(());
        }
    } else if username.is_none() {
        if content.starts_with("Server") {
            for member in state.config.members {
                client.send_command_packet(format!("msg {} {}", member, content).as_str())
            }
        }

        return Ok(());
    }

    if chat.is_whisper() {
        let command: Command = Command::parse(content.as_str()).await;
        handle_commands(command, username.unwrap(), client, chat, state).await?;
    }

    return Ok(());
}
