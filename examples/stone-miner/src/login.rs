use azalea::{BlockPos, BotClientExt, Client, Vec3};
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::PathfinderClientExt;
use log::info;
use crate::State;
use crate::utils::distance;

pub async fn handle_login(mut client: Client, state: State) -> anyhow::Result<()> {
    info!("Moving to the first checkpoint");
    let first_checkpoint = state.checkpoints[0];
    let first_checkpoint_block_pos = BlockPos::new(first_checkpoint[0] as i32, first_checkpoint[1] as i32, first_checkpoint[2] as i32);
    client.goto(BlockPosGoal(first_checkpoint_block_pos));

    tokio::task::spawn(async move {
        let first_checkpoint_vec = first_checkpoint_block_pos.to_vec3_floored();
        loop {
            let d = distance(first_checkpoint_vec, client.position()).await.expect("Unable to calculate distance");

            if d <= 1.0 {
                info!("Changed the state value");
                client.stop_pathfinding();
                // let second_checkpoint = state.checkpoints[1];
                // let second_checkpoint_vec = Vec3::new(second_checkpoint[0], second_checkpoint[1], second_checkpoint[2]);
                // client.look_at(second_checkpoint_vec);
                client.set_direction(state.initial_y_rot, -90.0);
                *state.at_checkpoint.lock() = true;
                break
            }
        }
    });

    Ok(())
}