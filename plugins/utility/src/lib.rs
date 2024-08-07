#![doc = include_str!("../README.md")]
#![feature(let_chains)]

pub mod auto_eat;
pub mod auto_totem;
pub mod client;

use crate::auto_eat::AutoEatPlugin;
use auto_totem::AutoTotemPlugin;
use azalea::app::{PluginGroup, PluginGroupBuilder};

/// Collection of basic utility plugins
pub struct UtilityPlugin;

impl PluginGroup for UtilityPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AutoEatPlugin)
            .add(AutoTotemPlugin)
    }
}
