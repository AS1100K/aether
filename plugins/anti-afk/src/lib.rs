use std::time::Instant;
use azalea::app::{App, Plugin};
use azalea::ecs::system::Query;
use azalea::prelude::*;

pub struct AntiAFKPlugin;

impl Plugin for AntiAFKPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            anti_afk
        );
    }
}

pub trait AntiAFKClientExt {
    fn set_anti_afk(&self, enabled: bool);
}

impl AntiAFKClientExt for Client {
    fn set_anti_afk(&self, enabled: bool) {
        let mut ecs = self.ecs.lock();
        let mut entity_mut = ecs.entity_mut(self.entity);

        if enabled {
            entity_mut.insert(AntiAFK {
                last_afk_tick: Instant::now()
            });
        } else {
            entity_mut.remove::<AntiAFK>();
        }

    }
}

#[derive(Component, Clone)]
pub struct AntiAFK {
    last_afk_tick: Instant
}

fn anti_afk(
    mut query: Query<&mut AntiAFK>
) {
    for anti_afk in &mut query {
        println!("Hello World");
    }
}