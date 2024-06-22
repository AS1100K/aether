use crate::commands::*;

use crate::command::Command;
use azalea::chat::ChatPacket;
use azalea::Client;
use crate::config::Bot;

pub async fn handle_commands(
    command: Command,
    username: String,
    client: Client,
    _chat: ChatPacket,
    state: Bot,
) -> anyhow::Result<()> {
    match command {
        Command::PearlLoad => pearl::load::handle_load(username, client, state).await,
        Command::PearlSet(x, y, z) => {
            pearl::set::handle_set(username, client, state, x, y, z).await
        }
        Command::Help => help::handle_help(username, client).await,
        Command::InvalidArguments => error::handle_invalid_arguments(username, client).await,
        Command::Unknown => error::handle_unknown(username, client).await,
    }

    Ok(())
}
