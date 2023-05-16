use crate::*;
use bevy::window::PrimaryWindow;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedBuilding>()
            .add_system(draw_ui.in_schedule(OnEnter(UiState::Upgrade)))
            .add_system(undraw_ui.in_schedule(OnExit(UiState::Upgrade)))
            .add_systems(
                (exit_uistate, collect_button_interaction, upgrade_button_interaction)
                    .in_set(OnUpdate(UiState::Upgrade)),
            );
    }
}

pub fn exit_uistate(
    keybinds: Res<Keybinds>,
    keys: Res<Input<KeyCode>>,
    previous_camera_state: Res<PreviousCameraState>,
    mut send_change_camera_state_event: EventWriter<ChangeCameraStateEvent>,
    mut next_ui_state: ResMut<NextState<UiState>>,
) {
    if !keys.just_pressed(keybinds.exit_uistate) {
        return;
    }

    next_ui_state.set(UiState::None);
    send_change_camera_state_event.send(ChangeCameraStateEvent(previous_camera_state.0.clone().unwrap()));
}

// Marker
#[derive(Component)]
struct RootUINode;

// Marker
#[derive(Component)]
struct LevelText;

// Marker
#[derive(Component)]
struct SpeedText;

#[derive(Component)]
struct YieldStatsText {
    position: usize,
}

#[derive(Component)]
struct UpgradeMaterialImage {
    position: usize,
}

#[derive(Component)]
struct UpgradeMaterialText {
    position: usize,
}

// Marker
#[derive(Component)]
struct BalanceText;

// Marker
#[derive(Component)]
struct CollectButton;

// Marker
#[derive(Component)]
pub struct YieldCountText {
    pub position: usize,
}

// Marker
#[derive(Component)]
struct UpgradeButton;

#[allow(clippy::complexity)]
fn collect_button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CollectButton>)>,
    mut buildings: Query<(Entity, &mut Building)>,
    selected_building: Res<SelectedBuilding>,
    mut inventory: ResMut<Inventory>,
    mut text_set: ParamSet<(
        Query<&mut Text, With<YieldCountText>>,
        Query<(&mut Text, &UpgradeMaterialText), With<UpgradeMaterialText>>,
    )>,
    upgrade_data: Res<UpgradeData>,
) {
    for (interaction, mut background_colour) in interaction_query.iter_mut() {
        let mut target_building = None;

        for (entity, building) in buildings.iter_mut() {
            if selected_building.building.is_none() {
                break;
            }

            if entity == selected_building.building.unwrap() {
                target_building = Some(building)
            }
        }

        // This shouldn't ever happen but it's here just in case
        if target_building.is_none() {
            return;
        }

        match interaction {
            Interaction::Clicked => {
                // Add all yields to inventory and set yields to 0
                target_building
                    .as_mut()
                    .unwrap()
                    .yields
                    .iter_mut()
                    .for_each(|(item_type, quantity)| {
                        inventory
                            .items
                            .iter_mut()
                            .find(|item| item.item_type == *item_type)
                            .unwrap()
                            .quantity += *quantity;

                        *quantity = 0;
                    });

                for mut text in text_set.p0().iter_mut() {
                    text.sections[0].value = "x0".to_string();
                }

                for (mut text, UpgradeMaterialText { position }) in text_set.p1().iter_mut() {
                    let items_required = upgrade_data.map[&target_building.as_ref().unwrap().building_type]
                        [&target_building.as_ref().unwrap().level]
                        .upgrade_materials[*position];

                    let quantity_acquired = inventory
                        .items
                        .iter()
                        .find(|item| item.item_type == items_required.0)
                        .unwrap()
                        .quantity;

                    let quantity_required = items_required.1;

                    let mut colour = Color::GREEN;

                    if quantity_acquired < quantity_required {
                        colour = Color::RED;
                    }

                    let quantity_text = format!("{quantity_acquired}/{quantity_required}");

                    text.sections[0].value = quantity_text;
                    text.sections[0].style.color = colour;
                }
            }
            Interaction::Hovered => *background_colour = Color::rgb(0.34, 0.37, 0.60).into(),
            _ => *background_colour = Color::rgb(0.22, 0.25, 0.48).into(),
        }
    }
}

