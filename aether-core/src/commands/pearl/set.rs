use crate::{msg, State};
use azalea::Client;

pub async fn handle_set(username: String, client: Client, state: State, x: i32, y: i32, z: i32) {
    msg!(client, username, "WIP...")
}
