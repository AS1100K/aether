use azalea::registry::Item;
use std::collections::HashMap;

/// List of Food items that can be consumed
pub(super) struct Foods(pub HashMap<Item, FoodInfo>);

pub(super) struct FoodInfo {
    pub(super) food_points: f32,
    pub(super) nourishment: Nourishment,
}

#[derive(PartialEq, PartialOrd)]
pub(super) enum Nourishment {
    Great = 5,
    Good = 4,
    Normal = 3,
    Low = 2,
    Poor = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nourishment_1() {
        let n1 = Nourishment::Good;
        let n2 = Nourishment::Low;

        assert_eq!(true, n1 > n2)
    }

    #[test]
    fn test_nourishment_2() {
        let n1 = Nourishment::Great;
        let n2 = Nourishment::Normal;

        assert_eq!(false, n1 < n2)
    }
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
                nourishment: Nourishment::Great,
            },
        );
        // 2. Golden Apple
        foods.insert(
            Item::GoldenApple,
            FoodInfo {
                food_points: 4.0,
                nourishment: Nourishment::Great,
            },
        );
        // 3. Golden Carrot
        foods.insert(
            Item::GoldenCarrot,
            FoodInfo {
                food_points: 6.0,
                nourishment: Nourishment::Great,
            },
        );

        // Good Nourishment
        // 1. Cooked Mutton
        foods.insert(
            Item::CookedMutton,
            FoodInfo {
                food_points: 6.0,
                nourishment: Nourishment::Good,
            },
        );
        // 2. Cooked Porkchop
        foods.insert(
            Item::CookedPorkchop,
            FoodInfo {
                food_points: 8.0,
                nourishment: Nourishment::Good,
            },
        );
        // 3. Cooked Salmon
        foods.insert(
            Item::CookedSalmon,
            FoodInfo {
                food_points: 6.0,
                nourishment: Nourishment::Good,
            },
        );
        // 4. Steak
        foods.insert(
            Item::CookedBeef,
            FoodInfo {
                food_points: 8.0,
                nourishment: Nourishment::Good,
            },
        );

        // Normal Nourishment
        // 1. Baked Potato
        foods.insert(
            Item::BakedPotato,
            FoodInfo {
                food_points: 5.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 2. Beetroot
        foods.insert(
            Item::Beetroot,
            FoodInfo {
                food_points: 1.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 3. Beetroot Soup
        foods.insert(
            Item::BeetrootSoup,
            FoodInfo {
                food_points: 6.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 4. Bread
        foods.insert(
            Item::Bread,
            FoodInfo {
                food_points: 5.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 5. Carrot
        foods.insert(
            Item::Carrot,
            FoodInfo {
                food_points: 3.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 6. Cooked Chicken
        foods.insert(
            Item::CookedChicken,
            FoodInfo {
                food_points: 6.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 7. Cooked Cod
        foods.insert(
            Item::CookedCod,
            FoodInfo {
                food_points: 5.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 8. Cooked Rabbit
        foods.insert(
            Item::CookedRabbit,
            FoodInfo {
                food_points: 5.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 9. Mushroom Stew
        foods.insert(
            Item::MushroomStew,
            FoodInfo {
                food_points: 6.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 10. Rabbit Stew
        foods.insert(
            Item::RabbitStew,
            FoodInfo {
                food_points: 10.0,
                nourishment: Nourishment::Normal,
            },
        );
        // 11. Suspicious Stew
        foods.insert(
            Item::SuspiciousStew,
            FoodInfo {
                food_points: 6.0,
                nourishment: Nourishment::Normal,
            },
        );

        // Low Nourishment
        // 1. Apple
        foods.insert(
            Item::Apple,
            FoodInfo {
                food_points: 4.0,
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
                nourishment: Nourishment::Low,
            },
        );
        // 4. Melon Slice
        foods.insert(
            Item::MelonSlice,
            FoodInfo {
                food_points: 2.0,
                nourishment: Nourishment::Low,
            },
        );
        // 5. Poisonous Potato (Not Supported)
        // foods.insert(
        //     Item::PoisonousPotato,
        //     FoodInfo {
        //         food_points: 2.0,
        //         saturation_restored: 1.2,
        //         saturation_ratio: 0.6,
        //         effect: vec![MobEffect::Poison],
        //         nourishment: Nourishment::Low,
        //     },
        // );
        // 6. Potato
        foods.insert(
            Item::Potato,
            FoodInfo {
                food_points: 1.0,
                nourishment: Nourishment::Low,
            },
        );
        // 7. Pumpkin Pie
        foods.insert(
            Item::PumpkinPie,
            FoodInfo {
                food_points: 8.0,
                nourishment: Nourishment::Low,
            },
        );
        // 8. Raw Beef
        foods.insert(
            Item::Beef,
            FoodInfo {
                food_points: 3.0,
                nourishment: Nourishment::Low,
            },
        );
        // 9. Raw Chicken (Not Supported)
        // foods.insert(
        //     Item::Chicken,
        //     FoodInfo {
        //         food_points: 2.0,
        //         saturation_restored: 1.2,
        //         saturation_ratio: 0.6,
        //         effect: vec![MobEffect::Hunger],
        //         nourishment: Nourishment::Low,
        //     },
        // );
        // 10. Raw Mutton
        foods.insert(
            Item::Mutton,
            FoodInfo {
                food_points: 2.0,
                nourishment: Nourishment::Low,
            },
        );
        // 11. Raw Porkchop
        foods.insert(
            Item::Porkchop,
            FoodInfo {
                food_points: 3.0,
                nourishment: Nourishment::Low,
            },
        );
        // 12. Sweet Berries
        foods.insert(
            Item::SweetBerries,
            FoodInfo {
                food_points: 2.0,
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
                nourishment: Nourishment::Poor,
            },
        );
        // 3. Glow Berries
        foods.insert(
            Item::GlowBerries,
            FoodInfo {
                food_points: 2.0,
                nourishment: Nourishment::Poor,
            },
        );
        // 4. Honey Bottle
        foods.insert(
            Item::HoneyBottle,
            FoodInfo {
                food_points: 6.0,
                nourishment: Nourishment::Poor,
            },
        );
        // 5. Pufferfish (Not Supported)
        // foods.insert(
        //     Item::Pufferfish,
        //     FoodInfo {
        //         food_points: 1.0,
        //         saturation_restored: 0.2,
        //         saturation_ratio: 0.2,
        //         effect: vec![MobEffect::Hunger, MobEffect::Nausea, MobEffect::Poison],
        //         nourishment: Nourishment::Poor,
        //     },
        // );
        // 6. Raw Cod
        foods.insert(
            Item::Cod,
            FoodInfo {
                food_points: 2.0,
                nourishment: Nourishment::Poor,
            },
        );
        // 7. Raw Salmon
        foods.insert(
            Item::Salmon,
            FoodInfo {
                food_points: 2.0,
                nourishment: Nourishment::Poor,
            },
        );
        // 8. Rotten Flesh (Not Supported)
        // foods.insert(
        //     Item::RottenFlesh,
        //     FoodInfo {
        //         food_points: 4.0,
        //         saturation_restored: 0.8,
        //         saturation_ratio: 0.2,
        //         effect: vec![MobEffect::Hunger],
        //         nourishment: Nourishment::Poor,
        //     },
        // );
        // 9. Spider Eye (Not Supported)
        // foods.insert(
        //     Item::SpiderEye,
        //     FoodInfo {
        //         food_points: 2.0,
        //         saturation_restored: 3.2,
        //         saturation_ratio: 1.6,
        //         effect: vec![MobEffect::Poison],
        //         nourishment: Nourishment::Poor,
        //     },
        // );
        // 10. Tropical Fish
        foods.insert(
            Item::TropicalFish,
            FoodInfo {
                food_points: 1.0,
                nourishment: Nourishment::Poor,
            },
        );

        Self(foods)
    }
}
