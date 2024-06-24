use crate::utils::distance;
use crate::State;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::PathfinderClientExt;
use azalea::{BlockPos, Client, Vec3};
use log::info;
use std::time::Duration;

pub async fn handle_login(mut client: Client, state: State) -> anyhow::Result<()> {
    #[cfg(feature = "login")]
    {
        info!("Logging into the server");
        client.send_command_packet(format!("login {}", state.password).as_str());
        info!("Logged into the server");
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
