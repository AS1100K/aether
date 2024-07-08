use azalea::app::{App, Plugin, Update};
use azalea::ecs::prelude::*;
use azalea::entity::metadata::Player;
use azalea::entity::LocalEntity;
use azalea::inventory::InventoryComponent;
use azalea::prelude::*;
use azalea::registry::{Item, MobEffect};
use std::collections::HashMap;

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
    executing_mini_tasks: bool
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
                executing_mini_tasks: false
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
        (Entity, &mut AutoEat, &mut InventoryComponent),
        (With<AutoEat>, With<LocalEntity>, With<Player>),
    >,
) {
    for (entity, mut auto_eat, mut inventory_component) in query.iter_mut() {
        if !auto_eat.executing_mini_tasks {
            todo!()
        }
    }
}

/// List of Food items that can be consumed
struct Foods(HashMap<Item, FoodInfo>);

struct FoodInfo {
    food_points: f32,
    saturation_restored: f32,
    saturation_ratio: f32,
    effect: Vec<MobEffect>,
    nourishment: Nourishment,
}

enum Nourishment {
    Great,
    Good,
    Normal,
    Low,
    Poor,
}

impl Default for Foods {
    fn default() -> Self {
        let mut foods: HashMap<Item, FoodInfo> = HashMap::new();
        // Great Nourishment
        // 1. Enchanted Golden Apple
        foods.insert(
            Item::EnchantedGoldenApple,
            FoodInfo {
                food_points: 4.0,
                saturation_restored: 9.6,
                saturation_ratio: 2.4,
                effect: vec![
                    MobEffect::Absorption,
                    MobEffect::Regeneration,
                    MobEffect::Resistance,
                    MobEffect::FireResistance,
                ],
                nourishment: Nourishment::Great,
            },
        );
        // 2. Golden Apple
        foods.insert(
            Item::GoldenApple,
            FoodInfo {
                food_points: 4.0,
                saturation_restored: 9.6,
                saturation_ratio: 2.4,
                effect: vec![MobEffect::Regeneration, MobEffect::Absorption],
                nourishment: Nourishment::Great,
            },
        );
        // 3. Golden Carrot
        foods.insert(
            Item::GoldenCarrot,
            FoodInfo {
                food_points: 6.0,
                saturation_restored: 14.4,
                saturation_ratio: 2.4,
                effect: vec![],
                nourishment: Nourishment::Great,
            },
        );

        // Good Nourishment
        // 1. Cooked Mutton
        foods.insert(
            Item::CookedMutton,
            FoodInfo {
                food_points: 6.0,
                saturation_restored: 9.6,
                saturation_ratio: 1.6,
                effect: vec![],
                nourishment: Nourishment::Good,
            },
        );
        // 2. Cooked Porkchop
        foods.insert(
            Item::CookedPorkchop,
            FoodInfo {
                food_points: 8.0,
                saturation_restored: 12.8,
                saturation_ratio: 1.6,
                effect: vec![],
                nourishment: Nourishment::Good,
            },
        );
        // 3. Cooked Salmon
        foods.insert(
            Item::CookedSalmon,
            FoodInfo {
                food_points: 6.0,
                saturation_restored: 9.6,
                saturation_ratio: 1.6,
                effect: vec![],
                nourishment: Nourishment::Good,
            },
        );
        // 4. Steak
        foods.insert(
            Item::CookedBeef,
            FoodInfo {
                food_points: 8.0,
                saturation_restored: 12.8,
                saturation_ratio: 1.6,
                effect: vec![],
                nourishment: Nourishment::Good,
            },
        );

        // Normal Nourishment
        // 1. Baked Potato
        foods.insert(
            Item::BakedPotato,
            FoodInfo {
                food_points: 5.0,
                saturation_restored: 6.0,
                saturation_ratio: 0.0,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 2. Beetroot
        foods.insert(
            Item::Beetroot,
            FoodInfo {
                food_points: 1.0,
                saturation_restored: 1.2,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 3. Beetroot Soup
        foods.insert(
            Item::BeetrootSoup,
            FoodInfo {
                food_points: 6.0,
                saturation_restored: 7.2,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 4. Bread
        foods.insert(
            Item::Bread,
            FoodInfo {
                food_points: 5.0,
                saturation_restored: 6.0,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 5. Carrot
        foods.insert(
            Item::Carrot,
            FoodInfo {
                food_points: 3.0,
                saturation_restored: 3.6,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 6. Cooked Chicken
        foods.insert(
            Item::CookedChicken,
            FoodInfo {
                food_points: 6.0,
                saturation_restored: 7.2,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 7. Cooked Cod
        foods.insert(
            Item::CookedCod,
            FoodInfo {
                food_points: 5.0,
                saturation_restored: 6.0,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 8. Cooked Rabbit
        foods.insert(
            Item::CookedRabbit,
            FoodInfo {
                food_points: 5.0,
                saturation_restored: 6.0,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 9. Mushroom Stew
        foods.insert(
            Item::MushroomStew,
            FoodInfo {
                food_points: 6.0,
                saturation_restored: 7.2,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 10. Rabbit Stew
        foods.insert(
            Item::RabbitStew,
            FoodInfo {
                food_points: 10.0,
                saturation_restored: 12.0,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );
        // 11. Suspicious Stew
        foods.insert(
            Item::SuspiciousStew,
            FoodInfo {
                food_points: 6.0,
                saturation_restored: 7.2,
                saturation_ratio: 1.2,
                effect: vec![],
                nourishment: Nourishment::Normal,
            },
        );

        // Low Nourishment
        // 1. Apple
        foods.insert(
            Item::Apple,
            FoodInfo {
                food_points: 4.0,
                saturation_restored: 2.4,
                saturation_ratio: 0.6,
                effect: vec![],
                nourishment: Nourishment::Low,
            },
        );
        // 2. Chorus Fruit (Not Supported because of random teleportation)
        // foods.insert(
        //     Item::ChorusFruit,
        //     FoodInfo {
        //         food_points: 4.0,
        //         saturation_restored: 2.4,
        //         saturation_ratio: 0.6,
        //         effect: vec![],
        //         nourishment: Nourishment::Low,
        //     }
        // );
        // 3. Dried Kelp
        foods.insert(
            Item::DriedKelp,
            FoodInfo {
                food_points: 1.0,
                saturation_restored: 0.6,
                saturation_ratio: 0.6,
                effect: vec![],
                nourishment: Nourishment::Low,
            },
        );
        // 4. Melon Slice
        foods.insert(
            Item::MelonSlice,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 1.2,
                saturation_ratio: 0.6,
                effect: vec![],
                nourishment: Nourishment::Low,
            },
        );
        // 5. Poisonous Potato
        foods.insert(
            Item::PoisonousPotato,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 1.2,
                saturation_ratio: 0.6,
                effect: vec![MobEffect::Poison],
                nourishment: Nourishment::Low,
            },
        );
        // 6. Potato
        foods.insert(
            Item::Potato,
            FoodInfo {
                food_points: 1.0,
                saturation_restored: 1.6,
                saturation_ratio: 0.6,
                effect: vec![],
                nourishment: Nourishment::Low,
            },
        );
        // 7. Pumpkin Pie
        foods.insert(
            Item::PumpkinPie,
            FoodInfo {
                food_points: 8.0,
                saturation_restored: 4.8,
                saturation_ratio: 0.6,
                effect: vec![],
                nourishment: Nourishment::Low,
            },
        );
        // 8. Raw Beef
        foods.insert(
            Item::Beef,
            FoodInfo {
                food_points: 3.0,
                saturation_restored: 1.8,
                saturation_ratio: 0.6,
                effect: vec![],
                nourishment: Nourishment::Low,
            },
        );
        // 9. Raw Chicken
        foods.insert(
            Item::Chicken,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 1.2,
                saturation_ratio: 0.6,
                effect: vec![MobEffect::Hunger],
                nourishment: Nourishment::Low,
            },
        );
        // 10. Raw Mutton
        foods.insert(
            Item::Mutton,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 1.2,
                saturation_ratio: 0.6,
                effect: vec![],
                nourishment: Nourishment::Low,
            },
        );
        // 11. Raw Porkchop
        foods.insert(
            Item::Porkchop,
            FoodInfo {
                food_points: 3.0,
                saturation_restored: 1.8,
                saturation_ratio: 0.6,
                effect: vec![],
                nourishment: Nourishment::Low,
            },
        );
        // 12. Sweet Berries
        foods.insert(
            Item::SweetBerries,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 0.4,
                saturation_ratio: 0.2,
                effect: vec![],
                nourishment: Nourishment::Low,
            },
        );

        // Poor Nourishment
        // 1. Cake _(whole)_ (Not Supported because it needs to be place and then consumed)
        // foods.insert(
        //     Item::Cake,
        //     FoodInfo {
        //         food_points: 14.0,
        //         saturation_restored: 2.8,
        //         saturation_ratio: 0.2,
        //         effect: vec![],
        //         nourishment: Nourishment::Poor,
        //     }
        // );
        // 2. Cookie
        foods.insert(
            Item::Cookie,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 0.4,
                saturation_ratio: 0.2,
                effect: vec![],
                nourishment: Nourishment::Poor,
            },
        );
        // 3. Glow Berries
        foods.insert(
            Item::GlowBerries,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 0.4,
                saturation_ratio: 0.2,
                effect: vec![],
                nourishment: Nourishment::Poor,
            },
        );
        // 4. Honey Bottle
        foods.insert(
            Item::HoneyBottle,
            FoodInfo {
                food_points: 6.0,
                saturation_restored: 1.2,
                saturation_ratio: 0.2,
                effect: vec![
                    // Clears Poison
                ],
                nourishment: Nourishment::Poor,
            },
        );
        // 5. Pufferfish
        foods.insert(
            Item::Pufferfish,
            FoodInfo {
                food_points: 1.0,
                saturation_restored: 0.2,
                saturation_ratio: 0.2,
                effect: vec![MobEffect::Hunger, MobEffect::Nausea, MobEffect::Poison],
                nourishment: Nourishment::Poor,
            },
        );
        // 6. Raw Cod
        foods.insert(
            Item::Cod,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 0.4,
                saturation_ratio: 0.2,
                effect: vec![],
                nourishment: Nourishment::Poor,
            },
        );
        // 7. Raw Salmon
        foods.insert(
            Item::Salmon,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 0.4,
                saturation_ratio: 0.2,
                effect: vec![],
                nourishment: Nourishment::Poor,
            },
        );
        // 8. Rotten Flesh
        foods.insert(
            Item::RottenFlesh,
            FoodInfo {
                food_points: 4.0,
                saturation_restored: 0.8,
                saturation_ratio: 0.2,
                effect: vec![MobEffect::Hunger],
                nourishment: Nourishment::Poor,
            },
        );
        // 9. Spider Eye
        foods.insert(
            Item::SpiderEye,
            FoodInfo {
                food_points: 2.0,
                saturation_restored: 3.2,
                saturation_ratio: 1.6,
                effect: vec![MobEffect::Poison],
                nourishment: Nourishment::Poor,
            },
        );
        // 10. Tropical Fish
        foods.insert(
            Item::TropicalFish,
            FoodInfo {
                food_points: 1.0,
                saturation_restored: 0.2,
                saturation_ratio: 0.2,
                effect: vec![],
                nourishment: Nourishment::Poor,
            },
        );

        Self(foods)
    }
}
