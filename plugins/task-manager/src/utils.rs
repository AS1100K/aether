use crate::{DelayTaskEvent, SendChatTaskEvent, TaskManager, TaskManagerRes};
use azalea::chat::{ChatPacketKind, SendChatKindEvent};
use azalea::ecs::prelude::*;
use log::info;

pub(crate) fn handle_delay_task_event(
    mut task_manager: ResMut<TaskManagerRes>,
    mut events: EventReader<DelayTaskEvent>,
    _query: Query<(), With<TaskManager>>,
) {
    for event in events.read() {
        info!("Received Delay Task");
        std::thread::sleep(event.duration);

        task_manager.queue.remove();
        task_manager.ongoing_task = false;
    }
}

pub(crate) fn handle_send_chat_task_event(
    mut task_manager: ResMut<TaskManagerRes>,
    mut events: EventReader<SendChatTaskEvent>,
    _query: Query<(), With<TaskManager>>,
    mut send_chat_kind_event: EventWriter<SendChatKindEvent>,
) {
    for event in events.read() {
        info!("Received Send Chat Task");
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
