use std::time::Duration;
use azalea::chat::{ChatPacketKind, SendChatKindEvent};
use azalea::ecs::prelude::*;
use crate::{TaskManager, TaskManagerRes};

#[derive(Event)]
pub(crate) struct DelayTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) duration: Duration
}

pub(crate) fn handle_delay_task(
    mut task_manager: ResMut<TaskManagerRes>,
    mut events: EventReader<DelayTaskEvent>,
    mut _query: Query<(), With<TaskManager>>
) {
    for event in events.read() {
        std::thread::sleep(event.duration);

        task_manager.queue.remove();
        task_manager.ongoing_task = false;
    }
}

#[derive(Event)]
pub(crate) struct SendChatTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) message: String
}

pub(crate) fn handle_send_chat_task(
    mut task_manager: ResMut<TaskManagerRes>,
    mut events: EventReader<SendChatTaskEvent>,
    mut _query: Query<(), With<TaskManager>>,
    mut send_chat_kind_event: EventWriter<SendChatKindEvent>
) {
    for event in events.read() {
        let message = event.message.to_owned();

        send_chat_kind_event.send(SendChatKindEvent {
            entity: event.entity,
            content: message,
            kind: ChatPacketKind::Message,
        });

        task_manager.queue.remove();
        task_manager.ongoing_task = false;
    }
}