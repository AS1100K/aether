use crate::msg;
use crate::utils::{distance, stop_pathfinding_when_reachable};
use crate::State;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::PathfinderClientExt;
use azalea::{BlockPos, Client};
use log::info;

pub async fn handle_load(username: String, client: Client, state: State) {
    info!("Received Pearl Loading Command from {}", username);

    {
        let mut ongoing_task = state.client_information.ongoing_task.lock();
        let mut is_afk = state.client_information.is_afk.lock();
        if *ongoing_task && !*is_afk {
            msg!(
                client,
                username,
                "I am currently going somewhere, please resend the command after a while."
            );
            return;
        }
        *ongoing_task = true;
        *is_afk = false;
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

    let client_clone = client.clone();
    let state_clone = state.clone();

    stop_pathfinding_when_reachable(
        client_clone,
        state_clone,
        trapdoor.to_vec3_floored(),
        None,
        Some(flip_trapdoor),
        Some((client, username, trapdoor)),
    )
}

async fn flip_trapdoor(args: Option<(Client, String, BlockPos)>) {
    if let Some((mut client, username, trapdoor)) = args {
        client.block_interact(trapdoor);
        msg!(client_clone, username, "Pearl Loaded");
        msg!(client_clone, username, "Make sure to put your pearl back!");
    }
}
