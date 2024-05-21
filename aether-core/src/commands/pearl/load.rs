use azalea::Client;
use crate::State;

pub async fn handle_load(username: String, client: Client, state: State) {
    client.send_command_packet(format!("msg {} Teleporting...", username).as_str())
}