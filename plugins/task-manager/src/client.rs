use crate::task_manager_queue::Task;
use crate::{AddTaskEvent, TaskManager};
use azalea::ecs::prelude::{EventReader, With};
use azalea::ecs::system::Query;
use azalea::entity::metadata::Player;
use azalea::entity::LocalEntity;
use azalea::Client;

pub trait TaskManagerExt {
    fn new_task(&self, task: Task) -> &Self;
    fn len_tasks(&self) -> usize;
}

impl TaskManagerExt for Client {
    /// Adds a new task to the task queue
    fn new_task(&self, task: Task) -> &Self {
        self.ecs.lock().send_event(AddTaskEvent {
            entity: self.entity,
            task,
        });

        self
    }

    fn len_tasks(&self) -> usize {
        let task_manager = self.component::<TaskManager>();

        task_manager.queue.len()
    }
}

#[allow(clippy::complexity)]
pub(crate) fn handle_add_task_event(
    mut events: EventReader<AddTaskEvent>,
    mut query: Query<&mut TaskManager, (With<TaskManager>, With<Player>, With<LocalEntity>)>,
) {
    for event in events.read() {
        let mut task_manager = query.get_mut(event.entity.to_owned()).unwrap();
        task_manager.queue.add(event.task.to_owned())
    }
}
