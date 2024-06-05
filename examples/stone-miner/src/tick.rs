use crate::utils::{direction, distance, mine};
use crate::State;
use azalea::{Client, Vec3, WalkDirection};
use log::{debug, info, trace};
use std::time::Duration;

async fn next_checkpoint(client: &mut Client, next_point: u8, state: &State) -> anyhow::Result<()> {
    debug!("Trying to reach next_point: {}", next_point);
    let current_position: Vec3 = client.position();
    let next_checkpoint = state.checkpoints.lock()[next_point as usize];
    let next_checkpoint_vec: Vec3 =
        Vec3::new(next_checkpoint[0], next_checkpoint[1], next_checkpoint[2]);

    let y_rot = direction(next_checkpoint_vec, current_position)
        .await
        .unwrap();
    trace!("y_rot: {}", y_rot);

    client.set_direction(y_rot as f32, -90.0);
    client.walk(WalkDirection::Forward);
    tokio::time::sleep(Duration::from_millis(35)).await;
    client.walk(WalkDirection::None);

    mine(client, state.y_start, state.y_end)
        .await
        .expect("Unable to mine the blocks.");

    let dist = distance(next_checkpoint_vec, current_position)
        .await
        .unwrap();
    trace!("distance is: {}", dist);

    if dist <= 0.5 {
        trace!("Distance less than 0.5, updating last_checkpoint to {}", next_point);
        {
            let mut last_checkpoint = state.last_checkpoint.lock();
            *last_checkpoint = next_point;
        }
        trace!("Updated to next_point: {}", next_point)
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

    let mut next_point = 1;
    {
        next_point = match *state.last_checkpoint.lock() {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 0,
            _ => 0,
        };
    }
    next_checkpoint(&mut client, next_point, &state).await?;
    Ok(())
}
