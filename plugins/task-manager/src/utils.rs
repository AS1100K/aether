use crate::{DelayTaskEvent, SendChatTaskEvent, TaskManager};
use azalea::chat::SendChatEvent;
use azalea::ecs::prelude::*;
use tracing::info;

pub(crate) fn handle_delay_task_event(
    mut events: EventReader<DelayTaskEvent>,
    mut query: Query<&mut TaskManager, With<TaskManager>>,
) {
    for event in events.read() {
        let mut task_manager = query.get_mut(event.entity).unwrap();

        info!("Received Delay Task");
        std::thread::sleep(event.duration);

        task_manager.queue.remove();
        task_manager.ongoing_task = false;
    }
}

pub(crate) fn handle_send_chat_task_event(
    mut events: EventReader<SendChatTaskEvent>,
    mut query: Query<&mut TaskManager, With<TaskManager>>,
    mut send_chat_event: EventWriter<SendChatEvent>,
) {
    for event in events.read() {
        let mut task_manager = query.get_mut(event.entity).unwrap();

        info!("Received Send Chat Task");
        let message = event.message.to_owned();

        send_chat_event.send(SendChatEvent {
            entity: event.entity,
            content: message,
        });

        task_manager.queue.remove();
        task_manager.ongoing_task = false;
    }
}
