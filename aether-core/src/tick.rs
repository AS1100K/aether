use std::time::{Duration, Instant};
use azalea::Client;
use log::trace;
use crate::State;
use rand::Rng;

pub async fn handle_tick(mut client: Client, state: State) -> anyhow::Result<()> {
    if *state.is_connected.lock().unwrap() && !*state.ongoing_task.lock().unwrap() && *state.is_afk.lock().unwrap() {
        let now = Instant::now();
        let mut last_tick = state.last_tick.lock().unwrap();

        if now.duration_since(*last_tick) >= Duration::from_secs(1) {
            let y_rot = rand::thread_rng().gen_range(-180..180);
            trace!("1 second has passed, setting `y_rot` to {}", y_rot);

            client.set_direction(y_rot as f32, client.direction().1);
            *last_tick = Instant::now();
        }
    }

    Ok(())
}