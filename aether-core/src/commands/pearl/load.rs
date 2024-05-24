use crate::msg;
use crate::utils::distance;
use crate::State;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::PathfinderClientExt;
use azalea::Client;
use log::info;

pub async fn handle_load(username: String, client: Client, state: State) {
    info!("Received Pearl Loading Command from {}", username);

    {
        let mut ongoing_task = state.client_information.ongoing_task.lock().unwrap();
        if *ongoing_task {
            msg!(client, username, "I am currently teleporting another player, please wait a few seconds and resend the command.");
            msg!(
                client,
                username,
                "A better way of processing tasks from multiple players, is WIP."
            );
            return;
        }
        *ongoing_task = true;
    }

    msg!(client, username, "Teleporting...");

    let trapdoor = state.config.pearl_locations.get(&username);
    if trapdoor.is_none() {
        msg!(
            client,
            username,
            "Unable to find your trapdoor coordinates, use !pearl set x y z"
        );
        return;
    }

    let trapdoor = *trapdoor.unwrap();

    client.goto(BlockPosGoal(trapdoor));

    let mut client_clone = client.clone();
    let state_clone = state.clone();

    tokio::task::spawn(async move {
        loop {
            let d = distance(client_clone.position(), trapdoor.to_vec3_floored());

            if d <= 4.0 {
                client_clone.stop_pathfinding();
                client_clone.block_interact(trapdoor);
                msg!(client_clone, username, "Pearl Loaded");
                msg!(client_clone, username, "Make sure to put your pearl back!");

                {
                    let mut ongoing_task =
                        state_clone.client_information.ongoing_task.lock().unwrap();
                    *ongoing_task = false;
                }

                break;
            }
        }
    });
}
