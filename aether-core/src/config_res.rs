use azalea::app::{App, Plugin};
use crate::config::Config;

pub struct ConfigResourcePlugin;

impl Plugin for ConfigResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config::default());
    }
}