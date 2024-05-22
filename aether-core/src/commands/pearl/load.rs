use crate::msg;
use crate::utils::distance;
use crate::State;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::PathfinderClientExt;
use azalea::Client;

pub async fn handle_load(username: String, mut client: Client, mut state: State) {
    println!("{:?}", state.ongoing_task.lock().unwrap());

    {
        let mut ongoing_task = state.ongoing_task.lock().unwrap();
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
            // This code block doesn't work as intended
            // if d <= 4.0 {
            //     if can_interact(
            //         client_clone.position(),
            //         trapdoor.to_vec3_floored(),
            //         client_clone.world(),
            //     )
            //     .await
            //     {
            //         client_clone.stop_pathfinding();
            //         client_clone.block_interact(trapdoor);
            //         msg!(client_clone, username, "Pearl Loaded");
            //         msg!(client_clone, username, "Make sure to put your pearl back!");
            //         break;
            //     } else {
            //         msg!(client_clone, username, "There is something between me and the chambers, so I can't load the pearl rn.");
            //         break;
            //     }
            // }

            if d <= 4.0 {
                client_clone.stop_pathfinding();
                client_clone.block_interact(trapdoor);
                msg!(client_clone, username, "Pearl Loaded");
                msg!(client_clone, username, "Make sure to put your pearl back!");

                {
                    let mut ongoing_task = state_clone.ongoing_task.lock().unwrap();
                    *ongoing_task = false;
                }

                break;
            }
        }
    });

    // TODO: Make the bot go back to afk location
}
