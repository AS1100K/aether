use crate::config::Config;
use azalea::app::{App, Plugin};

pub struct ConfigResourcePlugin;

impl Plugin for ConfigResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config::default());
    }
}
