use azalea::BlockPos;
use azalea::{world::Instance, Vec3};
use parking_lot::RwLock;
use std::sync::Arc;
#[macro_export]
macro_rules! msg {
    ($client: expr, $username: expr, $message: expr) => {
        $client.send_command_packet(format!("msg {} {}", $username, $message).as_str())
    };
}

pub fn distance(current: Vec3, target: Vec3) -> f64 {
    current.distance_to(&target)
}

/// Checks weather the bot can interact with the block position.
/// This function has bugs
pub async fn can_interact(current: Vec3, target: Vec3, world: Arc<RwLock<Instance>>) -> bool {
    let direction = (target - current).normalize();

    let mut current_pos = current;

    while (current_pos - current).length() < (target - current).length() {
        let block = world.read().get_block_state(&BlockPos::new(
            current_pos.x as i32,
            current_pos.y as i32,
            current_pos.z as i32,
        ));
        if !block.unwrap().is_air() {
            return false;
        }

        // Move the position along the ray
        current_pos += direction * 0.1;
    }

    return true;
}
