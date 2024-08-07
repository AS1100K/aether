use azalea::{app::Plugin, prelude::{Component, GameTick}};
use pearl::load::{handle_executing_task, handle_load_peral, LoadPearl};

pub mod pearl;

/// This Component is present when the bot has receieved a command and is executing it
#[derive(Component)]
pub struct ExecutingTask;

pub struct AetherCommandsPlugin;

impl Plugin for AetherCommandsPlugin {
    fn build(&self, app: &mut azalea::app::App) {
        app
        .add_event::<LoadPearl>()
        .add_systems(GameTick, handle_load_peral)
        .add_systems(GameTick, handle_executing_task);
    }
}