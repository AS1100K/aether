use crate::utils::distance;
use crate::State;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::PathfinderClientExt;
use azalea::{BlockPos, Client, Vec3};
use log::{error, info};
use std::time::Duration;
use azalea::inventory::operations::{ClickOperation, PickupClick};
use azalea::prelude::*;
use azalea::protocol::packets::game::serverbound_container_click_packet::ServerboundContainerClickPacket;
use azalea::protocol::packets::game::serverbound_interact_packet::InteractionHand;
use azalea::protocol::packets::game::serverbound_use_item_packet::ServerboundUseItemPacket;
use azalea_auto_mine::AutoMineExt;

pub async fn handle_login(mut client: Client, state: State) -> anyhow::Result<()> {
    #[cfg(feature = "login")]
    {
        info!("Logging into the server");
        client.send_command_packet(format!("login {}", state.password).as_str());
        info!("Logged into the server");
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    #[cfg(feature = "craftmc-survial")]
    {
        info!("Navigating through the server.");

        info!("Sending Use Item Packet");
        client.write_packet(ServerboundUseItemPacket {
            hand: InteractionHand::MainHand,
            sequence: 1,
        }.get()).expect("Unable to send Use Item Packet");
        tokio::time::sleep(Duration::from_secs(1)).await;

        let container_option = client.get_open_container();
        if let Some(container) = container_option {
            info!("Clicking 3rd slot");
            container.click(ClickOperation::Pickup(PickupClick::Left {
                slot: Some(3),
            }));
        } else {
            error!("Unable to click the 3rd slot");
        }

        info!("Clicking 11th slot");
        let container_option = client.get_open_container();
        if let Some(container) = container_option {
            container.click(ClickOperation::Pickup(PickupClick::Left {
                slot: Some(11),
            }));
        } else {
            error!("Unable to click the 11th slot");
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    info!("Moving to the first checkpoint");
    let first_checkpoint = state.checkpoints[0];
    let first_checkpoint_block_pos = BlockPos::new(
        first_checkpoint[0] as i32,
        first_checkpoint[1] as i32,
        first_checkpoint[2] as i32,
    );
    client.goto(BlockPosGoal(first_checkpoint_block_pos));

    tokio::task::spawn(async move {
        let first_checkpoint_vec = first_checkpoint_block_pos.to_vec3_floored();
        loop {
            let mut current_position = client.position();
            current_position = Vec3::new(
                current_position.x.floor(),
                current_position.y.floor(),
                current_position.z.floor(),
            );

            let d = distance(first_checkpoint_vec, current_position)
                .await
                .expect("Unable to calculate distance");

            if d <= 1.0 {
                info!("Changed the state value");
                client.stop_pathfinding();

                // Just to be safe...
                tokio::time::sleep(Duration::from_millis(500)).await;
                {
                    client.set_direction(state.initial_y_rot, -90.0);
                    *state.at_checkpoint.lock() = true;
                    client.auto_mine(true);
                }
                break;
            }
        }
    });

    Ok(())
}
