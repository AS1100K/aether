use crate::config::Bot;
use azalea::Client;
use tracing::warn;

pub async fn handle_set(username: String, _client: Client, _state: Bot, _x: i32, _y: i32, _z: i32) {
    // TODO: Integrate this command with discord
    warn!(
        "{} This command will be available by the next release.",
        username
    )
}
