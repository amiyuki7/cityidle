use crate::*;
use std::collections::HashMap;

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
                    upgrade_materials: [(ItemType::Branch, 20), (ItemType::Nougat, 20), (ItemType::Coffee, 20)],
                    upgrade_cost: 1000,
                },
            );

            stats_map.insert(
                2,
                LevelStats {
                    speed: 12,
                    yields: [
                        (ItemType::BronzeCoin, 14),
                        (ItemType::SilverCoin, 2),
                        (ItemType::GoldCoin, 0),
                    ],
                    upgrade_materials: [(ItemType::Branch, 60), (ItemType::Coffee, 90), (ItemType::Cocoa, 25)],
                    upgrade_cost: 5500,
                },
            );

            stats_map
        });

        Self { map }
    }
}
