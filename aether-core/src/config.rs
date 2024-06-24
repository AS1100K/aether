use azalea::prelude::*;
use azalea::BlockPos;
use tracing::{error, warn};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::Arc;

#[derive(Clone, Debug, Resource)]
pub struct Config {
    pub server: String,
    pub members: Vec<String>,
    pub bots: HashMap<String, Bot>,
    pub log_bridge: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct RawConfig {
    server: String,
    members: Vec<String>,
    bots: Vec<RawBot>,
    log_bridge: Option<String>,
    version: u8,
}

#[derive(Serialize, Deserialize)]
struct RawLocation {
    owner: String,
    cords: [i32; 3],
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub enum Mode {
    #[default]
    Online,
    Offline,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub enum Role {
    #[default]
    Pearl,
    // Experimental Role
    AFKAdvanced,
}

#[derive(Serialize, Deserialize)]
struct RawBot {
    username: String,
    mode: Mode,
    email: Option<String>,
    render_distance: Option<u8>,
    role: Role,
    afk_location: Option<[i32; 3]>,
    pearl_locations: Option<Vec<RawLocation>>,
    chat_bridge: Option<String>,
    queue_bridge: Option<String>,
}

#[derive(Component, Clone, Default, Debug)]
pub struct Bot {
    pub username: String,
    pub mode: Mode,
    pub email: Option<String>,
    pub render_distance: Option<u8>,
    pub role: Role,
    pub afk_location: Option<BlockPos>,
    pub pearl_locations: Option<HashMap<String, BlockPos>>,
    pub chat_bridge: Option<String>,
    pub queue_bridge: Option<String>,
    pub is_connected: Arc<Mutex<bool>>,
}

impl Default for Config {
    fn default() -> Self {
        let contents: String = read_to_string("config.json").expect("Unable to load config.json");
        let raw_config: RawConfig =
            serde_json::from_str(contents.as_str()).expect("Unable to parse config.json");

        if raw_config.version != 2 {
            error!("This bot only support version 2 of `config.json`. Learn more at https://github.com/as1100k/aether")
        }

        let mut bots: HashMap<String, Bot> = Default::default();

        for raw_bots in raw_config.bots {
            if raw_bots.render_distance.is_some_and(|rd| rd > 32) {
                warn!("Render distance can't be greater than 32 chunks, default value of 5 chunks will be used");
            }

            match raw_bots.role {
                Role::Pearl => {
                    let afk_location = raw_bots
                        .afk_location
                        .expect("`afk_location` is required for `Pearl` role.");
                    let pearl_locations = raw_bots
                        .pearl_locations
                        .expect("`pearl_locations is required for `Pearl` role.`");

                    let afk_location_block_pos = BlockPos::from(BlockPosArray(afk_location));
                    let mut pearl_locations_hash_map: HashMap<String, BlockPos> =
                        Default::default();

                    for pearls in pearl_locations {
                        pearl_locations_hash_map
                            .insert(pearls.owner, BlockPos::from(BlockPosArray(pearls.cords)));
                    }

                    bots.insert(
                        raw_bots.username.to_owned(),
                        Bot {
                            username: raw_bots.username,
                            mode: raw_bots.mode,
                            email: raw_bots.email,
                            render_distance: raw_bots.render_distance,
                            role: raw_bots.role,
                            afk_location: Option::from(afk_location_block_pos),
                            pearl_locations: Option::from(pearl_locations_hash_map),
                            chat_bridge: raw_bots.chat_bridge,
                            queue_bridge: raw_bots.queue_bridge,
                            is_connected: Arc::new(Mutex::new(false)),
                        },
                    );
                }
                Role::AFKAdvanced => {
                    bots.insert(
                        raw_bots.username.to_owned(),
                        Bot {
                            username: raw_bots.username,
                            mode: raw_bots.mode,
                            email: raw_bots.email,
                            render_distance: raw_bots.render_distance,
                            role: raw_bots.role,
                            afk_location: None,
                            pearl_locations: None,
                            chat_bridge: raw_bots.chat_bridge,
                            queue_bridge: raw_bots.queue_bridge,
                            is_connected: Arc::new(Mutex::new(false)),
                        },
                    );
                }
            }
        }

        Self {
            server: raw_config.server,
            members: raw_config.members,
            bots,
            log_bridge: raw_config.log_bridge,
        }
    }
}

impl Bot {
    pub fn set_connection_state(&mut self, state: bool) {
        *self.is_connected.lock() = state;
    }
}

struct BlockPosArray([i32; 3]);

impl From<BlockPosArray> for BlockPos {
    fn from(value: BlockPosArray) -> Self {
        BlockPos::new(value.0[0], value.0[1], value.0[2])
    }
}
