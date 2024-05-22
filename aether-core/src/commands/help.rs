use crate::msg;
use azalea::Client;
use log::info;

pub async fn handle_help(username: String, client: Client) {
    info!("Received help command from {}", username);
    msg!(
        client,
        username,
        "Hi Welcome to Project Aether - Minecraft bot for 2b2t build by AS1100K."
    );

    msg!(client, username, "Currently, aether is under development, so if you find any bugs fell free to contact AS1100K.");

    msg!(
        client,
        username,
        "!pearl load -> This will load your pearl by flicking the trapdoor."
    )
}
