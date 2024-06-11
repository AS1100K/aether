use crate::config::Config;
use azalea::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone, Component, Resource, Debug)]
pub struct State {
    pub config: Config,
    pub client_information: ClientInformation,
    pub game_information: GameInformation,
}

#[derive(Clone, Component, Resource, Debug)]
pub struct ClientInformation {
    pub ongoing_task: Arc<Mutex<bool>>,
    pub is_afk: Arc<Mutex<bool>>,
}

#[derive(Clone, Component, Resource, Debug)]
pub struct GameInformation {
    pub last_afk_tick: Arc<Mutex<Instant>>,
    pub is_connected: Arc<Mutex<bool>>,
}

impl Default for GameInformation {
    fn default() -> Self {
        Self {
            last_afk_tick: Arc::new(Mutex::new(Instant::now())),
            is_connected: Arc::new(Mutex::new(false)),
        }
    }
}

impl GameInformation {
    pub fn set_connection_state(&mut self, connection_state: bool) {
        *self.is_connected.lock() = connection_state;
    }
}

impl Default for ClientInformation {
    fn default() -> Self {
        Self {
            ongoing_task: Arc::new(Mutex::new(false)),
            is_afk: Arc::new(Mutex::new(true)),
        }
    }
}

impl ClientInformation {
    pub fn set_afk(&mut self, afk_state: bool) {
        *self.is_afk.lock() = afk_state;
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            config: Config::default(),
            client_information: ClientInformation::default(),
            game_information: GameInformation::default(),
        }
    }
}
