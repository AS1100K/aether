mod food;

use std::cmp::PartialOrd;
use azalea::app::{App, Plugin, Update};
use azalea::ecs::prelude::*;
use azalea::entity::metadata::{Player, ShiftKeyDown};
use azalea::entity::LocalEntity;
use azalea::inventory::{InventoryComponent, ItemSlot, Menu};
use azalea::prelude::*;
use azalea::registry::Item;
use azalea::Hunger;
use std::collections::HashSet;
use azalea::interact::CurrentSequenceNumber;
use azalea::packet_handling::game::SendPacketEvent;
use azalea::protocol::packets::game::serverbound_interact_packet::InteractionHand;
use azalea::protocol::packets::game::serverbound_use_item_packet::ServerboundUseItemPacket;
use azalea::world::MinecraftEntityId;
use crate::auto_eat::food::Foods;

pub struct AutoEatPlugin;

impl Plugin for AutoEatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartAutoEat>()
            .add_event::<StopAutoEat>()
            .add_systems(
                Update,
                (handle_start_auto_eat, handle_stop_auto_eat).chain(),
            )
            .add_systems(GameTick, handle_auto_eat);
    }
}

#[derive(Event)]
pub struct StartAutoEat {
    /// Will check for food in inventory, Default -> true
    pub use_inventory: bool,
    /// Check for food in the nearest chest, Default -> false
    pub check_nearest_chest: bool,
    /// Check for food in the nearest shulker box, Default -> false
    pub check_nearest_shulker: bool,
    /// Check for food in the ender chest: Default -> false
    pub use_ender_chest: bool,
}

impl Default for StartAutoEat {
    fn default() -> Self {
        Self {
            use_inventory: true,
            check_nearest_chest: false,
            check_nearest_shulker: false,
            use_ender_chest: false,
        }
    }
}

#[derive(Event)]
pub struct StopAutoEat;

#[derive(Component)]
pub struct AutoEat {
    use_inventory: bool,
    check_nearest_chest: bool,
    check_nearest_shulker: bool,
    use_ender_chest: bool,
    executing_mini_tasks: bool,
    next_food_to_eat: Option<Item>,
    foods: Foods,
    max_hunger: u8,
}

fn handle_start_auto_eat(
    mut events: EventReader<StartAutoEat>,
    query: Query<Entity, (With<Player>, With<LocalEntity>)>,
    mut commands: Commands,
) {
    for event in events.read() {
        for entity in query.iter() {
            let mut entity_commands = commands.entity(entity);
            entity_commands.insert(AutoEat {
                use_inventory: event.use_inventory,
                check_nearest_chest: event.check_nearest_chest,
                check_nearest_shulker: event.check_nearest_shulker,
                use_ender_chest: event.use_ender_chest,
                executing_mini_tasks: false,
                next_food_to_eat: None,
                foods: Default::default(),
                max_hunger: 14
            });
        }
    }
}

fn handle_stop_auto_eat(
    mut events: EventReader<StopAutoEat>,
    query: Query<Entity, (With<Player>, With<LocalEntity>, With<AutoEat>)>,
    mut commands: Commands,
) {
    for _event in events.read() {
        for entity in query.iter() {
            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<AutoEat>();
        }
    }
}

fn handle_auto_eat(
    mut query: Query<
        (
            Entity,
            &MinecraftEntityId,
            &ShiftKeyDown,
            &mut AutoEat,
            &mut InventoryComponent,
            &Hunger,
            &CurrentSequenceNumber
        ),
        (With<AutoEat>, With<LocalEntity>, With<Player>),
    >,
    mut send_packet_event: EventWriter<SendPacketEvent>
) {
    for (
        entity,
        minecraft_entity_id,
        shift_key_down,
        mut auto_eat,
        mut inventory_component,
        hunger,
        current_sequence_number
    ) in query.iter_mut() {
        if hunger.food <= auto_eat.max_hunger as u32 && !auto_eat.executing_mini_tasks {
            // TODO: Move the food to the hotbar and select it
            // TODO: If no food is available check in ender chest and nearest chest
            send_packet_event.send(SendPacketEvent {
                entity,
                packet: ServerboundUseItemPacket{
                    hand: InteractionHand::MainHand,
                    sequence: **current_sequence_number,
                }.get(),
            });
        }
    }
}

fn handle_chane_in_inventory(
    mut query: Query<
        (&mut AutoEat, &InventoryComponent),
        (
            With<Player>,
            With<LocalEntity>,
            Changed<InventoryComponent>,
            With<AutoEat>,
        ),
    >,
) {
    for (mut auto_eat, inventory_component) in query.iter_mut() {
        let mut food_available: HashSet<Item> = HashSet::new();
        let mut food_to_eat: Option<Item> = None;
        let mut max_hunger: u8 = 14;
        let menu = inventory_component.menu();

        // This is guaranteed to be `Menu::Player`
        if let Menu::Player(player) = menu {
            let inventory = &player.inventory;
            for item_slot in inventory.iter() {
                if let ItemSlot::Present(item_slot_data) = item_slot {
                    let item = item_slot_data.kind;
                    if auto_eat.foods.0.contains_key(&item) {
                        food_available.insert(item);
                    }
                }
            }
        }

        for food in food_available.iter() {
            if let Some(finalized_food_item) = food_to_eat {
                let finalized_food_item_info_option = auto_eat.foods.0.get(&finalized_food_item);
                let food_info_option = auto_eat.foods.0.get(&food);

                if let Some(finalized_food_item_info) = finalized_food_item_info_option
                    && let Some(food_info) = food_info_option
                {
                    let finalized_food_nourishment = &finalized_food_item_info.nourishment;
                    let food_nourishment = &food_info.nourishment;

                    if food_nourishment > finalized_food_nourishment {
                        max_hunger = auto_eat.foods.0.get(food).unwrap().food_points as u8;
                        food_to_eat = Some(*food)
                    }
                }
            } else {
                max_hunger = auto_eat.foods.0.get(food).unwrap().food_points as u8;
                food_to_eat = Some(*food);
            }
        }

        auto_eat.next_food_to_eat = food_to_eat;
        auto_eat.max_hunger = max_hunger;
    }
}
