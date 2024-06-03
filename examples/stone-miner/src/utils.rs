use azalea::Vec3;
use log::info;

pub async fn direction(end: Vec3, start: Vec3) -> anyhow::Result<f64> {
    let dx = (end.x - start.x).abs();
    let dz = (end.z - start.z).abs();
    let angle = dx.atan2(dz).to_degrees();
    info!("direction: start: ({}, {}), end: ({}, {}), angle: {}", start.x, start.z, end.x, end.z, angle);
    Ok(angle)
}

pub async fn distance(end: Vec3, start: Vec3) -> anyhow::Result<f64> {
    Ok(start.distance_to(&end))
}