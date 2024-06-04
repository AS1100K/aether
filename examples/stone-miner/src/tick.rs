use std::time::Duration;
use azalea::{Client, Vec3, WalkDirection};
use log::{info, trace};
use crate::State;
use crate::utils::{direction, distance};

async fn next_checkpoint(mut client: Client, next_point: u8, state: State) -> anyhow::Result<()> {
    info!("Trying to reach next_point: {}", next_point);
    let current_position: Vec3 = client.position();
    let next_checkpoint = state.checkpoints.lock().await[next_point as usize];
    let next_checkpoint_vec: Vec3 = Vec3::new(next_checkpoint[0], next_checkpoint[1], next_checkpoint[2]);

    let y_rot = direction(next_checkpoint_vec, current_position).await.unwrap();
    trace!("y_rot: {}", y_rot);

    client.set_direction(y_rot as f32, -71.6_f32);
    client.walk(WalkDirection::Forward);
    tokio::time::sleep(Duration::from_millis(45)).await;
    client.walk(WalkDirection::None);

    let dist = distance(next_checkpoint_vec, current_position).await.unwrap();
    trace!("distance is: {}", dist);

    if dist <= 0.5 {
        info!("Distance less than 0.5, updating last_checkpoint to {}", next_point);
        let mut last_checkpoint = state.last_checkpoint.lock().await;
        *last_checkpoint = next_point;
        info!("Updated to next_point: {}", next_point)
    }
    Ok(())
}

pub async fn handle_tick(client: Client, state: State) -> anyhow::Result<()> {
    let last_checkpoint_clone = state.last_checkpoint.clone();
    let last_checkpoint = last_checkpoint_clone.lock().await;
    match *last_checkpoint {
        0 => next_checkpoint(client, 1, state).await?,
        1 => next_checkpoint(client, 2, state).await?,
        2 => next_checkpoint(client, 3, state).await?,
        3 => next_checkpoint(client, 0, state).await?,
        _ => next_checkpoint(client, 0, state).await?,
    }
    Ok(())
}