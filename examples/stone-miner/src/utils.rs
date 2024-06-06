use azalea::{BlockPos, BotClientExt, Client, Vec3};
use log::{info, trace};

pub async fn distance(end: Vec3, start: Vec3) -> anyhow::Result<f64> {
    let dist = start.distance_to(&end);
    trace!("distance: {}", dist);
    Ok(dist)
}

pub async fn mine(client: &mut Client, y_start: i32, y_end: i32) -> anyhow::Result<()> {
    let current_position = client.position();
    let current_x = current_position.x.floor();
    let current_z = current_position.z.floor();

    // To not trigger anti-cheat the bot can mine upto 4 blocks safely
    // for example:
    // y_start = 6
    // y_end = 10
    for y in y_start..y_end {
        let block = BlockPos::new(current_x as i32, y, current_z as i32);
        trace!("Mining {:?}", block);
        client.mine(block).await;
    }

    Ok(())
}