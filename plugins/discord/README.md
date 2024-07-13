# Bevy Discord Plugin

![GitHub License](https://img.shields.io/github/license/AS1100K/aether)


A very simple, bevy plugin that let you send messages through discord webhooks. _In Future releases, this plugin will support 
discord applications & bots and can send & receive messages by them._

## Example
This example is shown inside azalea, but this plugin can be used with any bevy app.

```rust,no_run
use azalea::prelude::*;
use azalea::Vec3;
use bevy_discord::common::DiscordMessage;
use bevy_discord::webhook::{DiscordWebhookPlugin, DiscordWebhookRes, SendMessageEvent};

#[tokio::main]
async fn main() {
    let account = Account::offline("_aether");

    let discord_webhook = DiscordWebhookRes::new()
        .add_channel(
            "channel_name",
            "webhook_url",
            "",
            ""
        );
    ClientBuilder::new()
        .set_handler(handle)
        .add_plugins(DiscordWebhookPlugin::new(discord_webhook))
        .start(account, "localhost")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            let content = m.message();
            println!("{}", &content.to_ansi());
            let message = DiscordMessage::new()
                .content(content.to_string());

            bot.ecs.lock().send_event(SendMessageEvent::new("channel_name", message));
        }
        _ => {}
    }

    Ok(())
}
```
