use std::time::Instant;
use azalea::chat::ChatReceivedEvent;
use azalea::prelude::*;
use azalea::ecs::prelude::*;
use azalea::entity::LocalEntity;
use azalea::entity::metadata::Player;
use azalea::GameProfileComponent;
use tracing::error;
use crate::{AccountJoinedEvent, AccountLoggedInInformation, JoinCoordinateRes, utils::approx_time_to_join};


pub(crate) fn handle_chat<S>(
    mut join_coordinate_res: ResMut<JoinCoordinateRes<S>>,
    mut events: EventReader<ChatReceivedEvent>,
    query: Query<&GameProfileComponent, (With<Player>, With<LocalEntity>)>,
    mut account_joined_event: EventWriter<AccountJoinedEvent>
)
where S: Send + Sync + Clone + Component + 'static
{
    for event in events.read() {
        if let Ok(game_profile_component) = query.get(event.entity) {
            if let Some(account_queue_information) = join_coordinate_res.to_owned().accounts_in_queue.get_mut(&game_profile_component.name) {
                let message = event.packet.message().to_string();

                if let Some(position_in_queue_captures) = join_coordinate_res.re_position.captures(&*message) {
                    if let Some(position_in_queue) = position_in_queue_captures.get(1) {
                        let position = position_in_queue.as_str().parse::<u32>().expect("Unable to parse position");

                        if account_queue_information.position_in_queue != position {
                            join_coordinate_res.login_rate = login_rate(
                                account_queue_information.last_position_in_queue,
                                position,
                                account_queue_information.last_position_time
                            );

                            account_queue_information.last_position_in_queue = account_queue_information.position_in_queue;
                            account_queue_information.position_in_queue = position;
                            account_queue_information.last_position_time = Instant::now();

                            join_coordinate_res.eta = approx_time_to_join(
                                account_queue_information.position_in_queue,
                                join_coordinate_res.login_rate
                            );
                        }
                    }
                } else if let Some(total_queue_captures) = join_coordinate_res.re_total_position.captures(&*message) {
                    if let Some(total_queue_length) = total_queue_captures.get(1) {
                        let total_queue = total_queue_length.as_str().parse::<u32>().expect("Unable to parse total queue length");
                        join_coordinate_res.total_queue = total_queue;
                    }
                } else if message == "Connected to the server.".to_string() {
                    // Bot has passed the queue
                    let login_time = Instant::now();
                    join_coordinate_res.accounts_in_queue.remove(&game_profile_component.name);

                    let account_logged_in_information = AccountLoggedInInformation {
                        login_time,
                        is_disconnected: false,
                    };
                    join_coordinate_res.accounts_logged_in.insert(
                        game_profile_component.name.to_owned(),
                        account_logged_in_information
                    );

                    account_joined_event.send(AccountJoinedEvent {
                        username: game_profile_component.name.to_owned(),
                        login_time
                    });
                }
            }
        }
    }
}

// This way of calculating `login_rate` isn't very accurate
// TODO: more accurate `login_rate`
fn login_rate(last_position: u32, current_position: u32, last_position_time: Instant) -> f32 {
    if (last_position - current_position) > 0 {
        let time_took = last_position_time.elapsed();

        let rate = (last_position - current_position) as f32 / time_took.as_secs_f32();

        rate
    } else {
        error!("Position has increased");

        // Assuming rate of 400 players per 12 hours
        400f32/12f32/60f32/60f32
    }
}