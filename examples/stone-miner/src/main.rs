mod config;
mod tick;
mod utils;

use std::sync::Arc;
use tokio::sync::Mutex;
use log::{debug, info};
use crate::config::{Checkpoint, Config, Mode};
use azalea::prelude::*;
use crate::tick::handle_tick;

#[tokio::main]
async fn main() {
    info!("Loading Config");

    let config = Config::default();

    info!("Loaded config.json");
    debug!("```config = {:?}```", config);

    let account: Account = if config.mode == Mode::Offline {
        Account::offline(&config.username.as_str())
    } else {
        Account::microsoft(&config.email.unwrap().as_str()).await.expect("Unable to login via Microsoft")
    };

    let state: State = State {
        last_checkpoint: Arc::new(Mutex::new(0)),
        checkpoints: Arc::new(Mutex::new(config.checkpoints))
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
    last_checkpoint: Arc<Mutex<u8>>,
    checkpoints: Arc<Mutex<[Checkpoint; 4]>>
}

async fn handle(client: Client, event: Event, state: State) -> anyhow::Result<()> {
    if let Event::Tick = event {
        handle_tick(client, state).await?
    }
    Ok(())
}