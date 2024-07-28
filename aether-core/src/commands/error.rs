use crate::msg;
use azalea::Client;
use tracing::warn;

pub async fn handle_unknown(username: String, client: Client) {
    warn!("Got Unknown Command from {}", username);
    msg!(
        client,
        "{} Unknown Command, use !help for more information.",
        username
    )
}

pub async fn handle_invalid_arguments(username: String, client: Client) {
    warn!("Got Invalid Command Arguments from {}", username);
    msg!(
        client,
        "{} Invalid Arguments, use !help <command> for more information",
        username
    )
}
