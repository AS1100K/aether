#![feature(let_chains)]

mod config;
mod chat;
mod command;
mod handle_command;
mod commands;

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
use crate::chat::handle_chat;

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
        // Event::Chat(m) => {
        //     let (username, command) = m.split_sender_and_content();
        //
        //     if username.is_none() || username.unwrap() == state.config.username {
        //         return Ok(());
        //     }
        //
        //     println!("{}", command);
        //
        //     match command.as_str() {
        //         "!load" => {
        //             println!("Got Loading Command");
        //             let pearl_trapdoor = BlockPos::new(376, 71, 83);
        //             // let pearl_trapdoor = BlockPos::new(-19, 63, 25);
        //
        //             client.chat("On my way Sir!");
        //             client.goto(BlockPosGoal(pearl_trapdoor));
        //
        //             tokio::task::spawn(async move {
        //                 loop {
        //                     if distance(client.position(), pearl_trapdoor.to_vec3_floored()) <= 5.0
        //                     {
        //                         client.stop_pathfinding();
        //                         let load_packet = ServerboundUseItemOnPacket {
        //                             hand: InteractionHand::MainHand,
        //                             block_hit: BlockHit {
        //                                 block_pos: pearl_trapdoor,
        //                                 direction: Default::default(),
        //                                 location: pearl_trapdoor.to_vec3_floored(),
        //                                 inside: false,
        //                             },
        //                             sequence: 0,
        //                         };
        //
        //                         client.write_packet(load_packet.get()).unwrap();
        //
        //                         client.chat("Done Sir!");
        //                         break;
        //                     }
        //                 }
        //             });
        //
        //             return Ok(());
        //         }
        //         _ => {
        //             client.chat("IDK!");
        //             return Ok(());
        //         }
        //     }
        // }
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
