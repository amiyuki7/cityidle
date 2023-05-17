use std::time::Duration;

use bevy::utils::HashMap;
use rand::seq::IteratorRandom;

use crate::*;

#[derive(Resource, Default, Reflect)]
pub struct Timers {
    map: HashMap<Entity, Timer>,
}

impl Timers {
    pub fn add_timer(&mut self, entity: Entity, speed: u8) {
        let mut timer = Timer::new(Duration::from_secs(speed.into()), TimerMode::Repeating);
        // Allow the timer to run once almost instantly on spawn - its just so much nicer this way
        timer.set_elapsed(Duration::from_secs((speed - 1).into()));
        self.map.insert(entity, timer);
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
            .add_event::<AfterBoostUIEvent>()
            .init_resource::<Timers>()
            .add_systems((tick_timers, restock_market, after_boost_ui));
    }
}

struct RestockMarketEvent;

fn restock_market(
    mut market_inventory: ResMut<MarketInventory>,
    mut restock_market_events: EventReader<RestockMarketEvent>,
    previous_camera_state: Res<PreviousCameraState>,
    mut send_change_camera_state_event: EventWriter<ChangeCameraStateEvent>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    ui_state: Res<State<UiState>>,
) {
    for _ in restock_market_events.iter() {
        *market_inventory = MarketInventory::default();

        if ui_state.0 == UiState::Market {
            next_ui_state.set(UiState::None);
            send_change_camera_state_event.send(ChangeCameraStateEvent(previous_camera_state.0.clone().unwrap()));
        }
    }
}

struct AfterBoostUIEvent {
    boosted_items: Vec<ItemType>,
}

fn after_boost_ui(
    mut after_boost_ui_events: EventReader<AfterBoostUIEvent>,
    mut boost_images: Query<(&mut UiImage, &BoostImage)>,
    item_icons: Res<ItemIcons>,
    mut item_stats_sell_price: Query<&mut Text, With<ItemStatsSellPrice>>,
    selected_item_stats: Res<SelectedItemStats>,
) {
    for event in after_boost_ui_events.iter() {
        for (mut image, boost_image) in boost_images.iter_mut() {
            if let Some(item_type) = boost_image.item_type {
                if event.boosted_items.contains(&item_type) {
                    image.texture = item_icons.boost.clone();
                } else {
                    image.texture = item_icons.empty.clone();
                }
            }
        }

        if let Ok(mut text) = item_stats_sell_price.get_single_mut() {
            text.sections[0].value = format!("Sell for: ${}", selected_item_stats.sell_price);
        }
    }
}

#[allow(clippy::complexity)]
fn tick_timers(
    mut timers: ResMut<Timers>,
    mut buildings: Query<(Entity, &mut Building)>,
    time: Res<Time>,
    upgrade_data: Res<UpgradeData>,
    mut yield_stats_text: Query<(&mut Text, &YieldCountText)>,
    selected_building: ResMut<SelectedBuilding>,
    mut send_restock_market_event: EventWriter<RestockMarketEvent>,
    mut send_after_boost_ui_event: EventWriter<AfterBoostUIEvent>,
    mut inventory: ResMut<Inventory>,
    mut selected_item_stats: ResMut<SelectedItemStats>,
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
                // The fact that the type is Construct has no semantic meaning - it's only being
                // used because CityCentre is already occupied with another timer
                BuildingType::Construct => {
                    // Reset boosts and sell price
                    for item in inventory.items.iter_mut() {
                        item.boosted = false;
                        item.sell_price = Item::get_sell_price(item.item_type);

                        // If the user stays on the same item, drop the boost
                        if let Some(item_type) = selected_item_stats.item_type {
                            if item_type == item.item_type {
                                selected_item_stats.sell_price = item.sell_price;
                            }
                        }
                    }

                    let mut rng = rand::thread_rng();

                    // Choose 5 random item types to boost
                    let random_types = inventory
                        .items
                        .iter()
                        .map(|item| item.item_type)
                        .choose_multiple(&mut rng, 5);

                    for item in inventory.items.iter_mut() {
                        if random_types.contains(&item.item_type) {
                            item.boosted = true;
                            item.sell_price = Item::get_sell_price(item.item_type) * 2;
                        }
                    }

                    // If the user stays on the same item, manifest the boost
                    if let Some(item_type) = selected_item_stats.item_type {
                        if random_types.contains(&item_type) {
                            selected_item_stats.sell_price = inventory
                                .items
                                .iter()
                                .find(|item| item.item_type == item_type)
                                .unwrap()
                                .sell_price;
                        }
                    }

                    // Send event to update all required UI components
                    send_after_boost_ui_event.send(AfterBoostUIEvent {
                        boosted_items: random_types,
                    });
                }
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
