use crate::{TaskManager, TaskManagerRes};
use azalea::ecs::prelude::*;
use azalea::entity::Position;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::pathfinder::{moves, GotoEvent, StopPathfindingEvent};
use azalea::{BlockPos, TickBroadcast, Vec3};
use std::sync::Arc;
use std::time::Duration;

#[derive(Event)]
pub(crate) struct GotoTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) target: BlockPos,
    pub(crate) allow_mining: bool,
}

#[derive(Event)]
pub(crate) struct StopPathfindingWhenReachedEvent {
    pub(crate) entity: Entity,
    pub(crate) target: Vec3,
}

pub(crate) fn handle_goto_task_event(
    mut events: EventReader<GotoTaskEvent>,
    mut _query: Query<(), With<TaskManager>>,
    mut goto_event: EventWriter<GotoEvent>,
    mut stop_pathfinding_when_reached_event: EventWriter<StopPathfindingWhenReachedEvent>,
) {
    for event in events.read() {
        goto_event.send(GotoEvent {
            entity: event.entity,
            goal: Arc::new(BlockPosGoal(event.target)),
            successors_fn: moves::default_move,
            allow_mining: event.allow_mining,
        });

        std::thread::sleep(Duration::from_millis(20));

        stop_pathfinding_when_reached_event.send(StopPathfindingWhenReachedEvent {
            entity: event.entity,
            target: event.target.to_vec3_floored(),
        });
    }
}

pub(crate) async fn handle_stop_pathfinding_when_reached(
    mut task_manager: ResMut<TaskManagerRes>,
    mut events: EventReader<StopPathfindingWhenReachedEvent>,
    mut query: Query<&Position, With<TaskManager>>,
    mut tick_broadcast: Res<TickBroadcast>,
    mut stop_pathfinding_event: EventWriter<StopPathfindingEvent>,
) {
    for event in events.read() {
        let position = query
            .get_mut(event.entity)
            .expect("Unable to get `TaskManager` and `Position`");
        let mut receiver = tick_broadcast.subscribe();

        while receiver.recv().await.is_ok() {
            let distance = position.distance_to(&event.target).abs();

            if distance <= 0.1 {
                stop_pathfinding_event.send(StopPathfindingEvent {
                    entity: event.entity,
                    force: false,
                });

                task_manager.queue.remove();
                task_manager.ongoing_task = false;
            }
        }
    }
}
