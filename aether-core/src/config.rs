use azalea::BlockPos;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone)]
pub struct Config {
    pub username: String,
    pub server: String,
    pub mode: Mode,
    pub email: Option<String>,
    pub role: Role,
    pub afk_location: BlockPos,
    pub members: Vec<String>,
    pub pearl_locations: HashMap<String, BlockPos>,
}

#[derive(Serialize, Deserialize)]
struct RawConfig {
    username: String,
    server: String,
    mode: Mode,
    email: Option<String>,
    role: Role,
    afk_location: [i32; 3],
    members: Vec<String>,
    pearl_locations: Vec<RawLocation>,
}

#[derive(Serialize, Deserialize)]
struct RawLocation {
    owner: String,
    cords: [i32; 3],
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Mode {
    Online,
    Offline,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Role {
    Pearl,
}

impl Default for Config {
    fn default() -> Self {
        let contents: String = read_to_string("config.json").expect("Unable to load config.json");
        let raw_config: RawConfig =
            serde_json::from_str(&contents.as_str()).expect("Unable to parse config.json");

        let mut pearl_locations: HashMap<String, BlockPos> = HashMap::new();
        for location in raw_config.pearl_locations {
            pearl_locations.insert(
                location.owner,
                BlockPos::new(location.cords[0], location.cords[1], location.cords[2]),
            );
        }

        Self {
            username: raw_config.username,
            server: raw_config.server,
            mode: raw_config.mode,
            email: raw_config.email,
            role: raw_config.role,
            afk_location: BlockPos::new(
                raw_config.afk_location[0],
                raw_config.afk_location[1],
                raw_config.afk_location[2],
            ),
            members: raw_config.members,
            pearl_locations,
        }
    }
}
