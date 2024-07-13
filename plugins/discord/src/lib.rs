#![doc = include_str!("../README.md")]

use bevy_ecs::schedule::SystemSet;

pub mod webhook;
pub mod common;
mod runtime;

/// Bevy [`SystemSet`] that contains all system of this plugin.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscordSet;