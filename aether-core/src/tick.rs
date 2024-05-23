use azalea::Client;
use crate::State;
use rand::Rng;

pub async fn handle_tick(mut client: Client, state: State) -> anyhow::Result<()> {
    if *state.is_connected.lock().unwrap() && !*state.ongoing_task.lock().unwrap() && *state.is_afk.lock().unwrap() {
        let y_rot = rand::thread_rng().gen_range(-180..180);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        client.set_direction(y_rot as f32, client.direction().1);
    }

    Ok(())
}