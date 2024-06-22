# Auto-Mine Plugin

This plugin is the implementation of holding left-click and mines the block it is looking at.

## How to use this Plugin

Refer to the [example](../../examples/stone-miner) for better information on using this plugin.

Add this line to `cargo.toml`
```toml
[dependencies]
azalea-auto-mine = { git = "https://github.com/AS1100K/aether" }
```

## Example

```rust,no_run
use azalea::prelude::*;
use azalea_auto_mine::AutoMinePlugin;
use azalea_auto_mine::AutoMineExt;

#[tokio::main]
async fn main() {
    let account = Account::offline("_aether");
    
    ClientBuilder::new()
        .set_handler(handle)
        .add_plugins(AutoMinePlugin)
        .start(account, "10.9.12.3")
        .await
        .unwrap();
}

#[derive(Component, Clone, Default)]
struct State;

async fn handle(client: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            client.auto_mine(true);
        }
        _ => {}
    }
    Ok(())
}
```