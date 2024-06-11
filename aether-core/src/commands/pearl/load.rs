use std::time::Duration;
use crate::msg;
use crate::utils::stop_pathfinding_when_reachable;
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

    if let Some(trapdoor) = state.config.pearl_locations.get(&username) {
        msg!(client, username, "Teleporting...");

        let trapdoor = *trapdoor;

        client.goto(BlockPosGoal(trapdoor));

        stop_pathfinding_when_reachable(
            client.clone(),
            state.clone(),
            trapdoor.to_vec3_floored(),
            None,
            Some(flip_trapdoor),
            Some((client, state, username, trapdoor))
        )
    } else {
        msg!(
            client,
            username,
            "Unable to find your trapdoor coordinates, use !pearl set x y z"
        );

        let mut ongoing_task = state.client_information.ongoing_task.lock();
        *ongoing_task = false;
    }
}

async fn flip_trapdoor(args: Option<(Client, State, String, BlockPos)>) {
    if let Some((mut client, state, username, trapdoor)) = args {
        client.block_interact(trapdoor);
        msg!(client, username, "Pearl Loaded");
        msg!(client, username, "Make sure to put your pearl back!");

        tokio::time::sleep(Duration::from_secs(2)).await;
        {
            client.goto(BlockPosGoal(state.config.afk_location));
            stop_pathfinding_when_reachable(
                client,
                state.clone(),
                state.config.afk_location.to_vec3_floored(),
                Some(1.5),
                Some(set_afk),
                Some(state),
            );
        }
    }
}

async fn set_afk(args: Option<State>) {
    if let Some(mut state) = args {
        state.client_information.set_afk(true);
    }
}
