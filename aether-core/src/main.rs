mod chat;
mod command;
mod commands;
mod config;
mod handle_command;
mod utils;

use crate::chat::handle_chat;
use azalea::{
    pathfinder::goals::BlockPosGoal,
    prelude::*,
    protocol::packets::game::{
        serverbound_interact_packet::InteractionHand,
        serverbound_use_item_on_packet::{BlockHit, ServerboundUseItemOnPacket},
    },
    BlockPos, Vec3,
};
use std::cmp::PartialEq;
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
}

async fn handle(client: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(chat) => handle_chat(client, chat, state).await?,
        Event::Disconnect(text) => {
            eprintln!("Got Disconnected because of: {:?}", text)
        }
        _ => {}
    }

    Ok(())
}

impl Default for State {
    fn default() -> Self {
        Self {
            config: Config::default(),
            ongoing_task: Arc::new(Mutex::new(false)),
        }
    }
}
