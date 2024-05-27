use azalea::BlockPos;
use azalea::{world::Instance, Vec3};
use parking_lot::RwLock;
use std::sync::Arc;
use azalea::chat::ChatPacket;
use regex::Regex;
#[macro_export]
macro_rules! msg {
    ($client: expr, $username: expr, $message: expr) => {
        $client.send_command_packet(format!("w {} {}", $username, $message).as_str())
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

pub fn parse_chat_content(chat: &ChatPacket) -> (Option<String>, String, bool) {
    let (username, content) = chat.split_sender_and_content();
    // If the username is present, use split_sender_and_content
    if let Some(uname) = username {
        // Return the result of split_sender_and_content as it's not a whisper
        return (Some(uname), content, chat.is_whisper());
    }

    // Define regex pattern to match whispers
    let whisper_pattern = Regex::new(r"(?P<uname>\w+) whispers: (?P<message>.+)").unwrap();

    // If the content matches the whisper pattern
    if let Some(captures) = whisper_pattern.captures(&content) {
        let uname = captures.name("uname").map(|u| u.as_str().to_string());
        let message = captures.name("message").map(|m| m.as_str().to_string()).unwrap_or_default();
        return (uname, message, true); // It's a whisper
    }

    // If no match is found, return None for the username and the original content
    (None, content, false)
}
