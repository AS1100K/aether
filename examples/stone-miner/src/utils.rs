use azalea::{BlockPos, BotClientExt, Client, Vec3};
use log::{info, trace};

pub async fn direction(end: Vec3, start: Vec3) -> anyhow::Result<f64> {
    let dx = (end.x - start.x).abs();
    let dz = (end.z - start.z).abs();
    let angle = dx.atan2(dz).to_degrees();
    trace!("direction: start: ({}, {}), end: ({}, {}), angle: {}", start.x, start.z, end.x, end.z, angle);
    Ok(angle)
}

pub async fn distance(end: Vec3, start: Vec3) -> anyhow::Result<f64> {
    let dist = start.distance_to(&end);
    trace!("distance: {}", dist);
    Ok(dist)
}

pub async fn mine(client: &mut Client, y_start: i32, y_end: i32) -> anyhow::Result<()> {
    let current_position = client.position();
    let current_x: i32 = format!("{:.0}", current_position.x).parse().unwrap();
    let current_z: i32 = format!("{:.0}", current_position.z).parse().unwrap();

    // To not trigger anti-cheat the bot can mine upto 4 blocks safely
    // for example:
    // y_start = 6
    // y_end = 10
    for y in y_start..y_end {
        let block = BlockPos::new(current_x, y, current_z);
        trace!("Mining {:?}", block);
        client.mine(block).await;
    }

    Ok(())
}
