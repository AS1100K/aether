use std::collections::VecDeque;
use std::time::Duration;
use azalea::BlockPos;

#[derive(Default)]
pub struct TaskManagerQueue {
    tasks: VecDeque<Task>
}

pub enum Task {
    /// Uses azalea's pathfinding to go the `BlockPos`.
    ///
    /// It taskes two parameters,
    ///
    /// `target:` [`BlockPos`](azalea::BlockPos) -> Target Position
    ///
    /// `allow_mining: bool` -> Is the bot allowed to mine
    ///
    /// Note: Next Task will be executed once, the bot has reached there
    GotoTask(BlockPos, bool),
    /// Sends chat message
    ///
    /// Implementation of [`bot.send_chat_packet()`](azalea::Client::send_chat_packet)
    SendChatMessage(String),
    /// As the name suggest, it adds delay between tasks
    Delay(Duration)
}

impl TaskManagerQueue {
    /// Adds a single task to the TaskManagerQueue.
    pub fn add(&mut self, task: Task) {
        self.tasks.push_back(task)
    }

    /// Adds multiple task to the TaskManagerQueue
    pub fn add_multiple(&mut self, tasks: Vec<Task>) {
        for task in tasks {
            self.add(task)
        }
    }

    /// Removes the first task
    pub fn remove(&mut self) {
        self.pop_front();
    }

    /// Returns the length of tasks
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Implementation of [`VecDeque::get`](std::collections::VecDeque::get)
    pub fn get(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }
}