use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub username: String,
    pub server: String,
    pub mode: Mode,
    pub email: Option<String>,
    pub checkpoints: [Checkpoint; 4]
}

pub type Checkpoint = [f64; 3];

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
    Online,
    Offline,
}

impl Default for Config {
    fn default() -> Self {
        let contents: String = read_to_string("config.json").expect("Unable to load config.json");
        let config: Config =
            serde_json::from_str(&contents.as_str()).expect("Unable to parse config.json");

        config
    }
}
