use crate::config::{Bot, Role};

pub enum Command {
    PearlLoad,
    PearlSet(i32, i32, i32),
    Help,
    Unknown,
    InvalidArguments,
}

impl Command {
    pub async fn parse(input: &str, state: &Bot) -> Command {
        let parts: Vec<&str> = input.split_whitespace().collect();

        // TODO: Port all the commands to discord and only support necessary ones via minecraft chat
        match parts.as_slice() {
            ["!pearl", "load"] => {
                if state.role == Role::Pearl {
                    Command::PearlLoad
                } else {
                    Command::Unknown
                }
            }
            ["!pearl", "set", x, y, z] => {
                if state.role == Role::Pearl {
                    let x = x.parse::<i32>();
                    let y = y.parse::<i32>();
                    let z = z.parse::<i32>();

                    #[allow(clippy::unnecessary_unwrap)]
                    if x.is_ok() && y.is_ok() && z.is_ok() {
                        Command::PearlSet(x.unwrap(), y.unwrap(), z.unwrap())
                    } else {
                        Command::InvalidArguments
                    }
                } else {
                    Command::Unknown
                }
            }
            ["!help"] => Command::Help,
            _ => Command::Unknown,
        }
    }
}