#[allow(clippy::complexity)]
fn upgrade_button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<UpgradeButton>)>,
    selected_building: Res<SelectedBuilding>,
    upgrade_data: Res<UpgradeData>,
    mut buildings: Query<(Entity, &mut Building)>,
    mut inventory: ResMut<Inventory>,
    mut text_set: ParamSet<(
        Query<&mut Text, With<LevelText>>,
        Query<&mut Text, With<SpeedText>>,
        Query<(&mut Text, &YieldStatsText)>,
        Query<(&mut Text, &UpgradeMaterialText)>,
        Query<&mut Text, With<BalanceText>>,
    )>,
    mut upgrade_material_images: Query<(&mut UiImage, &UpgradeMaterialImage)>,
    item_icons: Res<ItemIcons>,
    mut timers: ResMut<Timers>,
) {
    let mut target_building = None;

    for (entity, building) in buildings.iter_mut() {
        if selected_building.building.is_none() {
            break;
        }

        if entity == selected_building.building.unwrap() {
            target_building = Some(building);
        }
    }

    // This shouldn't ever happen but it's here just in case
    if target_building.is_none() {
        return;
    }

    for (interaction, mut background_colour) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                // Check if can upgrade?
                let building = target_building.as_mut().unwrap();
                let level_stats = &upgrade_data.map[&building.building_type][&building.level];
                let next_level_stats = &upgrade_data.map[&building.building_type].get(&(building.level + 1));

                if next_level_stats.is_none() {
                    return;
                }

                let upgrade_materials = level_stats.upgrade_materials;
                let upgrade_material_types = upgrade_materials.map(|(item_type, _)| item_type);

                let acquired_materials = inventory
                    .items
                    .iter()
                    .filter(|item| upgrade_material_types.contains(&item.item_type))
                    .map(|item| (item.item_type, item.quantity))
                    .collect::<Vec<(ItemType, u32)>>();

                let sufficient_materials = acquired_materials
                    .iter()
                    .flat_map(|pair_a| upgrade_materials.iter().map(move |pair_b| (pair_a, pair_b)))
                    .filter(|(pair_a, pair_b)| pair_a.0 == pair_b.0)
                    .all(|((_, acquired), (_, required))| acquired >= required);

                debug!("ACQUIRED: {acquired_materials:?}");
                debug!("REQUIRED: {upgrade_materials:?}");
                debug!("SUFFICIENT: {sufficient_materials}");

                let sufficient_money = inventory.balance >= level_stats.upgrade_cost;

                if sufficient_materials && sufficient_money {
                    // Update the inventory
                    for item in inventory.items.iter_mut() {
                        for (item_type, quantity) in upgrade_materials {
                            if item.item_type == item_type {
                                item.quantity -= quantity;
                            }
                        }
                    }

                    inventory.balance -= level_stats.upgrade_cost;

                    // Increase building level and speed
                    building.level += 1;
                    building.speed = next_level_stats.unwrap().speed;

                    timers.update_timer_speed(&selected_building.building.unwrap(), building.speed);

                    // As the level changed, these also need to change
                    let level_stats = &upgrade_data.map[&building.building_type][&building.level];
                    let next_level_stats = &upgrade_data.map[&building.building_type].get(&(building.level + 1));

                    // Update all text and images
                    if let Ok(mut level_text) = text_set.p0().get_single_mut() {
                        level_text.sections[0].value = format!("Level: {} → {}", building.level, {
                            if next_level_stats.is_none() {
                                "MAX".to_string()
                            } else {
                                (building.level + 1).to_string()
                            }
                        });
                    }

                    if let Ok(mut speed_text) = text_set.p1().get_single_mut() {
                        speed_text.sections[0].value = format!("Speed: {}s → {}s", level_stats.speed, {
                            if next_level_stats.is_none() {
                                "MAX".to_string()
                            } else {
                                next_level_stats.unwrap().speed.to_string()
                            }
                        });
                    }

                    for (mut text, YieldStatsText { position }) in text_set.p2().iter_mut() {
                        text.sections[0].value = format!("x{} → {}", level_stats.yields[*position].1, {
                            if next_level_stats.is_none() {
                                "MAX".to_string()
                            } else {
                                next_level_stats.unwrap().yields[*position].1.to_string()
                            }
                        });
                    }

                    for (mut image, UpgradeMaterialImage { position }) in upgrade_material_images.iter_mut() {
                        // if next_level_stats.is_none() {
                        //     image.texture = item_icons.empty.clone();
                        // } else {
                        image.texture = match level_stats.upgrade_materials[*position].0 {
                            ItemType::BronzeCoin => item_icons.bronze_coin.clone(),
                            ItemType::SilverCoin => item_icons.silver_coin.clone(),
                            ItemType::GoldCoin => item_icons.gold_coin.clone(),
                            ItemType::Taffy => item_icons.taffy.clone(),
                            ItemType::Nougat => item_icons.nougat.clone(),
                            ItemType::Marshmallow => item_icons.marshmallow.clone(),
                            ItemType::Coffee => item_icons.coffee.clone(),
                            ItemType::Cocoa => item_icons.cocoa.clone(),
                            ItemType::Milkshake => item_icons.milkshake.clone(),
                            ItemType::Apple => item_icons.apple.clone(),
                            ItemType::Branch => item_icons.branch.clone(),
                            ItemType::Honey => item_icons.honey.clone(),
                            ItemType::Steel => item_icons.steel.clone(),
                            ItemType::Chip => item_icons.chip.clone(),
                            ItemType::Phone => item_icons.phone.clone(),
                            ItemType::Log => item_icons.log.clone(),
                            ItemType::Lantern => item_icons.lantern.clone(),
                            ItemType::Axe => item_icons.axe.clone(),
                        };
                        // }
                    }

                    for (mut text, UpgradeMaterialText { position }) in text_set.p3().iter_mut() {
                        let items_required = level_stats.upgrade_materials[*position];

                        let quantity_acquired = inventory
                            .items
                            .iter()
                            .find(|item| item.item_type == items_required.0)
                            .unwrap()
                            .quantity;

                        let quantity_required = items_required.1;

                        let mut colour = Color::GREEN;

                        if quantity_acquired < quantity_required {
                            colour = Color::RED;
                        }

                        let quantity_text = format!("{quantity_acquired}/{quantity_required}");

                        text.sections[0].value = quantity_text;
                        text.sections[0].style.color = colour;
                    }

                    if let Ok(mut balance_text) = text_set.p4().get_single_mut() {
                        balance_text.sections[0].value =
                            format!("${} / ${}", inventory.balance, level_stats.upgrade_cost);

                        balance_text.sections[0].style.color = if inventory.balance >= level_stats.upgrade_cost {
                            Color::GREEN
                        } else {
                            Color::RED
                        };
                    }
                }
            }
            Interaction::Hovered => *background_colour = Color::rgb(0.34, 0.37, 0.60).into(),
            _ => *background_colour = Color::rgb(0.22, 0.25, 0.48).into(),
        }
    }
}

