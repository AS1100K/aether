#![doc = include_str!("../README.md")]

pub mod client;
mod interaction;
mod movement;
pub mod task_manager_queue;
mod utils;

use crate::client::handle_add_task_event;
use crate::interaction::handle_interact_with_block_task_event;
use crate::movement::{handle_goto_task_event, handle_stop_pathfinding_when_reached};
use crate::task_manager_queue::{Task, TaskManagerQueue};
use crate::utils::{handle_delay_task_event, handle_send_chat_task_event};
use azalea::app::{App, Plugin, PreUpdate, Update};
use azalea::ecs::prelude::*;
use azalea::entity::metadata::Player;
use azalea::entity::LocalEntity;
use azalea::physics::PhysicsSet;
use azalea::prelude::*;
use azalea::{BlockPos, Vec3};
#[cfg(feature = "anti-afk")]
use azalea_anti_afk::AntiAFK;
use std::time::{Duration, Instant};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskManagerSet;

pub struct TaskManagerPlugin;

impl Plugin for TaskManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GotoTaskEvent>()
            .add_event::<DelayTaskEvent>()
            .add_event::<SendChatTaskEvent>()
            .add_event::<InteractWithBlockTaskEvent>()
            .add_event::<AddTaskEvent>()
            .add_systems(PreUpdate, add_default_task_manager)
            .add_systems(
                GameTick,
                (handle_stop_pathfinding_when_reached, task_executor).chain(),
            )
            .add_systems(
                Update,
                (
                    handle_goto_task_event,
                    handle_delay_task_event,
                    handle_send_chat_task_event,
                    handle_interact_with_block_task_event,
                    handle_add_task_event,
                )
                    .chain()
                    .in_set(TaskManagerSet)
                    .before(PhysicsSet),
            );
    }
}

#[derive(Component, Default, Clone)]
pub struct TaskManager {
    pub queue: TaskManagerQueue,
    pub ongoing_task: bool,
}

#[allow(clippy::complexity)]
fn add_default_task_manager(
    mut commands: Commands,
    mut query: Query<Entity, (Without<TaskManager>, With<LocalEntity>, With<Player>)>,
) {
    for entity in &mut query {
        commands.entity(entity).insert(TaskManager::default());
    }
}

#[allow(clippy::complexity)]
fn task_executor(
    #[cfg(feature = "anti-afk")] mut commands: Commands,
    mut query: Query<
        (&mut TaskManager, Entity),
        (With<TaskManager>, With<Player>, With<LocalEntity>),
    >,
    mut goto_task_event: EventWriter<GotoTaskEvent>,
    mut send_chat_task: EventWriter<SendChatTaskEvent>,
    mut delay_task: EventWriter<DelayTaskEvent>,
    mut interact_with_block_task_event: EventWriter<InteractWithBlockTaskEvent>,
) {
    for (mut task_manager, entity) in &mut query {
        if task_manager.queue.len() > 0 && !task_manager.ongoing_task {
            task_manager.ongoing_task = true;

            // Cool! there is a task, let's execute it
            let next_task = task_manager.queue.get(0).unwrap();

            match next_task {
                Task::GotoTask(target, allow_mining, distance) => {
                    let target = target.to_owned();
                    let allow_mining = allow_mining.to_owned();
                    let distance = distance.to_owned();

                    goto_task_event.send(GotoTaskEvent {
                        entity,
                        target,
                        allow_mining,
                        distance,
                    });
                }
                Task::SendChatMessage(message) => {
                    let message = message.to_owned();

                    send_chat_task.send(SendChatTaskEvent { entity, message });
                }
                Task::Delay(duration) => {
                    let duration = duration.to_owned();

                    delay_task.send(DelayTaskEvent { entity, duration });
                }
                Task::InteractWithBlock(target) => {
                    let target = target.to_owned();

                    interact_with_block_task_event
                        .send(InteractWithBlockTaskEvent { entity, target });
                }
                #[cfg(feature = "anti-afk")]
                Task::SetAntiAFK(enabled, anti_afk_config) => {
                    if *enabled {
                        commands.entity(entity).insert(AntiAFK {
                            last_afk_tick: Instant::now(),
                            config: anti_afk_config
                                .to_owned()
                                .expect("AntiAFK Config wasn't passed"),
                            has_moved: None,
                        });
                    } else {
                        commands.entity(entity).remove::<AntiAFK>();
                    }

                    task_manager.queue.remove();
                    task_manager.ongoing_task = false;
                }
            }
        }
    }
}

/// Send this event to add a task to current entity
#[derive(Event)]
pub struct AddTaskEvent {
    pub entity: Entity,
    pub task: Task,
}

#[derive(Event)]
pub(crate) struct GotoTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) target: BlockPos,
    pub(crate) allow_mining: bool,
    pub(crate) distance: f64,
}

#[derive(Component)]
pub(crate) struct StopPathfindingWhenReached {
    pub(crate) target: Vec3,
    pub(crate) distance: f64,
}

#[derive(Event)]
pub(crate) struct DelayTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) duration: Duration,
}

#[derive(Event)]
pub(crate) struct SendChatTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) message: String,
}

#[derive(Event)]
pub(crate) struct InteractWithBlockTaskEvent {
    pub(crate) entity: Entity,
    pub(crate) target: BlockPos,
}
