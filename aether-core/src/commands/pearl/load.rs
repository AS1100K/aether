use crate::msg;
use crate::utils::distance;
use crate::State;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::PathfinderClientExt;
use azalea::Client;

pub async fn handle_load(username: String, mut client: Client, state: State) {
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
                break;
            }
        }
    });

    // TODO: Make the bot go back to afk location
}
