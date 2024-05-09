use azalea::prelude::*;
// use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let account = Account::offline("_aether");
    // or Account::microsoft("example@example.com").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost:12345")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            println!("{}", m.message().to_ansi());
        }
        _ => {}
    }

    Ok(())
}
