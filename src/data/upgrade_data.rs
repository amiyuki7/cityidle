use crate::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LevelStats {
    pub speed: u8,
    pub yields: [(ItemType, u8); 3],
    pub upgrade_materials: [(ItemType, u32); 3],
    pub upgrade_cost: u32,
}

#[derive(Resource)]
pub struct UpgradeData {
    pub map: HashMap<BuildingType, HashMap<u8, LevelStats>>,
}

impl Default for UpgradeData {
    fn default() -> Self {
        let mut map = HashMap::new();

        map.insert(BuildingType::CityCentre, {
            let mut stats_map = HashMap::new();
            stats_map.insert(
                1,
                LevelStats {
                    speed: 15,
                    yields: [
                        (ItemType::BronzeCoin, 10),
                        (ItemType::SilverCoin, 0),
                        (ItemType::GoldCoin, 0),
                    ],
                    upgrade_materials: [(ItemType::Branch, 20), (ItemType::Taffy, 50), (ItemType::Coffee, 50)],
                    upgrade_cost: 1000,
                },
            );

            stats_map.insert(
                2,
                LevelStats {
                    speed: 12,
                    yields: [
                        (ItemType::BronzeCoin, 15),
                        (ItemType::SilverCoin, 3),
                        (ItemType::GoldCoin, 0),
                    ],
                    upgrade_materials: [(ItemType::Log, 40), (ItemType::Nougat, 38), (ItemType::Cocoa, 38)],
                    upgrade_cost: 5500,
                },
            );

            stats_map
        });

        map.insert(BuildingType::CandyShop, {
            let mut stats_map = HashMap::new();
            stats_map.insert(
                1,
                LevelStats {
                    speed: 15,
                    yields: [(ItemType::Taffy, 10), (ItemType::Nougat, 1), (ItemType::Marshmallow, 0)],
                    upgrade_materials: [(ItemType::Taffy, 60), (ItemType::Nougat, 10), (ItemType::Steel, 5)],
                    upgrade_cost: 800,
                },
            );

            stats_map.insert(
                2,
                LevelStats {
                    speed: 12,
                    yields: [(ItemType::Taffy, 18), (ItemType::Nougat, 6), (ItemType::Marshmallow, 0)],
                    upgrade_materials: [
                        (ItemType::Nougat, 30),
                        (ItemType::BronzeCoin, 35),
                        (ItemType::Steel, 18),
                    ],
                    upgrade_cost: 1600,
                },
            );

            stats_map
        });

        map.insert(BuildingType::CoffeeShop, {
            let mut stats_map = HashMap::new();
            stats_map.insert(
                1,
                LevelStats {
                    speed: 15,
                    yields: [(ItemType::Coffee, 10), (ItemType::Cocoa, 2), (ItemType::Milkshake, 0)],
                    upgrade_materials: [(ItemType::Coffee, 60), (ItemType::Cocoa, 20), (ItemType::Branch, 5)],
                    upgrade_cost: 1000,
                },
            );

            stats_map.insert(
                2,
                LevelStats {
                    speed: 12,
                    yields: [(ItemType::Coffee, 20), (ItemType::Cocoa, 5), (ItemType::Milkshake, 1)],
                    upgrade_materials: [
                        (ItemType::Cocoa, 35),
                        (ItemType::BronzeCoin, 35),
                        (ItemType::Branch, 30),
                    ],
                    upgrade_cost: 2100,
                },
            );

            stats_map
        });

        map.insert(BuildingType::Tree, {
            let mut stats_map = HashMap::new();
            stats_map.insert(
                1,
                LevelStats {
                    speed: 15,
                    yields: [(ItemType::Apple, 8), (ItemType::Branch, 2), (ItemType::Honey, 0)],
                    upgrade_materials: [
                        (ItemType::Apple, 60),
                        (ItemType::BronzeCoin, 30),
                        (ItemType::Branch, 16),
                    ],
                    upgrade_cost: 1700,
                },
            );

            stats_map.insert(
                2,
                LevelStats {
                    speed: 12,
                    yields: [(ItemType::Apple, 20), (ItemType::Branch, 6), (ItemType::Honey, 1)],
                    upgrade_materials: [
                        (ItemType::Branch, 40),
                        (ItemType::SilverCoin, 30),
                        (ItemType::Honey, 10),
                    ],
                    upgrade_cost: 2600,
                },
            );

            stats_map
        });

        map.insert(BuildingType::Factory, {
            let mut stats_map = HashMap::new();
            stats_map.insert(
                1,
                LevelStats {
                    speed: 15,
                    yields: [(ItemType::Steel, 6), (ItemType::Chip, 1), (ItemType::Phone, 0)],
                    upgrade_materials: [(ItemType::Steel, 36), (ItemType::Chip, 10), (ItemType::Log, 20)],
                    upgrade_cost: 2500,
                },
            );

            stats_map.insert(
                2,
                LevelStats {
                    speed: 12,
                    yields: [(ItemType::Steel, 14), (ItemType::Chip, 3), (ItemType::Phone, 0)],
                    upgrade_materials: [(ItemType::Steel, 120), (ItemType::Phone, 5), (ItemType::Log, 50)],
                    upgrade_cost: 5600,
                },
            );

            stats_map
        });

        map.insert(BuildingType::Cabin, {
            let mut stats_map = HashMap::new();
            stats_map.insert(
                1,
                LevelStats {
                    speed: 15,
                    yields: [(ItemType::Log, 6), (ItemType::Lantern, 2), (ItemType::Axe, 0)],
                    upgrade_materials: [(ItemType::Log, 36), (ItemType::BronzeCoin, 35), (ItemType::Branch, 50)],
                    upgrade_cost: 2400,
                },
            );

            stats_map.insert(
                2,
                LevelStats {
                    speed: 12,
                    yields: [(ItemType::Log, 14), (ItemType::Lantern, 5), (ItemType::Axe, 0)],
                    upgrade_materials: [(ItemType::Log, 130), (ItemType::SilverCoin, 30), (ItemType::Axe, 5)],
                    upgrade_cost: 5300,
                },
            );

            stats_map
        });

        Self { map }
    }
}
