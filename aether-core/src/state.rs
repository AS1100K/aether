use crate::config::Config;
use azalea::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Clone, Component, Resource, Debug)]
pub struct State {
    pub config: Config,
    pub game_information: GameInformation,
}

#[derive(Clone, Component, Resource, Debug)]
pub struct GameInformation {
    pub is_connected: Arc<Mutex<bool>>,
}

impl Default for GameInformation {
    fn default() -> Self {
        Self {
            is_connected: Arc::new(Mutex::new(false)),
        }
    }
}

impl GameInformation {
    pub fn set_connection_state(&mut self, connection_state: bool) {
        *self.is_connected.lock() = connection_state;
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            config: Config::default(),
            game_information: GameInformation::default(),
        }
    }
}
