use crate::State;
use azalea::Client;
use log::warn;

pub async fn handle_set(
    username: String,
    _client: Client,
    _state: State,
    _x: i32,
    _y: i32,
    _z: i32,
) {
    // TODO: Integrate this command with discord
    warn!("{} This command will be available by the next release.", username)
}
