use crate::config::Bot;
use azalea::Client;
use crate::msg;

pub async fn handle_set(username: String, client: Client, _state: Bot, _x: i32, _y: i32, _z: i32) {
    msg!(
        client,
        username,
        "This command is currently not supported. Check out https://github.com/as1100k/aether"
    )
}
