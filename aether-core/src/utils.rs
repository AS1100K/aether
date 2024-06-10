use crate::state::State;
use azalea::chat::ChatPacket;
use azalea::pathfinder::PathfinderClientExt;
use azalea::{world::Instance, Vec3};
use azalea::{BlockPos, Client};
use parking_lot::RwLock;
use regex::Regex;
use std::future::Future;
use std::sync::Arc;
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
        let message = captures
            .name("message")
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        return (uname, message, true); // It's a whisper
    }

    // If no match is found, return None for the username and the original content
    (None, content, false)
}

/// Stops ongoing pathfinding when 3 blocks away from the `target_pos`
/// Once, there it sets `state.client_information.ongoing_task = false`
///
/// Arguments Taken: `client`, `state`, `target_pos`, `future_function`, `args`
/// `client` -> Azalea Client
/// `state` -> Bot State
/// `target_pos` -> `Vec3` position where the bot is intended to go
/// `future_function` -> `Option<F>` Function that will be executed once the bot is within 3 blocks range of `target_pos`
/// `args` -> `Option<Args>` Arguments that will be passed to `future_function`
///
/// # Example
/// ```no-run
/// let trapdoor: BlockPos = Trapdoor::default();
/// let client_clone: Client = client.clone();
///
/// client.goto(BlockPosGoal::new(trapdoor));
/// stop_pathfinding_when_reachable(client_clone, state, trapdoor.to_vec3_floored(), Some(flip_trapdoor), Some(client, username, trapdoor));
///
/// // --snipe--
/// async fn flip_trapdoor(args: Option<(Client, String, BlockPos)>) {
///     if let Some((mut client, username, trapdoor)) = args {
///         client.block_interact(trapdoor);
///         msg!(client_clone, username, "Pearl Loaded");
///         msg!(client_clone, username, "Make sure to put your pearl back!");
///     }
/// }
/// ```
pub fn stop_pathfinding_when_reachable<F, Fut, Args>(
    client: Client,
    state: State,
    target_pos: Vec3,
    future_function: Option<F>,
    args: Option<Args>,
) where
    F: FnOnce(Option<Args>) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
    Args: Send + 'static,
{
    tokio::task::spawn(async move {
        loop {
            let d = distance(client.position(), target_pos);

            if d <= 3.0 {
                client.stop_pathfinding();
                if let Some(f) = future_function {
                    f(args).await;
                }

                {
                    let mut ongoing_task = state.client_information.ongoing_task.lock();
                    *ongoing_task = false;
                }

                break;
            }
        }
    });
}
