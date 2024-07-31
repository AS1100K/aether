use crate::utils::distance;
use crate::State;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::PathfinderClientExt;
use azalea::{BlockPos, Client, Vec3};
use log::info;
use std::time::Duration;
#[cfg(feature = "craftmc-survial")]
use azalea::inventory::{ContainerClickEvent, InventoryComponent};
#[cfg(feature = "craftmc-survial")]
use azalea::inventory::operations::{ClickOperation, PickupClick};
#[cfg(feature = "craftmc-survial")]
use azalea::protocol::packets::game::serverbound_container_click_packet::ServerboundContainerClickPacket;
#[cfg(feature = "craftmc-survial")]
use azalea::protocol::packets::game::serverbound_interact_packet::InteractionHand;
#[cfg(feature = "craftmc-survial")]
use azalea::protocol::packets::game::serverbound_use_item_packet::ServerboundUseItemPacket;

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

        let inventory_component_option = client.get_entity_component::<InventoryComponent>(client.entity);

        if let Some(inventory_component) = inventory_component_option {
            // Clicking on 3rd slot
            client.ecs.lock().send_event(ContainerClickEvent {
                entity: client.entity,
                window_id: inventory_component.id,
                operation: ClickOperation::Pickup(PickupClick::Left {
                    slot: Some(3),
                })
            });

            tokio::time::sleep(Duration::from_secs(5)).await;

            // Click on 11th slot
            client.ecs.lock().send_event(ContainerClickEvent {
                entity: client.entity,
                window_id: inventory_component.id,
                operation: ClickOperation::Pickup(PickupClick::Left {
                    slot: Some(11),
                })
            });
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
                    client.left_click_mine(true);
                }
                break;
            }
        }
    });

    Ok(())
}
