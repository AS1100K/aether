use crate::State;
use azalea::Client;
use log::trace;
use rand::Rng;
use std::time::{Duration, Instant};

pub async fn handle_tick(mut client: Client, state: State) -> anyhow::Result<()> {
    if *state.game_information.is_connected.lock().unwrap()
        && !*state.client_information.ongoing_task.lock().unwrap()
        && *state.client_information.is_afk.lock().unwrap()
    {
        let now = Instant::now();
        let mut last_tick = state.game_information.last_afk_tick.lock().unwrap();

        if now.duration_since(*last_tick) >= Duration::from_secs(1) {
            let y_rot = rand::thread_rng().gen_range(-180..180);
            trace!("1 second has passed, setting `y_rot` to {}", y_rot);

            client.set_direction(y_rot as f32, client.direction().1);
            *last_tick = Instant::now();
        }
    }

    Ok(())
}
