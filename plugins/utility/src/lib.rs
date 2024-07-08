#![feature(let_chains)]

pub mod auto_eat;
pub mod client;

use crate::auto_eat::AutoEatPlugin;
use azalea::app::{PluginGroup, PluginGroupBuilder};
use azalea::ecs::prelude::*;

/// Collection of basic utility plugins
pub struct UtilityPlugin;

impl PluginGroup for UtilityPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(AutoEatPlugin)
    }
}
