use crate::msg;
use azalea::Client;

pub async fn handle_unknown(username: String, client: Client) {
    msg!(
        client,
        username,
        "Unknown Command, use !help for more information."
    )
}

pub async fn handle_invalid_arguments(username: String, client: Client) {
    msg!(
        client,
        username,
        "Invalid Arguments, use !help <command> for more information"
    )
}
