#![feature(let_chains)]

mod chat;
mod client;
mod command;
mod commands;
mod config;
mod config_res;
mod discord;
mod plugins;
mod utils;

use crate::client::{handle_death, handle_init};
use std::time::Duration;

use azalea::{prelude::*, swarm::prelude::*};
use azalea_anti_afk::AntiAFKPlugin;
use azalea_task_manager::TaskManagerPlugin;
use azalea_utility::UtilityPlugin;
use bevy_discord::bot::serenity::all::{GatewayIntents, OnlineStatus};
use bevy_discord::bot::serenity::gateway::ActivityData;
use bevy_discord::bot::{DiscordBotConfig, DiscordBotPlugin};
use plugins::AetherDefaultPlugins;
use tracing::info;

use crate::config::{Bot, Config, Mode};
use crate::config_res::ConfigResourcePlugin;

#[tokio::main]
async fn main() {
    let config: Config = Config::default();

    let server_url: String = config.server.clone();
    let mut discord_bot_activity = String::from("Bots under control: \n");

    let mut swarm = SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .add_plugins(ConfigResourcePlugin)
        .add_plugins(AntiAFKPlugin)
        .add_plugins(TaskManagerPlugin)
        .add_plugins(UtilityPlugin)
        .add_plugins(AetherDefaultPlugins)
        .join_delay(Duration::from_secs(5));

    for (bot_username, bot) in config.bots {
        let account = if bot.mode == Mode::Offline {
            Account::offline(bot.username.as_str())
        } else {
            Account::microsoft(bot.email.clone().unwrap().as_str())
                .await
                .unwrap_or_else(|_| {
                    panic!(
                        "Unable to login via microsoft for {}",
                        bot_username.as_str()
                    )
                })
        };

        if config.discord_bot_token.is_some() {
            discord_bot_activity.insert_str(
                discord_bot_activity.len(),
                format!("{}\n", &account.username).as_str(),
            );
        }

        swarm = swarm.add_account_with_state(account, bot);
    }

    if let Some(token) = config.discord_bot_token {
        let discord_bot_config = DiscordBotConfig::default()
            .activity(ActivityData::playing(discord_bot_activity))
            .gateway_intents(
                GatewayIntents::GUILD_MESSAGES
                    | GatewayIntents::MESSAGE_CONTENT
                    | GatewayIntents::DIRECT_MESSAGES,
            )
            .token(token)
            .status(OnlineStatus::Online);

        swarm = swarm.add_plugins(DiscordBotPlugin::new(discord_bot_config));
    }

    swarm
        .start(server_url.as_str())
        .await
        .expect("Unable to start the swarm");
}

async fn handle(client: Client, event: Event, state: Bot) -> anyhow::Result<()> {
    match event {
        Event::Init => handle_init(client, state).await?,
        Event::Death(death) => handle_death(client, state, death).await?,
        _ => {}
    }

    Ok(())
}

async fn swarm_handle(mut swarm: Swarm, event: SwarmEvent, state: Config) -> anyhow::Result<()> {
    if let SwarmEvent::Disconnect(account, _join_opts) = event {
        info!(
            "{} got disconnected from the server. Reconnecting...",
            account.username
        );

        tokio::time::sleep(Duration::from_secs(3)).await;
        let bot_state = state.bots.get(&*account.username).unwrap_or_else(|| {
            panic!(
                "Unable to find the bot with the username: {} in `config.json",
                account.username
            )
        });

        swarm
            .add_and_retry_forever(&account, bot_state.to_owned())
            .await;
    }

    Ok(())
}
