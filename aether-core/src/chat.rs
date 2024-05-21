use azalea::chat::ChatPacket;
use azalea::Client;
use crate::State;

pub async fn handle_chat(client: Client, chat: ChatPacket, state: State) -> anyhow::Result<()> {
    let (username, content) = chat.split_sender_and_content();

    if let Some(ref uname) = username {
        if uname == &state.config.username || !state.config.members.contains(uname) {
            return Ok(())
        }
    } else if username.is_none() {
        if content.starts_with("Server") {
            for member in state.config.members {
                client.send_command_packet(format!("msg {} {}", member, content).as_str())
            }
        }

        return Ok(())
    } else if content.starts_with("!") {
        // Parse the command
    }



    return Ok(())
}