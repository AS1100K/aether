mod config;
mod tick;
#[cfg(feature = "trial")]
mod trial;
mod utils;
mod login;

use crate::config::{Checkpoint, Config, Mode, WalkDir};
use crate::tick::handle_tick;
use azalea::prelude::*;
use log::{debug, info};
use parking_lot::Mutex;
use std::sync::Arc;
use crate::login::handle_login;

#[tokio::main]
async fn main() {
    info!("Loading Config");

    let config = Config::default();

    info!("Loaded config.json");
    debug!("```config = {:?}```", config);

    let account: Account = if config.mode == Mode::Offline {
        Account::offline(&config.username.as_str())
    } else {
        Account::microsoft(&config.email.unwrap().as_str())
            .await
            .expect("Unable to login via Microsoft")
    };

    let state: State = State {
        at_checkpoint: Arc::new(Mutex::new(false)),
        last_checkpoint: Arc::new(Mutex::new(0)),
        checkpoints: config.checkpoints,
        directions: config.directions,
        initial_y_rot: config.initial_y_rot,
        y_start: config.y_start,
        y_end: config.y_end,
    };

    ClientBuilder::new()
        .set_handler(handle)
        .set_state(state)
        .start(account, config.server.as_str())
        .await
        .expect("Unable to start the bot.");
}

#[derive(Default, Component, Clone, Debug)]
pub struct State {
    at_checkpoint: Arc<Mutex<bool>>,
    last_checkpoint: Arc<Mutex<u8>>,
    checkpoints: [Checkpoint; 4],
    directions: [WalkDir; 4],
    initial_y_rot: f32,
    y_start: i32,
    y_end: i32,
}

async fn handle(client: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Tick => handle_tick(client, state).await?,
        Event::Login => handle_login(client, state).await?,
        _ => {}
    }
    Ok(())
}
