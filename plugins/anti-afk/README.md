# Anti-AFK Plugin

This is an advanced anti-afk plugin that do actions like swing, jump, sneak, interact, walk _(actions are customizable)_
in every 3 seconds to avoid getting AFK kick.

## How to use this Plugin
Refer to the [example](../../examples/anti-afk) for better information on using this plugin.

Add this line to `cargo.toml`
```toml
[dependencies]
azalea-anti-afk = { git = "https://github.com/AS1100K/aether" }
```

### Example

```rust,no_run
use azalea_anti_afk::AntiAFKPlugin;
use azalea_anti_afk::config::AntiAFKConfig;
use azalea_anti_afk::AntiAFKClientExt;
use azalea::prelude::*;

#[tokio::main]
async fn main() {
    let account = Account::offline("_aether");

    ClientBuilder::new()
        .set_handler(handle)
        .add_plugins(AntiAFKPlugin)
        .start(account, "10.9.12.3")
        .await
        .unwrap();
}

#[derive(Component, Clone, Default)]
struct State;

async fn handle(client: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            let anti_afk_config = AntiAFKConfig {
                jump: true,
                sneak: true,
                walk: true,
                flip_lever: true,
                // It is recommended to provide `central_afk_location` for more stable `walk`
                central_afk_location: None
            };
            client.set_anti_afk(true, Some(anti_afk_config));
        }
        _ => {}
    }
    Ok(())
}
```

## Actions Available
| Action               | Chances | Customizable |
|----------------------|---------|--------------|
| Random Head Rotation | 50%     | No           |
| Swing Arm            | 12.5%   | No           |
| Jump                 | 12.5%   | Yes          |
| Sneak                | 12.5%   | Yes          |
| Walk                 | 12.5%   | Yes          |
| Flipping the lever   | 25%     | Yes          |

_Customizable Actions means that they can be turned off and on._