use azalea::{
    app::{Plugin, Update},
    ecs::prelude::*,
    entity::{metadata::Player, LocalEntity},
    inventory::{
        operations::{ClickOperation, SwapClick},
        ContainerClickEvent, InventoryComponent, ItemSlot, Menu,
    },
    prelude::*,
};

pub struct AutoTotemPlugin;

impl Plugin for AutoTotemPlugin {
    fn build(&self, app: &mut azalea::app::App) {
        app.add_event::<NoTotemAvailable>()
            .add_event::<EnableAutoTotem>()
            .add_event::<DisableAutoTotem>()
            .add_systems(GameTick, handle_auto_totem)
            .add_systems(Update, enable_auto_totem)
            .add_systems(Update, disable_auto_totem);
    }
}

/// Component which is present when `AutoTotem` is enabled
#[derive(Component)]
pub struct AutoTotem;

/// This Event is send when NoTotems are available
///
/// NOTE: This Event will be send every GameTick
#[derive(Event)]
pub struct NoTotemAvailable;

/// When this event is send, AutoTotem is enabled
#[derive(Event)]
pub struct EnableAutoTotem {
    pub entity: Entity,
}

/// When this event is send, AutoTotem is disabled
#[derive(Event)]
pub struct DisableAutoTotem {
    pub entity: Entity,
}

#[allow(clippy::complexity)]
fn handle_auto_totem(
    query: Query<(Entity, &InventoryComponent), (With<AutoTotem>, With<Player>, With<LocalEntity>)>,
    mut container_click_event: EventWriter<ContainerClickEvent>,
    mut no_totem_available: EventWriter<NoTotemAvailable>,
) {
    for (entity, inventory_component) in query.iter() {
        // This is guaranteed to be a `Menu::Player`
        if let Menu::Player(player_inventory) = &inventory_component.inventory_menu {
            let offhand_item = &player_inventory.offhand;

            if offhand_item.kind() != azalea::registry::Item::TotemOfUndying {
                // Totem is not present in the offhand move one

                let mut totem_index: Option<usize> = None;
                for (i, item) in inventory_component
                    .menu()
                    .slots()
                    .iter()
                    .skip(8)
                    .enumerate()
                {
                    if let ItemSlot::Present(item_kind) = item {
                        if item_kind.kind == azalea::registry::Item::TotemOfUndying {
                            // Ignore slots from 0 to 8 as they are either of armor or crafting
                            totem_index = Some(i + 8);
                            break;
                        }
                    }
                }

                if let Some(index) = totem_index {
                    container_click_event.send(ContainerClickEvent {
                        entity,
                        window_id: inventory_component.id,
                        operation: ClickOperation::Swap(SwapClick {
                            source_slot: index as u16,
                            // For Offhand button needed to be 40
                            target_slot: 40,
                        }),
                    });
                } else {
                    // No Totems Available
                    no_totem_available.send(NoTotemAvailable);
                }
            }
        }
    }
}

fn enable_auto_totem(mut events: EventReader<EnableAutoTotem>, mut commands: Commands) {
    for event in events.read() {
        commands.entity(event.entity).insert(AutoTotem);
    }
}

fn disable_auto_totem(mut events: EventReader<DisableAutoTotem>, mut commands: Commands) {
    for event in events.read() {
        commands.entity(event.entity).remove::<AutoTotem>();
    }
}
