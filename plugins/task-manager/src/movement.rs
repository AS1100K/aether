use crate::{GotoTaskEvent, StopPathfindingWhenReached, TaskManager};
use azalea::ecs::prelude::*;
use azalea::entity::metadata::Player;
use azalea::entity::{LocalEntity, Position};
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::{moves, GotoEvent, StopPathfindingEvent};
use azalea::Vec3;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

pub(crate) fn handle_goto_task_event(
    mut commands: Commands,
    mut events: EventReader<GotoTaskEvent>,
    _query: Query<(), With<TaskManager>>,
    mut goto_event: EventWriter<GotoEvent>,
) {
    for event in events.read() {
        info!("Received Goto Task to: {}", event.target);

        std::thread::sleep(Duration::from_millis(50));

        goto_event.send(GotoEvent {
            entity: event.entity,
            goal: Arc::new(BlockPosGoal(event.target)),
            successors_fn: moves::default_move,
            allow_mining: event.allow_mining,
        });

        std::thread::sleep(Duration::from_millis(50));

        commands
            .entity(event.entity)
            .insert(StopPathfindingWhenReached {
                target: event.target.to_vec3_floored(),
                distance: event.distance,
            });
    }
}

#[allow(clippy::complexity)]
pub(crate) fn handle_stop_pathfinding_when_reached(
    mut commands: Commands,
    mut query: Query<
        (
            &mut TaskManager,
            &StopPathfindingWhenReached,
            &Position,
            Entity,
        ),
        (
            With<TaskManager>,
            With<StopPathfindingWhenReached>,
            With<Player>,
            With<LocalEntity>,
        ),
    >,
    mut stop_pathfinding_event: EventWriter<StopPathfindingEvent>,
) {
    for (mut task_manager, component, position, entity) in query.iter_mut() {
        let distance = position.floor().distance_to(&component.target).abs();

        if distance <= component.distance {
            stop_pathfinding_event.send(StopPathfindingEvent {
                entity,
                force: true,
            });

            info!("Completed Goto Task");
            task_manager.queue.remove();
            task_manager.ongoing_task = false;
            commands
                .entity(entity)
                .remove::<StopPathfindingWhenReached>();
        }
    }
}

pub trait PositionExt {
    fn floor(self) -> Self;
}

impl PositionExt for Position {
    fn floor(self) -> Self {
        let new_position = Vec3::new(self.x.floor(), self.y.floor(), self.z.floor());

        Position::new(new_position)
    }
}
