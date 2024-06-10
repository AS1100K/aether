mod config;
mod tick;
#[cfg(feature = "trial")]
mod trial;
mod utils;
mod login;

use crate::config::{Checkpoint, Config, Mode, WalkDir};
use crate::tick::handle_tick;
use azalea::prelude::*;
#[cfg(feature = "auto-reconnect")]
use azalea::swarm::prelude::*;
#[cfg(feature = "auto-reconnect")]
use std::time::Duration;
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

    #[cfg(feature = "auto-reconnect")]
    SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .add_account(account)
        .start(config.server.as_str())
        .await
        .expect("Unable to start the Swarm");

    #[cfg(not(feature = "auto-reconnect"))]
    ClientBuilder::new()
        .set_handler(handle)
        .start(account, config.server.as_str())
        .await
        .expect("Unable to start the bot.");

}

#[derive(Resource, Component, Clone, Debug)]
pub struct State {
    #[cfg(feature = "login")]
    password: String,
    #[cfg(feature = "sell")]
    loop_counter: Arc<Mutex<u8>>,
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

#[cfg(feature = "auto-reconnect")]
async fn swarm_handle(mut swarm: Swarm, event: SwarmEvent, _state: State) -> anyhow::Result<()> {
    match event {
        SwarmEvent::Disconnect(account, _join_opts) => {
            info!("Got disconnected from the server. Reconnecting...");

            tokio::time::sleep(Duration::from_secs(5)).await;
            swarm.add(&*account, State::default()).await?;
        }
        _ => {}
    }

    Ok(())
}

impl Default for State {
    fn default() -> Self {
        let config = Config::default();

        Self {
            #[cfg(feature = "login")]
            password: config.password,
            #[cfg(feature = "sell")]
            loop_counter: Arc::new(Mutex::new(0)),
            at_checkpoint: Arc::new(Mutex::new(false)),
            last_checkpoint: Arc::new(Mutex::new(0)),
            checkpoints: config.checkpoints,
            directions: config.directions,
            initial_y_rot: config.initial_y_rot,
            y_start: config.y_start,
            y_end: config.y_end,
        }
    }
}