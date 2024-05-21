#[macro_export]
macro_rules! msg {
    ($client: expr, $username: expr, $message: expr) => {
        $client.send_command_packet(format!("msg {} {}", $username, $message).as_str());
    };
}