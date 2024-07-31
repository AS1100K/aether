use crate::msg;
use azalea::Client;
use tracing::info;

pub async fn handle_help(username: String, client: Client) {
    info!("Got Help Command from {}", username);

    msg!(
        client,
        username,
        "This Bot is powered by Project Aether by AS1100K. Supported commands: `!pearl load`, `!pearl set x y z`,`!help`"
    )
}
