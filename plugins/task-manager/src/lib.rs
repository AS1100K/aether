pub mod task_manager_queue;
mod movement;
mod utils;
pub mod client;

use std::time::Duration;
use azalea::app::{App, Plugin, PreUpdate, Update};
use azalea::{BlockPos, Vec3};
use azalea::prelude::*;
use azalea::ecs::prelude::*;
use azalea::entity::LocalEntity;
use azalea::entity::metadata::Player;
use crate::movement::{handle_goto_task_event, handle_stop_pathfinding_when_reached};
use crate::task_manager_queue::{Task, TaskManagerQueue};
use crate::utils::{handle_delay_task_event, handle_send_chat_task_event};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskManagerSet;

pub struct TaskManagerPlugin;

impl Plugin for TaskManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TaskManagerRes::default())
            .add_event::<GotoTaskEvent>()
            .add_event::<DelayTaskEvent>()
            .add_event::<SendChatTaskEvent>()
            .add_systems(PreUpdate, add_default_task_manager)
            .add_systems(
                GameTick,
                (
                    handle_stop_pathfinding_when_reached,
                    task_executor
                )
                    .chain()
            )
            .add_systems(
                Update,
                (
                    handle_goto_task_event,
                    handle_delay_task_event,
                    handle_send_chat_task_event
                )
                    .chain()
                    .in_set(TaskManagerSet)
            );
    }
}

#[derive(Resource, Default)]
pub(crate) struct TaskManagerRes {
    pub queue: TaskManagerQueue,
    pub ongoing_task: bool
}

#[derive(Component, Default)]
pub struct TaskManager;

fn add_default_task_manager(
    mut commands: Commands,
    mut query: Query<Entity, (Without<TaskManager>, With<LocalEntity>, With<Player>)>
) {
    for entity in &mut query {
        commands.entity(entity).insert(TaskManager);
    }
}

fn task_executor(
    mut task_manager: ResMut<TaskManagerRes>,
    mut query: Query<Entity, With<TaskManager>>,
    mut goto_task_event: EventWriter<GotoTaskEvent>,
    mut send_chat_task: EventWriter<SendChatTaskEvent>,
    mut delay_task: EventWriter<DelayTaskEvent>
) {
    for entity in &mut query {
        if task_manager.queue.len() > 0 &&!task_manager.ongoing_task {
            task_manager.ongoing_task = true;

            // Cool! there is a task, let's execute it
            let next_task = task_manager.queue.get(0).unwrap();

            match next_task {
                Task::GotoTask(target, allow_mining) => {
                    let target = target.to_owned();
                    let allow_mining = allow_mining.to_owned();

                    goto_task_event.send(GotoTaskEvent {
                        entity,
                        target,
                        allow_mining
                    });
                },
                Task::SendChatMessage(message) => {
                    let message = message.to_owned();

                    send_chat_task.send(SendChatTaskEvent {
                        entity,
                        message
                    });
                },
                Task::Delay(duration) => {
                    let duration = duration.to_owned();

                    delay_task.send(DelayTaskEvent {
                        entity,
                        duration
                    });
                }
            }
        }
    }
}

#[derive(Event)]
pub(crate) struct GotoTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) target: BlockPos,
    pub(crate) allow_mining: bool,
}

#[derive(Component)]
pub(crate) struct StopPathfindingWhenReached {
    pub(crate) target: Vec3
}

#[derive(Event)]
pub(crate) struct DelayTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) duration: Duration
}

#[derive(Event)]
pub(crate) struct SendChatTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) message: String
}