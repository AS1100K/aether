use crate::{GotoTaskEvent, StopPathfindingWhenReached, TaskManager, TaskManagerRes};
use azalea::ecs::prelude::*;
use azalea::entity::Position;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::{moves, GotoEvent, StopPathfindingEvent};
use std::sync::Arc;
use std::time::Duration;

pub(crate) fn handle_goto_task_event(
    mut commands: Commands,
    mut events: EventReader<GotoTaskEvent>,
    mut _query: Query<(), With<TaskManager>>,
    mut goto_event: EventWriter<GotoEvent>,
) {
    for event in events.read() {
        goto_event.send(GotoEvent {
            entity: event.entity,
            goal: Arc::new(BlockPosGoal(event.target)),
            successors_fn: moves::default_move,
            allow_mining: event.allow_mining,
        });

        std::thread::sleep(Duration::from_millis(20));

        commands.entity(event.entity).insert(StopPathfindingWhenReached {
            target: event.target.to_vec3_floored(),
        });
    }
}

pub(crate) fn handle_stop_pathfinding_when_reached(
    mut commands: Commands,
    mut task_manager: ResMut<TaskManagerRes>,
    mut query: Query<(&StopPathfindingWhenReached, &Position, Entity), With<TaskManager>>,
    mut stop_pathfinding_event: EventWriter<StopPathfindingEvent>,
) {
    for (component, position, entity) in query.iter_mut() {
        let distance = position.distance_to(&component.target).abs();

        if distance <= 0.1 {
            stop_pathfinding_event.send(StopPathfindingEvent {
                entity,
                force: false,
            });

            task_manager.queue.remove();
            task_manager.ongoing_task = false;
            commands.entity(entity).remove::<StopPathfindingWhenReached>();
        }
    }
}
