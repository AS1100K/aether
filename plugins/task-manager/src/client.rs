use azalea::Client;
use azalea::ecs::prelude::*;
use crate::task_manager_queue::Task;
use crate::TaskManager;

pub trait TaskManagerExt {
    fn new_task(&self, task: Task) -> Self;
}

impl TaskManagerExt for Client {
    /// Adds a new task to the task queue
    fn new_task(&self, task: Task) -> Self {
        todo!()
    }
}

#[derive(Event)]
pub struct AddTaskEvent {
    task: Task
}

pub fn handle_add_task_event(
    mut events: EventReader<AddTaskEvent>,
    mut query: Query<(), With<TaskManager>>
) {
    todo!()
}