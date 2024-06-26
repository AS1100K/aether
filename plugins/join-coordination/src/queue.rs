use azalea::chat::{ChatPacketKind, SendChatKindEvent};
use azalea::entity::LocalEntity;
use azalea::entity::metadata::Player;
use azalea::GameProfileComponent;
use azalea::prelude::Component;
use bevy::prelude::{Entity, EventWriter, Query, ResMut, With};
use crate::JoinCoordinateRes;

pub(crate) fn queue_information<S> (
    mut join_coordinate_res: ResMut<JoinCoordinateRes<S>>,
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