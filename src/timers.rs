use std::time::Duration;

use bevy::utils::HashMap;

use crate::*;

#[derive(Resource, Default, Reflect)]
pub struct Timers {
    map: HashMap<Entity, Timer>,
}

impl Timers {
    pub fn add_timer(&mut self, entity: Entity, speed: u8) {
        self.map.insert(
            entity,
            Timer::new(Duration::from_secs(speed.into()), TimerMode::Repeating),
        );
    }

    pub fn update_timer_speed(&mut self, entity: &Entity, speed: u8) {
        self.map
            .get_mut(entity)
            .unwrap()
            .set_duration(Duration::from_secs(speed.into()));
    }
}

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RestockMarketEvent>()
            .init_resource::<Timers>()
            .add_system(tick_timers)
            .add_system(restock_market);
    }
}

struct RestockMarketEvent;

fn restock_market(
    mut market_inventory: ResMut<MarketInventory>,
    mut restock_market_events: EventReader<RestockMarketEvent>,
) {
    for _ in restock_market_events.iter() {
        *market_inventory = MarketInventory::default();
    }
}

fn tick_timers(
    mut timers: ResMut<Timers>,
    mut buildings: Query<(Entity, &mut Building)>,
    time: Res<Time>,
    upgrade_data: Res<UpgradeData>,
    mut yield_stats_text: Query<(&mut Text, &YieldCountText)>,
    selected_building: ResMut<SelectedBuilding>,
    mut send_restock_market_event: EventWriter<RestockMarketEvent>,
) {
    for (entity, timer) in timers.map.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            // get the building

            let mut target_building = None;

            for (building_entity, building) in buildings.iter_mut() {
                if *entity == building_entity {
                    target_building = Some(building)
                }
            }

            if target_building.is_none() {
                return;
            }

            match target_building.as_ref().unwrap().building_type {
                BuildingType::Market => {
                    send_restock_market_event.send(RestockMarketEvent);
                }
                BuildingType::Construct => panic!("This is impossible"),
                _ => {
                    // Add items to the building's yield
                    let mut building = target_building.unwrap();
                    // let mut yields = &mut building.yields;
                    let add = upgrade_data.map[&building.building_type][&building.level].yields;

                    for (item_type, qty) in building.yields.iter_mut() {
                        for item in add {
                            if item.0 == *item_type {
                                *qty += item.1 as u32;
                            }
                        }
                    }

                    if selected_building.building == Some(*entity) {
                        for (mut text, YieldCountText { position }) in yield_stats_text.iter_mut() {
                            text.sections[0].value = format!("x{}", building.yields[*position].1);
                        }
                    }
                }
            }

            // match the building type and action from there
        }
    }
}