#[derive(Resource, Default)]
pub struct SelectedBuilding {
    pub building: Option<Entity>,
}

#[allow(clippy::complexity)]
fn draw_ui(
    mut commands: Commands,
    inventory: Res<Inventory>,
    asset_server: Res<AssetServer>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    item_icons: Res<ItemIcons>,
    mut buildings: Query<(Entity, &mut Building)>,
    mut upgrade_target_events: EventReader<UpgradeTarget>,
    mut selected_building_resource: ResMut<SelectedBuilding>,
    upgrade_data: Res<UpgradeData>,
) {
    // UI takes up half the screen & renders at a ratio of 1:1.777
    let mut inventory_width = primary_window.single().resolution.width() / 2.0;
    let mut inventory_height = inventory_width / (1920.0 / 1080.0);

    if inventory_height > primary_window.single().resolution.height() {
        inventory_height = primary_window.single().resolution.height() / 2.0;
        inventory_width = inventory_height * (1920.0 / 1080.0);
    }

    let mut target_building = None;

    let target = upgrade_target_events.iter().next().unwrap();

    // if let Some(target) = upgrade_target_events.iter().next() {
    //     // target.target_entity
    for (entity, building) in buildings.iter_mut() {
        if entity == target.target_entity {
            selected_building_resource.building = Some(entity);
            target_building = Some(building);
            break;
        }
    }
    // }

    // 100% sure that this will not panic
    let target_building = target_building.unwrap();

    let level_stats = &upgrade_data.map[&target_building.building_type][&target_building.level];
    // If this is None, then we are at the MAX level
    let next_level_stats = &upgrade_data.map[&target_building.building_type].get(&(target_building.level + 1));

    trace!("TARGET BUILDING {:?}", target_building);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
            ..default()
        })
        .insert(Name::new("UI Root"))
        .insert(RootUINode)
        .with_children(|commands| {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(inventory_width), Val::Px(inventory_height)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::SpaceAround,
                        align_self: AlignSelf::Center,
                        margin: UiRect::left(Val::Px(
                            // Offset required for the centre of inventory width to align with centre of screen
                            (primary_window.single().resolution.width() - inventory_width) / 2.0,
                        )),
                        ..default()
                    },
                    background_color: Color::rgb(0.13, 0.14, 0.26).into(),
                    ..default()
                })
                .insert(Name::new("Layout"))
                .with_children(|commands| {
                    // Left half
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(45.0), Val::Percent(90.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Name::new("Left side container"))
                        .with_children(|commands| {
                            // Container for building name & stats
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(75.0)),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    commands
                                        .spawn(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                                flex_direction: FlexDirection::Column,
                                                justify_content: JustifyContent::SpaceBetween,
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .insert(Name::new("Building title text container"))
                                        .with_children(|commands| {
                                            commands.spawn(TextBundle {
                                                style: Style {
                                                    margin: UiRect::all(Val::Percent(5.0)),
                                                    align_self: AlignSelf::Center,
                                                    ..default()
                                                },
                                                text: Text::from_section(
                                                    target_building.building_type.get_name(),
                                                    TextStyle {
                                                        font: asset_server.load("font.otf"),
                                                        font_size: inventory_width / 36.0,
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                ..default()
                                            });
                                        });

                                    commands
                                        .spawn(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                                flex_direction: FlexDirection::Column,
                                                justify_content: JustifyContent::SpaceAround,
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|commands| {
                                            commands
                                                .spawn(TextBundle {
                                                    style: Style {
                                                        margin: UiRect::all(Val::Percent(5.0)),
                                                        ..default()
                                                    },
                                                    text: Text::from_section(
                                                        format!("Level: {} → {}", target_building.level, {
                                                            if next_level_stats.is_none() {
                                                                "MAX".to_string()
                                                            } else {
                                                                (target_building.level + 1).to_string()
                                                            }
                                                        }),
                                                        TextStyle {
                                                            font: asset_server.load("font.otf"),
                                                            font_size: inventory_width / 42.0,
                                                            color: Color::WHITE,
                                                        },
                                                    ),
                                                    ..default()
                                                })
                                                .insert(LevelText);

                                            commands
                                                .spawn(TextBundle {
                                                    style: Style {
                                                        // margin: UiRect::all(Val::Percent(5.0)),
                                                        margin: UiRect::new(
                                                            Val::Percent(5.0),
                                                            Val::Percent(5.0),
                                                            Val::Percent(5.0),
                                                            Val::Percent(10.0),
                                                        ),
                                                        ..default()
                                                    },
                                                    text: Text::from_section(
                                                        format!("Speed: {}s → {}s", level_stats.speed, {
                                                            if next_level_stats.is_none() {
                                                                "MAX".to_string()
                                                            } else {
                                                                next_level_stats.unwrap().speed.to_string()
                                                            }
                                                        }),
                                                        TextStyle {
                                                            font: asset_server.load("font.otf"),
                                                            font_size: inventory_width / 42.0,
                                                            color: Color::WHITE,
                                                        },
                                                    ),
                                                    ..default()
                                                })
                                                .insert(SpeedText);
                                        });

                                    for i in 0..=2 {
                                        commands
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                                    flex_direction: FlexDirection::Row,
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .with_children(|commands| {
                                                // Image

                                                commands.spawn(ImageBundle {
                                                    style: Style {
                                                        // size: Size::new(Val::Percent(18.0), Val::Percent(100.0)),
                                                        size: Size::new(
                                                            Val::Px(inventory_width / 16.0),
                                                            Val::Px(inventory_width / 16.0),
                                                        ),
                                                        margin: UiRect::left(Val::Percent(2.0)),
                                                        ..default()
                                                    },
                                                    transform: Transform::from_scale(Vec3::splat(0.9)),
                                                    image: UiImage {
                                                        texture: match level_stats.yields[i].0 {
                                                            ItemType::BronzeCoin => item_icons.bronze_coin.clone(),
                                                            ItemType::SilverCoin => item_icons.silver_coin.clone(),
                                                            ItemType::GoldCoin => item_icons.gold_coin.clone(),
                                                            ItemType::Taffy => item_icons.taffy.clone(),
                                                            ItemType::Nougat => item_icons.nougat.clone(),
                                                            ItemType::Marshmallow => item_icons.marshmallow.clone(),
                                                            ItemType::Coffee => item_icons.coffee.clone(),
                                                            ItemType::Cocoa => item_icons.cocoa.clone(),
                                                            ItemType::Milkshake => item_icons.milkshake.clone(),
                                                            ItemType::Apple => item_icons.apple.clone(),
                                                            ItemType::Branch => item_icons.branch.clone(),
                                                            ItemType::Honey => item_icons.honey.clone(),
                                                            ItemType::Steel => item_icons.steel.clone(),
                                                            ItemType::Chip => item_icons.chip.clone(),
                                                            ItemType::Phone => item_icons.phone.clone(),
                                                            ItemType::Log => item_icons.log.clone(),
                                                            ItemType::Lantern => item_icons.lantern.clone(),
                                                            ItemType::Axe => item_icons.axe.clone(),
                                                        },
                                                        ..default()
                                                    },
                                                    ..default()
                                                });

                                                commands
                                                    .spawn(TextBundle {
                                                        style: Style {
                                                            margin: UiRect::all(Val::Percent(5.0)),
                                                            ..default()
                                                        },
                                                        text: Text::from_section(
                                                            format!("x{} → {}", level_stats.yields[i].1, {
                                                                if next_level_stats.is_none() {
                                                                    "MAX".to_string()
                                                                } else {
                                                                    next_level_stats.unwrap().yields[i].1.to_string()
                                                                }
                                                            }),
                                                            TextStyle {
                                                                font: asset_server.load("font.otf"),
                                                                font_size: inventory_width / 42.0,
                                                                color: Color::WHITE,
                                                            },
                                                        ),
                                                        ..default()
                                                    })
                                                    .insert(YieldStatsText { position: i });
                                            });
                                    }
                                });
                            // Container for yield collection
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::top(Val::Percent(5.0)),
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.17, 0.19, 0.36).into(),
                                    ..default()
                                })
                                .with_children(|commands| {
                                    for i in 0..=2 {
                                        commands
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                                                    flex_direction: FlexDirection::Column,
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .with_children(|commands| {
                                                commands.spawn(ImageBundle {
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Px(inventory_width / 30.0),
                                                            Val::Px(inventory_width / 30.0),
                                                        ),
                                                        ..default()
                                                    },
                                                    image: UiImage {
                                                        texture: match target_building.yields[i].0 {
                                                            ItemType::BronzeCoin => item_icons.bronze_coin.clone(),
                                                            ItemType::SilverCoin => item_icons.silver_coin.clone(),
                                                            ItemType::GoldCoin => item_icons.gold_coin.clone(),
                                                            ItemType::Taffy => item_icons.taffy.clone(),
                                                            ItemType::Nougat => item_icons.nougat.clone(),
                                                            ItemType::Marshmallow => item_icons.marshmallow.clone(),
                                                            ItemType::Coffee => item_icons.coffee.clone(),
                                                            ItemType::Cocoa => item_icons.cocoa.clone(),
                                                            ItemType::Milkshake => item_icons.milkshake.clone(),
                                                            ItemType::Apple => item_icons.apple.clone(),
                                                            ItemType::Branch => item_icons.branch.clone(),
                                                            ItemType::Honey => item_icons.honey.clone(),
                                                            ItemType::Steel => item_icons.steel.clone(),
                                                            ItemType::Chip => item_icons.chip.clone(),
                                                            ItemType::Phone => item_icons.phone.clone(),
                                                            ItemType::Log => item_icons.log.clone(),
                                                            ItemType::Lantern => item_icons.lantern.clone(),
                                                            ItemType::Axe => item_icons.axe.clone(),
                                                        },
                                                        ..default()
                                                    },
                                                    ..default()
                                                });

                                                commands
                                                    .spawn(TextBundle {
                                                        style: Style {
                                                            margin: UiRect::all(Val::Percent(5.0)),
                                                            ..default()
                                                        },
                                                        text: Text::from_section(
                                                            format!("x{}", target_building.yields[i].1),
                                                            TextStyle {
                                                                font: asset_server.load("font.otf"),
                                                                font_size: inventory_width / 45.0,
                                                                color: Color::WHITE,
                                                            },
                                                        ),
                                                        ..default()
                                                    })
                                                    .insert(YieldCountText { position: i })
                                                    .insert(Name::new("Yield count text"));
                                            });
                                    }

                                    commands
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(40.0), Val::Percent(100.0)),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::rgb(0.22, 0.25, 0.48).into(),
                                            ..default()
                                        })
                                        .insert(CollectButton)
                                        .insert(Name::new("Collect button"))
                                        .with_children(|commands| {
                                            commands.spawn(TextBundle {
                                                style: Style { ..default() },
                                                text: Text::from_section(
                                                    "COLLECT",
                                                    TextStyle {
                                                        font: asset_server.load("font.otf"),
                                                        font_size: inventory_width / 32.0,
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                ..default()
                                            });
                                        });
                                });
                        });

                    // Right half
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(45.0), Val::Percent(90.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: Color::rgb(0.17, 0.19, 0.36).into(),
                            ..default()
                        })
                        .insert(Name::new("Right side container"))
                        .with_children(|commands| {
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    commands.spawn(TextBundle {
                                        text: Text::from_section(
                                            "Next Level Requirements",
                                            TextStyle {
                                                font: asset_server.load("font.otf"),
                                                font_size: inventory_width / 36.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        ..default()
                                    });
                                });

                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceAround,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    for i in 0..=2 {
                                        commands
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(Val::Percent(30.0), Val::Percent(80.0)),
                                                    flex_direction: FlexDirection::Column,
                                                    justify_content: JustifyContent::SpaceEvenly,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .with_children(|commands| {
                                                // Upgrade material image
                                                commands
                                                    .spawn(ImageBundle {
                                                        style: Style {
                                                            size: Size::new(
                                                                Val::Px(inventory_width / 10.0),
                                                                Val::Px(inventory_width / 10.0),
                                                            ),
                                                            ..default()
                                                        },
                                                        image: UiImage {
                                                            texture: {
                                                                let upgrade_materials = upgrade_data.map
                                                                    [&target_building.building_type]
                                                                    [&target_building.level]
                                                                    .upgrade_materials;

                                                                match upgrade_materials[i].0 {
                                                                    ItemType::BronzeCoin => {
                                                                        item_icons.bronze_coin.clone()
                                                                    }
                                                                    ItemType::SilverCoin => {
                                                                        item_icons.silver_coin.clone()
                                                                    }
                                                                    ItemType::GoldCoin => item_icons.gold_coin.clone(),
                                                                    ItemType::Taffy => item_icons.taffy.clone(),
                                                                    ItemType::Nougat => item_icons.nougat.clone(),
                                                                    ItemType::Marshmallow => {
                                                                        item_icons.marshmallow.clone()
                                                                    }
                                                                    ItemType::Coffee => item_icons.coffee.clone(),
                                                                    ItemType::Cocoa => item_icons.cocoa.clone(),
                                                                    ItemType::Milkshake => item_icons.milkshake.clone(),
                                                                    ItemType::Apple => item_icons.apple.clone(),
                                                                    ItemType::Branch => item_icons.branch.clone(),
                                                                    ItemType::Honey => item_icons.honey.clone(),
                                                                    ItemType::Steel => item_icons.steel.clone(),
                                                                    ItemType::Chip => item_icons.chip.clone(),
                                                                    ItemType::Phone => item_icons.phone.clone(),
                                                                    ItemType::Log => item_icons.log.clone(),
                                                                    ItemType::Lantern => item_icons.lantern.clone(),
                                                                    ItemType::Axe => item_icons.axe.clone(),
                                                                }
                                                            },
                                                            ..default()
                                                        },
                                                        ..default()
                                                    })
                                                    .insert(UpgradeMaterialImage { position: i });

                                                let mut colour = Color::GREEN;

                                                let items_required: &(ItemType, u32) = &upgrade_data.map
                                                    [&target_building.building_type][&target_building.level]
                                                    .upgrade_materials[i];

                                                let quantity_acquired = inventory
                                                    .items
                                                    .iter()
                                                    .find(|item| item.item_type == items_required.0)
                                                    .unwrap()
                                                    .quantity;

                                                let quantity_required = items_required.1;

                                                if quantity_acquired < quantity_required {
                                                    colour = Color::RED;
                                                }

                                                let quantity_text = format!("{quantity_acquired}/{quantity_required}");

                                                // Upgrade material text
                                                commands
                                                    .spawn(TextBundle {
                                                        style: Style { ..default() },
                                                        text: Text::from_section(
                                                            quantity_text,
                                                            TextStyle {
                                                                font: asset_server.load("font.otf"),
                                                                font_size: inventory_width / 36.0,
                                                                color: colour,
                                                            },
                                                        ),
                                                        ..default()
                                                    })
                                                    .insert(UpgradeMaterialText { position: i });
                                            });
                                    }
                                });

                            let mut cost_colour = Color::GREEN;

                            let cost =
                                upgrade_data.map[&target_building.building_type][&target_building.level].upgrade_cost;
                            let balance = inventory.balance;

                            if balance < cost {
                                cost_colour = Color::RED;
                            }

                            let cost_text = format!("${balance} / ${cost}");

                            // Balance text
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    commands
                                        .spawn(TextBundle {
                                            text: Text::from_section(
                                                cost_text,
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    font_size: inventory_width / 36.0,
                                                    color: cost_colour,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(BalanceText);
                                });

                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    commands
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(30.0), Val::Percent(50.0)),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::rgb(0.22, 0.25, 0.48).into(),
                                            ..default()
                                        })
                                        .insert(UpgradeButton)
                                        .insert(Name::new("Upgrade button"))
                                        .with_children(|commands| {
                                            commands.spawn(TextBundle::from_section(
                                                "Upgrade",
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    font_size: inventory_width / 30.0,
                                                    color: Color::WHITE,
                                                },
                                            ));
                                        });
                                });
                        });
                });
        });
}

fn undraw_ui(
    mut commands: Commands,
    ui_root: Query<Entity, With<RootUINode>>,
    mut selected_building: ResMut<SelectedBuilding>,
) {
    selected_building.building = None;

    for entity in ui_root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
