use azalea::Vec3;
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