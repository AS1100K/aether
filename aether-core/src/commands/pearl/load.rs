use std::time::Duration;

use azalea::chat::SendChatEvent;
use azalea_anti_afk::config::AntiAFKConfig;
use azalea_task_manager::{task_manager_queue::Task, AddTaskEvent};
use bevy_ecs::prelude::*;

use crate::{commands::ExecutingTask, config::Bot};

/// Event triggered when Pearl Loading Command is received
#[derive(Event)]
pub struct LoadPearl {
    pub entity: Entity,
    pub username: String,
}

#[allow(clippy::complexity)]
pub fn handle_load_peral(
    mut events: EventReader<LoadPearl>,
    query: Query<(&Bot, Entity), Without<ExecutingTask>>,
    mut add_task: EventWriter<AddTaskEvent>,
) {
    for LoadPearl { entity, username } in events.read() {
        if let Ok((state, entity_deref)) = query.get(*entity) {
            // Check if we have trapdoor location
            // `state.pearl_locations.unwrap()` is guranteed to return a value
            if let Some(trapdoor) = state.pearl_locations.to_owned().unwrap().get(username) {
                let anti_afk_config = AntiAFKConfig {
                    jump: true,
                    sneak: false,
                    walk: false,
                    flip_lever: true,
                    central_afk_location: None,
                };

                let trapdoor = *trapdoor;

                add_task.send(AddTaskEvent {
                    entity: entity_deref,
                    task: Task::SetAntiAFK(false, None),
                });

                add_task.send(AddTaskEvent {
                    entity: entity_deref,
                    task: Task::SendChatMessage(format!(
                        "/w {} Teleporting... Make sure to put your pearl back when done!",
                        username
                    )),
                });

                add_task.send(AddTaskEvent {
                    entity: entity_deref,
                    task: Task::GotoTask(trapdoor, false, 2.0),
                });

                add_task.send(AddTaskEvent {
                    entity: entity_deref,
                    task: Task::Delay(Duration::from_millis(500)),
                });

                add_task.send(AddTaskEvent {
                    entity: entity_deref,
                    task: Task::InteractWithBlock(trapdoor),
                });

                add_task.send(AddTaskEvent {
                    entity: entity_deref,
                    task: Task::Delay(Duration::from_millis(500)),
                });

                add_task.send(AddTaskEvent {
                    entity: entity_deref,
                    task: Task::GotoTask(state.afk_location.unwrap(), false, 1.0),
                });

                add_task.send(AddTaskEvent {
                    entity: entity_deref,
                    task: Task::SetAntiAFK(true, Some(anti_afk_config)),
                });
            } else {
                add_task.send(AddTaskEvent {
                    entity: entity_deref,
                    task: Task::SendChatMessage(format!(
                        "/w {} Your Pearl Trapdoor isn't registered here.",
                        username
                    )),
                });
            }
        }
    }
}

#[allow(clippy::complexity)]
pub fn handle_executing_task(
    mut events: EventReader<LoadPearl>,
    query: Query<(), With<ExecutingTask>>,
    mut send_chat_event: EventWriter<SendChatEvent>,
) {
    for LoadPearl { entity, username } in events.read() {
        if let Ok(()) = query.get(*entity) {
            send_chat_event.send(SendChatEvent {
                entity: *entity,
                content: format!(
                    "/w {} I am executing a task rn. Please try again in a while.",
                    username
                ),
            });
        }
    }
}
