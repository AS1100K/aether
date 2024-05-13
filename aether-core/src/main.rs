use azalea::prelude::*;
// use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let account = Account::offline("_aether");
    // or Account::microsoft("example@example.com").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "10.9.12.173:12345")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(client: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            let (username, command) = m.split_sender_and_content();

            if username.is_none() || username.unwrap() == "_aether" {
                return Ok(())
            }

            match command.as_str() {
                "!load" => {
                    // Pearl Loading Code goes here
                }
                _ => {
                    client.chat("IDK!");
                }
            }
        }
        _ => {}
    }

    Ok(())
}
