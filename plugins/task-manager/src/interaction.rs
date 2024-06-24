use crate::{InteractWithBlockTaskEvent, TaskManager};
use azalea::ecs::prelude::*;
use azalea::interact::BlockInteractEvent;
use tracing::info;

pub(crate) fn handle_interact_with_block_task_event(
    mut events: EventReader<InteractWithBlockTaskEvent>,
    mut query: Query<&mut TaskManager, With<TaskManager>>,
    mut block_interact_event: EventWriter<BlockInteractEvent>,
) {
    for event in events.read() {
        let mut task_manager = query.get_mut(event.entity).unwrap();

        info!("Received Task to interact with the block");
        block_interact_event.send(BlockInteractEvent {
            entity: event.entity,
            position: event.target,
        });

        task_manager.queue.remove();
        task_manager.ongoing_task = false;
    }
}
