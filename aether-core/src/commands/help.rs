use crate::msg;
use azalea::Client;

pub async fn handle_help(username: String, client: Client) {
    msg!(
        client,
        username,
        "Help Command is a TODO\n but it will be done soon."
    )
}
