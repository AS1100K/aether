use azalea::app::{PluginGroup, PluginGroupBuilder};

use crate::{chat::ChatPlugin, commands::AetherCommandsPlugin, discord::AetherDiscordPlugin};

pub struct AetherDefaultPlugins;

impl PluginGroup for AetherDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ChatPlugin)
            .add(AetherCommandsPlugin)
            .add(AetherDiscordPlugin)
    }
}
