use crate::State;
use azalea::Client;
use rand::random;
use std::time::{Duration, Instant};
use crate::afk::{open_nearest_chest, random_head_rotations, send_swing_packet};

pub async fn handle_tick(mut client: Client, state: State) -> anyhow::Result<()> {
    if *state.game_information.is_connected.lock()
        && !*state.client_information.ongoing_task.lock()
        && *state.client_information.is_afk.lock()
    {
        let now = Instant::now();
        let last_tick = state.game_information.last_afk_tick;

        if now.duration_since(*last_tick.lock()) >= Duration::from_secs(3) {
            let chances: f64 = random();

            if chances < 0.5 {
                random_head_rotations(&mut client).await?
            } else if chances < 0.75 {
                open_nearest_chest(&mut client).await?
            } else {
                send_swing_packet(&mut client).await?
            }

            *last_tick.lock() = Instant::now();
        }
    }

    Ok(())
}
