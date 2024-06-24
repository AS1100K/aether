use azalea::BlockPos;
use azalea_anti_afk::config::AntiAFKConfig;
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Default, Clone)]
pub struct TaskManagerQueue {
    tasks: VecDeque<Task>,
}

#[derive(Clone)]
pub enum Task {
    /// Uses azalea's pathfinding to go the `BlockPos`.
    ///
    /// It tasks two parameters,
    ///
    /// `target:` [`BlockPos`](azalea::BlockPos) -> Target Position
    ///
    /// `allow_mining: bool` -> Is the bot allowed to mine
    ///
    /// `distance: f64` -> The distance at which pathfinding should stop. It should have
    /// minimum value of `2.0` (including error).
    ///
    /// Note: Next Task will be executed once, the bot has reached there
    GotoTask(BlockPos, bool, f64),
    /// This is the implementation of [`Client::block_interact()`](azalea::Client::block_interact)
    ///
    /// Note: The bot needs to be in the range to interact with the block
    InteractWithBlock(BlockPos),
    /// Sends chat message
    ///
    /// Implementation of [`bot.send_chat_packet()`](azalea::Client::send_chat_packet)
    SendChatMessage(String),
    /// As the name suggest, it adds delay between tasks
    Delay(Duration),
    #[cfg(feature = "anti-afk")]
    /// This task, can set [`AntiAFK`](azalea_anti_afk::AntiAFK) state to `false` or `true`.
    ///
    /// NOTE: You need to add the plugin [`AntiAFKPlugin`](azalea_anti_afk::AntiAFKPlugin) before
    /// sending this task.
    ///
    /// This task is equivalent to [`bot.set_anti_afk()`](azalea_anti_afk::AntiAFKClientExt::set_anti_afk).
    /// Also, this is not as effective to it because it can overwrite component if enabled twice.
    /// This is not a deal-breaker for `AntiAFKPlugin`.
    SetAntiAFK(bool, Option<AntiAFKConfig>),
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
        self.tasks.pop_front();
    }

    #[allow(clippy::len_without_is_empty)]
    /// Returns the length of tasks
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Implementation of [`VecDeque::get`](std::collections::VecDeque::get)
    pub fn get(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }
}
