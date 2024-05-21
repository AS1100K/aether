use azalea::Client;
use crate::State;
use crate::msg;

pub async fn handle_load(username: String, client: Client, state: State) {
    msg!(client, username, "Teleporting...");
}