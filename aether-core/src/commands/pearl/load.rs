use crate::config::Bot;
use azalea::Client;
use azalea_anti_afk::config::AntiAFKConfig;
use azalea_task_manager::client::TaskManagerExt;
use azalea_task_manager::task_manager_queue::Task;
use std::time::Duration;
use tracing::{info, warn};

pub async fn handle_load(username: String, client: Client, state: Bot) {
    info!(
        "Received Pearl Loading Command from {} by {}",
        username, state.username
    );

    {
        let ongoing_task = client.len_tasks() > 0;
        if ongoing_task {
            warn!(
                "{} I am currently going somewhere, please resend the command after a while.",
                username
            );
            return;
        }
    }

    if let Some(trapdoor) = state.pearl_locations.unwrap().get(&username) {
        let central_afk_location = state
            .afk_location
            .map(|afk_location| afk_location.to_vec3_floored());

        let anti_afk_config = AntiAFKConfig {
            jump: false,
            sneak: false,
            walk: false,
            flip_lever: true,
            central_afk_location,
        };

        let trapdoor = *trapdoor;
        let _ = client
            .new_task(Task::SetAntiAFK(false, Some(anti_afk_config)))
            .new_task(Task::SendChatMessage(format!(
                "/w {} Teleporting...",
                username
            )))
            .new_task(Task::GotoTask(trapdoor, false, 2.0))
            .new_task(Task::Delay(Duration::from_millis(500)))
            .new_task(Task::InteractWithBlock(trapdoor))
            .new_task(Task::SendChatMessage(format!("/w {} Teleported", username)))
            .new_task(Task::Delay(Duration::from_secs(1)))
            .new_task(Task::SendChatMessage(format!(
                "/w {} Make sure to put your pearl back",
                username
            )))
            .new_task(Task::GotoTask(state.afk_location.unwrap(), false, 1.0))
            .new_task(Task::SetAntiAFK(true, Some(anti_afk_config)));
    } else {
        warn!(
            "{} Unable to find your trapdoor coordinates, use !pearl set x y z",
            username
        );
    }
}
