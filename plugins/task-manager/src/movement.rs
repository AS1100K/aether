use azalea::BlockPos;
use azalea::ecs::prelude::*;
use crate::TaskManager;

#[derive(Event)]
pub(crate) struct GotoTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) target: BlockPos,
    pub(crate) allow_mining: bool
}

#[derive(Event)]
pub(crate) struct StopPathfindingWhenReachedEvent {
    pub(crate) entity: Entity,
    pub(crate) target: BlockPos
}

pub(crate) fn handle_goto_task_event(
    mut events: EventReader<GotoTaskEvent>,
    mut query: Query<(), With<TaskManager>>
) {
    todo!()
}

pub(crate) fn handle_stop_pathfinding_when_reached(
    mut events: EventReader<StopPathfindingWhenReachedEvent>,
    mut query: Query<(), With<TaskManager>>
) {
    todo!()
}