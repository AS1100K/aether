use crate::utils::{distance, mine};
use crate::State;
use azalea::{BlockPos, BotClientExt, Client, Vec3, WalkDirection};
use log::{debug, info, trace};
use std::time::Duration;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::PathfinderClientExt;
use crate::config::WalkDir;

async fn next_checkpoint(client: &mut Client, next_point: u8, state: &State) -> anyhow::Result<()> {
    #[cfg(feature = "sell")]
    if next_point == 3 {
        *state.loop_counter.lock() += 1;
    }

    debug!("Trying to reach next_point: {}", next_point);
    let current_position: Vec3 = client.position();
    let next_checkpoint = state.checkpoints[next_point as usize];
    let next_checkpoint_vec: Vec3 =
        Vec3::new(next_checkpoint[0], next_checkpoint[1], next_checkpoint[2]);

    client.walk(state.directions[next_point as usize].to_azalea_walk_direction());
    tokio::time::sleep(Duration::from_millis(45)).await;
    client.walk(WalkDirection::None);

    mine(client, state.y_start, state.y_end)
        .await
        .expect("Unable to mine the blocks.");

    let dist = distance(next_checkpoint_vec, current_position)
        .await
        .unwrap();
    trace!("distance is: {}", dist);

    if dist <= 2.0 {
        trace!(
            "Distance less than 2.0, updating last_checkpoint to {}",
            next_point
        );
        {
            let mut last_checkpoint = state.last_checkpoint.lock();
            *last_checkpoint = next_point;
        }
        trace!("Updated to next_point: {}", next_point)
    }

    #[cfg(feature = "sell")]
    // Sell item every 5 or 6 loops, 250 is there to account for error.
    if *state.loop_counter.lock() >= 250 {
        info!("Selling All Items");
        client.send_command_packet("sellall");
        tokio::time::sleep(Duration::from_millis(500)).await;
        client.send_command_packet("naprawkilof");
        info!("Sold All Items");
        *state.loop_counter.lock() = 0;
    }

    Ok(())
}

pub async fn handle_tick(mut client: Client, state: State) -> anyhow::Result<()> {
    #[cfg(feature = "trial")]
    {
        use crate::trial::is_trial_over;
        if is_trial_over() {
            client.disconnect();
            panic!("Your Trial has been over. Contact AS1100K to get the full version.");
        }
    }

    let mut next_point;
    let mut at_checkpoint;
    {
        next_point = match *state.last_checkpoint.lock() {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 0,
            _ => 0,
        };

        at_checkpoint = *state.at_checkpoint.lock();
    }

    if at_checkpoint {
        next_checkpoint(&mut client, next_point, &state).await?;
    }

    Ok(())
}
