use azalea::{
    app::{App, Plugin},
    ecs::prelude::*,
    entity::Position, interact::HitResultComponent, mining::StartMiningBlockEvent,
    prelude::*
};
use azalea::physics::PhysicsSet;

pub struct AutoMinePlugin;

impl Plugin for AutoMinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            handle_auto_mine
                .after(PhysicsSet)
        );
    }
}

pub trait AutoMineExt {
    fn auto_mine(&self, enabled: bool);
}

impl AutoMineExt for Client {
    fn auto_mine(&self, enabled: bool) {
        let mut ecs = self.ecs.lock();
        let mut entity_mut = ecs.entity_mut(self.entity);

        if enabled {
            entity_mut.insert(AutoMine);
        } else {
            entity_mut.remove::<AutoMine>();
        }
    }
}

#[derive(Component, Clone)]
pub struct AutoMine;

fn handle_auto_mine(
    mut query: Query<(&HitResultComponent, &Position, Entity), With<AutoMine>>,
    mut start_mining_block_event_writer: EventWriter<StartMiningBlockEvent>
) {
    for (hit_result_component, position, entity) in &mut query.iter_mut() {
        let block_pos = hit_result_component.block_pos;

        if position.distance_to(&block_pos.to_vec3_floored()) <= 7.0 {
            start_mining_block_event_writer.send(StartMiningBlockEvent {
                entity,
                position: block_pos,
            });
        }
    }
}