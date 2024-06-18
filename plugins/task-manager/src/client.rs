use azalea::Client;
use crate::task_manager_queue::Task;
use crate::TaskManagerRes;

pub trait TaskManagerExt {
    fn new_task(self, task: Task) -> Self;
}

impl TaskManagerExt for Client {
    /// Adds a new task to the task queue
    fn new_task(self, task: Task) -> Self {
        let mut ecs = self.ecs.lock();

        let mut task_manager = ecs.resource_mut::<TaskManagerRes>();
        task_manager.queue.add(task);

        return self;
    }
}