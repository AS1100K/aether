use crate::{msg, State};
use azalea::Client;

pub async fn handle_set(username: String, client: Client, _state: State, _x: i32, _y: i32, _z: i32) {
    msg!(
        client,
        username,
        "This command will be available by the next release."
    )
}
