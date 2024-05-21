use azalea::Client;

pub async fn handle_unknown(username: String, client: Client) {
    client.send_command_packet(format!("msg {} Unknown Command, use !help for more information.", username).as_str())
}

pub async fn handle_invalid_arguments(username: String, client: Client) {
    client.send_command_packet(format!("msg {} Invalid Arguments, use !help <command> for more information", username).as_str())
}