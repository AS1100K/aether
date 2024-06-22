use std::time::Duration;
use azalea::Client;
use log::{info, warn};
use azalea_task_manager::client::TaskManagerExt;
use azalea_task_manager::task_manager_queue::Task;
use crate::config::Bot;

pub async fn handle_load(username: String, client: Client, state: Bot) {
    info!("Received Pearl Loading Command from {} by {}", username, state.username);

    {
        let ongoing_task = client.len_tasks() > 0;
        if ongoing_task {
            warn!("{} I am currently going somewhere, please resend the command after a while.", username);
            return;
        }
    }

    if let Some(trapdoor) = state.pearl_locations.unwrap().get(&username) {
        let trapdoor = *trapdoor;
        let _ = client
            .new_task(Task::SetAntiAFK(false))
            .new_task(Task::GotoTask(trapdoor, false, 2.0))
            .new_task(Task::Delay(Duration::from_secs(1)))
            .new_task(Task::InteractWithBlock(trapdoor))
            .new_task(Task::Delay(Duration::from_secs(1)))
            .new_task(Task::GotoTask(state.afk_location.unwrap(), false, 1.0))
            .new_task(Task::SetAntiAFK(true));
    } else {
        warn!("{} Unable to find your trapdoor coordinates, use !pearl set x y z", username);
    }
}
