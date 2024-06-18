# Azalea Task Manager Plugin

Currently, this plugin is WIP and there are a lot of things that can be added to it.

> [!NOTE]
> This Plugin is currently unstable and might not work as expected

## Todos

- [ ] Support Discord Plugin (WIP)
- [ ] Support multiple tasks like interaction, etc.
- [ ] Support for task that can send custom event, insert components.

## How to use this plugin

Firstly, add this to your dependencies i.e. inside `Cargo.toml`
```toml
azalea-task-manager = { git = "https://github.com/as1100k/aether" }
```

Now, add the plugin to your `main.rs`
```text
    ClientBuilder::new()
        .set_handler(handle)
        .add_plugins(TaskManagerPlugin)
        ...
```

Now, to create a new task, there are two ways:

1. Use `azalea::Client` trait implementation:
    ```rust
    use azalea::Client;
    use azalea_task_manager::client::TaskManagerExt;
    use azalea_task_manager::task_manager_queue::Task;
        
    fn handle(bot: Client, state: State) -> anyhow::Result<()> {
        // --snip--
            let _ = bot
                        .next_task(Task::Goto(blockpos, false, 2.0))
                        .next_task(Task::Goto(blockpos_new, true, 2.0));
    }    
    ```    

2. Send bevy `Event` (WIP)

## Supported Tasks
For Latest information see [`task_manager_queue.rs`](./src/task_manager_queue.rs).

1. `GotoTask(target, allow_mining, distance)`
2. `InteractWithBlock(target)`
3. `SendChatMessage(message)`
4. `Delay(Duration)`