use azalea::{BlockPos, BotClientExt, Client, Vec3};
use log::{info, trace};

pub async fn distance(end: Vec3, start: Vec3) -> anyhow::Result<f64> {
    let new_start = Vec3::new(start.x.floor(), start.y.floor(), start.z.floor());
    let dist = new_start.distance_to(&end);
    info!("end: {}", end);
    info!("distance: {}", dist);
    info!("new_start: {}", new_start);
    Ok(dist)
}