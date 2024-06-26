use std::time::Instant;
use azalea::disconnect::DisconnectEvent;
use azalea::GameProfileComponent;
use azalea::prelude::*;
use azalea::ecs::prelude::*;
use azalea::swarm::Swarm;
use bevy_tasks::IoTaskPool;
use crate::{AccountQueueInformation, JoinCoordinateRes};

pub(crate) fn handle_bots<S> (
    mut join_coordinate_res: ResMut<JoinCoordinateRes<S>>,
    mut swarm: ResMut<Swarm>
)
where S: Send + Sync + Clone + Component + 'static
{
    todo!()
}

pub(crate) fn login_first_account<S> (
    mut join_coordinate_res: ResMut<JoinCoordinateRes<S>>,
    mut swarm: ResMut<Swarm>
)
where S: Send + Sync + Clone + Component + 'static
{
    let first_account = join_coordinate_res.accounts.iter().next();
    if let Some((username, (account, default_state))) = first_account {
        IoTaskPool::get()
            .spawn(async move {
                swarm.add_and_retry_forever(account, default_state).await
            })
            .detach();

        let account_queue_information = AccountQueueInformation {
            position_in_queue: 400,
            last_position_in_queue: 400,
            last_position_time: Instant::now()
        };

        join_coordinate_res.accounts_in_queue.insert(username.to_owned(), account_queue_information);
    }
}

pub(crate) fn handle_bot_disconnect<S>(
    mut join_coordinate_res: ResMut<JoinCoordinateRes<S>>,
    mut events: EventReader<DisconnectEvent>,
    query: Query<&GameProfileComponent>
)
where S: Send + Sync + Clone + Component + 'static
{
    for event in events.read() {
        if let Ok(gamer_profile_component) = query.get(event.entity) {
            if let Some(_) = join_coordinate_res.accounts.get(&gamer_profile_component.name) {
                // Reconnecting the bot will be determined and managed by `handle_bots` system
                join_coordinate_res.accounts_in_queue.remove(&gamer_profile_component.name);
                join_coordinate_res.accounts_logged_in.remove(&gamer_profile_component.name);
            }
        }
    }
}