use azalea::{BlockPos, BotClientExt, Client, Vec3};
use log::{info, trace};

pub async fn distance(end: Vec3, start: Vec3) -> anyhow::Result<f64> {
    let dist = start.distance_to(&end);
    trace!("distance: {}", dist);
    Ok(dist)
}