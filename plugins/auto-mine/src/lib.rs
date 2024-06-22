#![doc = include_str!("../README.md")]

use azalea::{app::{App, Plugin}, BlockPos, ecs::prelude::*, entity::Position, interact::HitResultComponent, mining::StartMiningBlockEvent, prelude::*};
use azalea::entity::LocalEntity;
use azalea::entity::metadata::Player;
use azalea::inventory::InventoryComponent;
use azalea::mining::{MineBlockPos, MineItem, Mining};
use azalea::physics::PhysicsSet;

pub struct AutoMinePlugin;

impl Plugin for AutoMinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            handle_auto_mine
                .before(PhysicsSet)
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
    mut query: Query<
        (
            &HitResultComponent,
            &Position,
            Entity,
            Option<&Mining>,
            &InventoryComponent,
            &MineBlockPos,
            &MineItem,
        ),
        (With<AutoMine>, With<Player>, With<LocalEntity>),
    >,
    mut start_mining_block_event_writer: EventWriter<StartMiningBlockEvent>,
) {
    for (
        hit_result_component,
        position,
        entity,
        mining,
        inventory,
        current_mining_pos,
        current_mining_item,
    ) in &mut query.iter_mut()
    {
        let block_pos = hit_result_component.block_pos;

        if (mining.is_none()
            || !is_same_mining_target(
            block_pos,
            inventory,
            current_mining_pos,
            current_mining_item,
        ))
            && position.distance_to(&block_pos.to_vec3_floored()) <= 7.0
        {
            start_mining_block_event_writer.send(StartMiningBlockEvent {
                entity,
                position: block_pos,
            });
        }
    }
}

// This code block is copied from https://azalea.matdoes.dev/src/azalea_client/mining.rs.html#290-298
fn is_same_mining_target(
    target_block: BlockPos,
    inventory: &InventoryComponent,
    current_mining_pos: &MineBlockPos,
    current_mining_item: &MineItem,
) -> bool {
    let held_item = inventory.held_item();
    Some(target_block) == current_mining_pos.0 && held_item == current_mining_item.0
}