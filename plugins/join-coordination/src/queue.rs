use azalea::chat::{ChatPacketKind, SendChatKindEvent};
use azalea::entity::LocalEntity;
use azalea::entity::metadata::Player;
use azalea::GameProfileComponent;
use azalea::prelude::*;
use azalea::ecs::prelude::*;
use crate::{AccountInQueue, AccountJoinedEvent, AccountLoggedIn, JoinCoordinateRes};

pub(crate) fn queue_information<S> (
    join_coordinate_res: ResMut<JoinCoordinateRes<S>>,
    query: Query<(&GameProfileComponent, Entity), (With<Player>, With<LocalEntity>)>,
    mut send_chat_kind_event: EventWriter<SendChatKindEvent>
)
where S: Send + Sync + Clone + Component + 'static
{
    for (game_profile_component, entity) in query.iter() {
        // Every bot which is in the queue will send `/queue` to the server.
        // Only one bot should do this, otherwise it impacts the performance.
        if join_coordinate_res.accounts_in_queue.contains_key(&game_profile_component.name) {
            send_chat_kind_event.send(SendChatKindEvent {
                entity,
                content: "queue".to_string(),
                kind: ChatPacketKind::Command,
            });
        }
    }
}

pub(crate) fn add_queue_component<S> (
    join_coordinate_res: Res<JoinCoordinateRes<S>>,
    mut events: EventReader<AccountJoinedEvent>,
    query: Query<(Entity, &GameProfileComponent), (With<Player>, With<LocalEntity>)>,
    mut commands: Commands
)
where S: Send + Sync + Clone + Component + 'static
{
    for _ in events.read() {
        for (entity, game_profile_component) in query.iter() {
            if join_coordinate_res.accounts_in_queue.contains_key(&game_profile_component.name) {
                commands.entity(entity).insert(AccountInQueue);
            }
        }
    }
}

pub(crate) fn add_login_component<S> (
    mut commands: Commands,
    join_coordinate_res: Res<JoinCoordinateRes<S>>,
    mut events: EventReader<AccountJoinedEvent>,
    query: Query<(Entity, &GameProfileComponent), (With<Player>, With<LocalEntity>)>
)
where S: Send + Sync + Clone + Component + 'static
{
    for _ in events.read() {
        for (entity, game_profile_component) in query.iter() {
            if join_coordinate_res.accounts_logged_in.contains_key(&game_profile_component.name) {
                let mut entity_commands = commands.entity(entity);
                entity_commands.remove::<AccountInQueue>();
                entity_commands.insert(AccountLoggedIn);
            }
        }
    }
}