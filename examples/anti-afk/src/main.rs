use azalea::prelude::*;
use azalea::Vec3;
use azalea_anti_afk::config::AntiAFKConfig;
use azalea_anti_afk::{AntiAFKClientExt, AntiAFKPlugin};
use azalea_utility::client::UtilityExt;
use azalea_utility::UtilityPlugin;

#[tokio::main]
async fn main() {
    let account = Account::offline("_aether");

    ClientBuilder::new()
        .set_handler(handle)
        .add_plugins(AntiAFKPlugin)
        .add_plugins(UtilityPlugin)
        .start(account, "localhost")
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
                walk: true,
                flip_lever: true,
                central_afk_location: Some(Vec3::new(0f64, 0f64, 0f64)),
            };
            bot.set_anti_afk(true, Some(anti_afk_config));
        }
        Event::Login => {
            bot.set_auto_eat(Some(Default::default()))
        }
        Event::Chat(m) => {
            println!("{}", m.message().to_ansi());
        }
        _ => {}
    }

    Ok(())
}
