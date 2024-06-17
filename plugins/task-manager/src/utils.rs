use std::time::Duration;
use azalea::ecs::prelude::*;
use crate::TaskManager;

#[derive(Event)]
pub(crate) struct DelayTask {
    pub(crate) duration: Duration
}

pub(crate) fn handle_delay_task(
    mut events: EventReader<DelayTask>,
    mut query: Query<(), With<TaskManager>>
) {
    todo!()
}

pub(crate) struct SendChatTask {
    pub(crate) entity: Entity,
    pub(crate) message: String
}

pub(crate) fn handle_send_chat_task(
    mut events: EventReader<SendChatTask>,
    mut query: Query<(), With<TaskManager>>
) {

}