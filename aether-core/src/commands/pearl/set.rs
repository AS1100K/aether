use azalea::Client;
use crate::State;

pub async fn handle_set(username: String, client: Client, state: State, x: i32, y: i32, z: i32) {
    client.send_command_packet(format!("msg {} WIP...", username).as_str())
}