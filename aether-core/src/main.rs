mod chat;
mod client;
mod command;
mod commands;
mod config;
mod handle_command;
mod state;
mod utils;

use crate::chat::handle_chat;
use crate::client::{handle_death, handle_init};
use std::time::Duration;

use azalea::{prelude::*, swarm::prelude::*};
use log::info;
use azalea_anti_afk::AntiAFKPlugin;
use azalea_task_manager::TaskManagerPlugin;

use crate::config::{Config, Mode};
use crate::state::State;

#[tokio::main]
async fn main() {
    let config: Config = Config::default();

    let account: Account = if config.mode == Mode::Offline {
        Account::offline(&config.username.as_str())
    } else {
        Account::microsoft(&config.email.clone().unwrap().as_str())
            .await
            .expect("Unable to login via microsoft.")
    };

    let server_url: String = config.server.clone();

    SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .add_plugins(azalea_viaversion::ViaVersionPlugin::start("1.20.6").await)
        .add_plugins(AntiAFKPlugin)
        .add_plugins(TaskManagerPlugin)
        .add_account(account)
        .join_delay(Duration::from_secs(3))
        .start(server_url.as_str())
        .await
        .unwrap();
}

async fn handle(client: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(chat) => handle_chat(client, chat, state).await?,
        Event::Init => handle_init(client, state).await?,
        Event::Death(death) => handle_death(client, state, death).await?,
        _ => {}
    }

    Ok(())
}

async fn swarm_handle(mut swarm: Swarm, event: SwarmEvent, state: State) -> anyhow::Result<()> {
    match event {
        SwarmEvent::Disconnect(account, _join_opts) => {
            info!("Got disconnected from the server. Reconnecting...");
            *state.game_information.is_connected.lock() = false;
            info!("Changed Game Information - is connected to false");

            tokio::time::sleep(Duration::from_secs(5)).await;
            swarm.add(&*account, State::default()).await?;
        }
        _ => {}
    }

    Ok(())
}
