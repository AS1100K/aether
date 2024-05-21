#![feature(let_chains)]

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

#[derive(Default, Clone, Component)]
pub struct State {
    config: Config,
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

fn distance(position: Vec3, destination: Vec3) -> f64 {
    let x: f64 = f64::powi(position.x - destination.x, 2);
    let y: f64 = f64::powi(position.y - destination.y, 2);
    let z: f64 = f64::powi(position.z - destination.z, 2);

    let d: f64 = f64::powf(x + y + z, 0.5);
    return d;
}
