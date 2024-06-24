use azalea::Client;
use tracing::warn;

pub async fn handle_unknown(username: String, _client: Client) {
    warn!(
        "{} Unknown Command, use !help for more information.",
        username
    )
}

pub async fn handle_invalid_arguments(username: String, _client: Client) {
    warn!(
        "{} Invalid Arguments, use !help <command> for more information",
        username
    )
}
