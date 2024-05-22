mod chat;
mod client;
mod command;
mod commands;
mod config;
mod handle_command;
mod utils;

use crate::chat::handle_chat;
use crate::client::{handle_death, handle_init};

use azalea::prelude::*;
use std::sync::{Arc, Mutex};

use crate::config::{Config, Mode};

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

    ClientBuilder::new()
        .set_handler(handle)
        // .set_state(State { config })
        .start(account, server_url.as_str())
        .await
        .unwrap();
}

#[derive(Clone, Component)]
pub struct State {
    config: Config,
    ongoing_task: Arc<Mutex<bool>>,
    is_connected: Arc<Mutex<bool>>
}

async fn handle(client: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(chat) => handle_chat(client, chat, state).await?,
        Event::Init => handle_init(client, state).await?,
        // Event::Disconnect(text) => handle_disconnect(client, state, text).await?,
        Event::Death(death) => handle_death(client, state, death).await?,
        _ => {}
    }

    Ok(())
}

impl Default for State {
    fn default() -> Self {
        Self {
            config: Config::default(),
            ongoing_task: Arc::new(Mutex::new(false)),
            is_connected: Arc::new(Mutex::new(false))
        }
    }
}
