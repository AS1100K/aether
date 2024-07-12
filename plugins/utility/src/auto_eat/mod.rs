mod food;

use crate::auto_eat::food::Foods;
use azalea::app::{App, Plugin, Update};
use azalea::ecs::prelude::*;
use azalea::entity::metadata::Player;
use azalea::entity::LocalEntity;
use azalea::interact::CurrentSequenceNumber;
use azalea::inventory::{ContainerClickEvent, InventoryComponent, InventorySet, ItemSlot, Menu, SetContainerContentEvent, SetSelectedHotbarSlotEvent};
use azalea::packet_handling::game::SendPacketEvent;
use azalea::prelude::*;
use azalea::protocol::packets::game::serverbound_interact_packet::InteractionHand;
use azalea::protocol::packets::game::serverbound_use_item_packet::ServerboundUseItemPacket;
use azalea::registry::Item;
use azalea::Hunger;
use std::cmp::PartialEq;
use std::collections::HashSet;
use azalea::inventory::operations::{ClickOperation, SwapClick};

pub struct AutoEatPlugin;

impl Plugin for AutoEatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartAutoEat>()
            .add_event::<StopAutoEat>()
            .add_systems(
                Update,
                (handle_start_auto_eat, handle_stop_auto_eat).chain(),
            )
            .add_systems(
                GameTick,
                (handle_change_in_inventory, handle_auto_eat)
                    .chain()
                    .after(InventorySet),
            );
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
    mini_task: MiniTask,
    next_food_to_eat: Option<NextFoodToEat>,
    foods: Foods,
    max_hunger: u8,
}

#[derive(Eq, Hash, PartialEq)]
struct NextFoodToEat {
    kind: Item,
    slot: u16
}

#[derive(PartialEq)]
enum MiniTask {
    /// Puts the food back to the slot
    PutFoodBack(u16),
    /// Searches the food in chests
    SearchFoodInChests,
    None
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
                mini_task: MiniTask::None,
                next_food_to_eat: None,
                foods: Default::default(),
                max_hunger: 14,
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
            &mut AutoEat,
            &InventoryComponent,
            &Hunger,
            &CurrentSequenceNumber,
        ),
        (With<AutoEat>, With<LocalEntity>, With<Player>),
    >,
    mut send_packet_event: EventWriter<SendPacketEvent>,
    mut container_click_event: EventWriter<ContainerClickEvent>,
    mut set_selected_hotbar_slot_event: EventWriter<SetSelectedHotbarSlotEvent>
) {
    for (entity, mut auto_eat, inventory_component, hunger, current_sequence_number) in
        query.iter_mut()
    {
        if hunger.food <= (20 - auto_eat.max_hunger as u32) && auto_eat.mini_task != MiniTask::SearchFoodInChests {
            println!("Hunger -> {}", hunger.food);
            if let Some(next_food_to_eat) = &auto_eat.next_food_to_eat {
                // If 7th slot in hot bar isn't selected, select it
                if inventory_component.selected_hotbar_slot != 7 {
                    set_selected_hotbar_slot_event.send(SetSelectedHotbarSlotEvent {
                        entity,
                        slot: 7,
                    });

                    return;
                }

                if let ItemSlot::Present(item_held) = inventory_component.held_item() && item_held.kind == next_food_to_eat.kind {
                    // Continue Eating...
                    send_packet_event.send(SendPacketEvent {
                        entity,
                        packet: ServerboundUseItemPacket {
                            hand: InteractionHand::MainHand,
                            sequence: **current_sequence_number,
                        }
                            .get(),
                    });
                } else {
                    // Food is somewhere in the inventory, move it to the hotbar
                    container_click_event.send(ContainerClickEvent {
                        entity,
                        window_id: inventory_component.id,
                        operation: ClickOperation::Swap(SwapClick {
                            source_slot: next_food_to_eat.slot,
                            target_slot: 7,
                        }),
                    });

                    // After Eating the food, put it back where it came from
                    auto_eat.mini_task = MiniTask::PutFoodBack(next_food_to_eat.slot);
                    auto_eat.executing_mini_tasks = true;

                    return;
                }
            } else {
                // TODO: If no food is available check in ender chest and nearest chest
                // TODO: Integration with Discord Plugin
            }
        } else if let MiniTask::PutFoodBack(slot) = auto_eat.mini_task {
            // Put the food back
            container_click_event.send(ContainerClickEvent {
                entity,
                window_id: inventory_component.id,
                operation: ClickOperation::Swap(SwapClick {
                    source_slot: slot,
                    target_slot: 7,
                }),
            });

            auto_eat.mini_task = MiniTask::None;
            auto_eat.executing_mini_tasks = false;

            return;
        }
    }
}

fn handle_change_in_inventory(
    // mut events: EventReader<SetContainerContentEvent>,
    mut query: Query<
        (&mut AutoEat, &InventoryComponent),
        (
            With<Player>,
            With<LocalEntity>,
            With<AutoEat>,
            Changed<InventoryComponent>
        ),
    >,
) {
    for (mut auto_eat, inventory_component) in query.iter_mut() {
        let mut food_available: HashSet<NextFoodToEat> = HashSet::new();
        let mut food_to_eat: Option<NextFoodToEat> = None;
        let mut max_hunger: u8 = 14;
        let menu = inventory_component.menu().slots();

        let inventory = menu.iter();

        // Searchable slots that should be searched for food.
        // NOTE: offhand slot is always included.
        let searchable_slots = if auto_eat.use_inventory {
            // Ignore slots from 0 to 8 as they are either of armor or crafting
            inventory.to_owned().skip(8)
        } else {
            // Skips the entire inventory except hotbar
            inventory.to_owned().skip(35)
        };

        for (slot, item_slot) in searchable_slots.enumerate() {
            if let ItemSlot::Present(item_slot_data) = item_slot {
                let item = item_slot_data.kind;
                if auto_eat.foods.0.contains_key(&item) {
                    food_available.insert(NextFoodToEat {
                        kind: item,
                        slot: (slot + 8) as u16
                    });
                }
            }
        }

        for food in food_available.into_iter() {
            if let Some(finalized_food_item) = &food_to_eat {
                let finalized_food_item_info_option = auto_eat.foods.0.get(&finalized_food_item.kind);
                let food_info_option = auto_eat.foods.0.get(&food.kind);

                if let Some(finalized_food_item_info) = finalized_food_item_info_option
                    && let Some(food_info) = food_info_option
                {
                    let finalized_food_nourishment = &finalized_food_item_info.nourishment;
                    let food_nourishment = &food_info.nourishment;

                    if food_nourishment > finalized_food_nourishment {
                        max_hunger = auto_eat.foods.0.get(&food.kind).unwrap().food_points as u8;
                        food_to_eat = Some(food)
                    }
                }
            } else {
                max_hunger = auto_eat.foods.0.get(&food.kind).unwrap().food_points as u8;
                food_to_eat = Some(food);
            }
        }

        auto_eat.next_food_to_eat = food_to_eat;
        auto_eat.max_hunger = max_hunger;
    }
}
