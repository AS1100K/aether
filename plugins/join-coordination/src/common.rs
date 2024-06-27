use std::time::Instant;
use azalea::disconnect::DisconnectEvent;
use azalea::GameProfileComponent;
use azalea::prelude::*;
use azalea::ecs::prelude::*;
use azalea::swarm::Swarm;
use bevy_tasks::IoTaskPool;
use crate::{AccountInQueue, AccountLoggedIn, AccountQueueInformation, AddAccount, AddAccountEvent, JoinCoordinateRes};

pub(crate) fn handle_bots<S> (
    mut join_coordinate_res: ResMut<JoinCoordinateRes<S>>,
    mut add_account: EventWriter<AddAccount<S>>,
    account_in_queue_query: Query<(), With<AccountInQueue>>,
    account_logged_in_query: Query<(), With<AccountLoggedIn>>
)
where S: Send + Sync + Clone + Component + 'static
{
    // Check if there is not a single bot in queue
    if join_coordinate_res.accounts_in_queue.len() == 0 {
        let first_account = join_coordinate_res.accounts.iter().next();
        if let Some((_username, (account, default_state))) = first_account {
            add_account.send(AddAccount {
                account: account.to_owned(),
                state: default_state.to_owned(),
            });
        }
    } else {
        todo!()
    }
}

pub(crate) fn login_first_account<S> (
    join_coordinate_res: ResMut<JoinCoordinateRes<S>>,
    mut add_account: EventWriter<AddAccount<S>>
)
where S: Send + Sync + Clone + Component + 'static
{
    let first_account = join_coordinate_res.accounts.iter().next();
    if let Some((_username, (account, default_state))) = first_account {
        add_account.send(AddAccount {
            account: account.to_owned(),
            state: default_state.to_owned(),
        });
    }
}

// This system doesn't compile
pub(crate) fn add_account<S> (
    mut join_coordinate_res: ResMut<JoinCoordinateRes<S>>,
    mut swarm: ResMut<Swarm>,
    mut events: EventReader<AddAccount<S>>,
    mut add_account_event: EventWriter<AddAccountEvent>
)
where S: Send + Sync + Clone + Component + 'static
{
    for event in events.read() {
        IoTaskPool::get()
            .spawn(async move {
                let event_clone = event.clone();
                let account = event_clone.account.to_owned();
                let state = event_clone.state.to_owned();
                let _ = swarm.add_and_retry_forever(&account, state).await;

                let account_queue_information = AccountQueueInformation {
                    position_in_queue: 400,
                    last_position_in_queue: 400,
                    last_position_time: Instant::now(),
                };

                join_coordinate_res.accounts_in_queue.insert(
                    account.username.to_owned(),
                    account_queue_information
                );

                add_account_event.send(AddAccountEvent {
                    account
                });
            })
            .detach();
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