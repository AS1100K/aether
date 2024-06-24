# Azalea Discord Plugin

A very simple, discord plugin that let you send messages through discord webhooks. _In Future releases, this plugin will support 
discord applications & bots and can send & receive messages by them._

## Example

```rust,no_run
use azalea_discord::DiscordPlugin;
use azalea_discord::DiscordExt;
use azalea_discord::SendDiscordMessage;
use azalea::prelude::*;

#[tokio::main]
async fn main() {
    let account = azalea::Account::offline("_aether");

    ClientBuilder::new()
        .set_handler(handle)
        .add_plugins(DiscordPlugin)
        .start(account, "10.9.12.3")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            bot.send_discord_message(SendDiscordMessage {
                webhook: "https://discord.com".to_string(),
                contents: "Logged into the server".to_string(),
                username: None,
                avatar_url: None
            });
        }
        _ => {}
    }

    Ok(())
}
```