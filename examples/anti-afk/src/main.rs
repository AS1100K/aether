use azalea::prelude::*;
use azalea_anti_afk::{AntiAFKClientExt, AntiAFKPlugin};
use azalea_anti_afk::config::AntiAFKConfig;

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

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Init => {
            let anti_afk_config = AntiAFKConfig {
                jump: true,
                sneak: true,
                walk: Default::default(),
                walk_distance: None,
                central_afk_location: None,
                flip_lever: true,
            };
            bot.set_anti_afk(true, Some(anti_afk_config));
        }
        Event::Chat(m) => {
            println!("{}", m.message().to_ansi());
        }
        _ => {}
    }

    Ok(())
}
