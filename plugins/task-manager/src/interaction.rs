use std::time::Duration;
use azalea::ecs::prelude::*;
use azalea::interact::BlockInteractEvent;
use log::info;
use crate::{InteractWithBlockTaskEvent, TaskManager, TaskManagerRes};

pub(crate) fn handle_interact_with_block_task_event(
    mut task_manager: ResMut<TaskManagerRes>,
    mut events: EventReader<InteractWithBlockTaskEvent>,
    _query: Query<(), With<TaskManager>>,
    mut block_interact_event: EventWriter<BlockInteractEvent>
) {
    for event in events.read() {
        info!("Received Task to interact with the block");
        block_interact_event.send(BlockInteractEvent {
            entity: event.entity,
            position: event.target,
        });

        task_manager.queue.remove();
        task_manager.ongoing_task = false;
    }
}