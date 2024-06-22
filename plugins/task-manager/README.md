# Azalea Task Manager Plugin

This plugin is the task manager that executes tasks one by one, currently it supports very limited tasks,
but in near future it will support more. For better example on how to use this plugin, checkout
[`aether-core`](../../aether-core).

## Todos

- [ ] Support Discord Plugin (WIP)
- [x] Integrate with Anti-AFK Plugin
- [x] Support multiple tasks like interaction, etc.
- [ ] Support for task that can send custom event, insert components.

## How to use this plugin

Firstly, add this to your dependencies i.e. inside `Cargo.toml`
```toml
azalea-task-manager = { git = "https://github.com/as1100k/aether" }
```

### Example

```rust,no_run
use std::time::Duration;
use azalea::prelude::*;
use azalea::BlockPos;
use azalea_task_manager::TaskManagerPlugin;
use azalea_task_manager::client::TaskManagerExt;
use azalea_task_manager::task_manager_queue::Task;
use azalea_task_manager::AddTaskEvent;

#[tokio::main]
async fn main() {
   let account = Account::offline("_aether");

   ClientBuilder::new()
           .set_handler(handle)
           .add_plugins(TaskManagerPlugin)
           .start(account, "10.9.12.3")
           .await
           .unwrap();
}

#[derive(Component, Clone, Default)]
struct State;

async fn handle(client: Client, event: Event, state: State) -> anyhow::Result<()> {
   match event {
      Event::Login => {
         let blockpos = BlockPos::new(0, 0, 0);
         let blockpos_new = BlockPos::new(100, 0, 50);
         
         let _ = client
                 .new_task(Task::GotoTask(blockpos, false, 2.0))
                 .new_task(Task::Delay(Duration::from_secs(2)))
                 .new_task(Task::GotoTask(blockpos_new, false, 2.0));
         
         // Or send event
         client.ecs.lock().send_event(AddTaskEvent {
            entity: client.entity,
            task: Task::Delay(Duration::from_secs(2))
         });
      }
      _ => {}
   }
   Ok(())
}
```

## Supported Tasks
For Latest information see [`task_manager_queue.rs`](./src/task_manager_queue.rs).

1. `GotoTask(target, allow_mining, distance)`
2. `InteractWithBlock(target)`
3. `SendChatMessage(message)`
4. `Delay(Duration)`
5. `SetAntiAFK(enabled)` -> Only with feature: `anti-afk`