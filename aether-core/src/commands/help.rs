use azalea::chat::ChatPacket;
use azalea::Client;

pub async fn handle_help(username: String, client: Client) {
    client.send_command_packet(format!("msg {} Help Command is a TODO\n but it will be done soon.", username).as_str())
}