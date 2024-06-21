pub enum Command {
    PearlLoad,
    PearlSet(i32, i32, i32),
    Help,
    Unknown,
    InvalidArguments,
}

impl Command {
    pub async fn parse(input: &str) -> Command {
        let parts: Vec<&str> = input.split_whitespace().collect();

        // TODO: Port all the commands to discord and only support necessary ones via minecraft chat
        match parts.as_slice() {
            ["!pearl", "load"] => Command::PearlLoad,
            ["!pearl", "set", x, y, z] => {
                let x = x.parse::<i32>();
                let y = y.parse::<i32>();
                let z = z.parse::<i32>();

                if x.is_ok() && y.is_ok() && z.is_ok() {
                    Command::PearlSet(x.unwrap(), y.unwrap(), z.unwrap())
                } else {
                    Command::InvalidArguments
                }
            }
            ["!help"] => Command::Help,
            _ => Command::Unknown,
        }
    }
}
