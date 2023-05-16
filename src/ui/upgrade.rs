use crate::*;
use bevy::window::PrimaryWindow;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedBuilding>()
            .add_system(draw_ui.in_schedule(OnEnter(UiState::Upgrade)))
            .add_system(undraw_ui.in_schedule(OnExit(UiState::Upgrade)))
            .add_systems((exit_uistate, collect_button_interaction).in_set(OnUpdate(UiState::Upgrade)));
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
struct CollectButton;

// Marker
#[derive(Component)]
struct YieldCountText;

#[allow(clippy::complexity)]
fn collect_button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CollectButton>)>,
    mut buildings: Query<(Entity, &mut Building)>,
    selected_building: Res<SelectedBuilding>,
    mut inventory: ResMut<Inventory>,
    mut yield_count_texts: Query<&mut Text, With<YieldCountText>>,
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

                for mut text in yield_count_texts.iter_mut() {
                    text.sections[0].value = "x0".to_string();
                }
            }
            Interaction::Hovered => *background_colour = Color::rgb(0.34, 0.37, 0.60).into(),
            _ => *background_colour = Color::rgb(0.22, 0.25, 0.48).into(),
        }
    }
}

#[derive(Resource, Default)]
struct SelectedBuilding {
    building: Option<Entity>,
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
    let physical_screen_height = primary_window.single().resolution.physical_height() as f32;

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
    let mut target_building = target_building.unwrap();

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
                        size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::SpaceAround,
                        align_self: AlignSelf::Center,
                        margin: UiRect::left(Val::Percent(25.0)),
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
                                                        font_size: physical_screen_height / 72.0,
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
                                            commands.spawn(TextBundle {
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
                                                        font_size: physical_screen_height / 84.0,
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                ..default()
                                            });

                                            commands.spawn(TextBundle {
                                                style: Style {
                                                    margin: UiRect::all(Val::Percent(5.0)),
                                                    ..default()
                                                },
                                                text: Text::from_section(
                                                    format!("Speed {}s → {}s", level_stats.speed, {
                                                        if next_level_stats.is_none() {
                                                            "MAX".to_string()
                                                        } else {
                                                            next_level_stats.unwrap().speed.to_string()
                                                        }
                                                    }),
                                                    TextStyle {
                                                        font: asset_server.load("font.otf"),
                                                        font_size: physical_screen_height / 84.0,
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                ..default()
                                            });
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
                                                // let item_type = level_stats.yields.map(|(item_type, _)| item_type);
                                                // let y = level_stats.yields;

                                                commands.spawn(ImageBundle {
                                                    style: Style {
                                                        size: Size::new(Val::Percent(18.0), Val::Percent(100.0)),
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

                                                commands.spawn(TextBundle {
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
                                                            font_size: physical_screen_height / 84.0,
                                                            color: Color::WHITE,
                                                        },
                                                    ),
                                                    ..default()
                                                });
                                            });
                                    }
                                });
                            // Container for yield collection
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        align_items: AlignItems::Center,
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
                                                        size: Size::new(Val::Percent(75.0), Val::Percent(45.0)),
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
                                                                font_size: physical_screen_height / 90.0,
                                                                color: Color::WHITE,
                                                            },
                                                        ),
                                                        ..default()
                                                    })
                                                    .insert(YieldCountText)
                                                    .insert(Name::new("Yield count text"));
                                            });
                                    }

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
                                        .insert(CollectButton)
                                        .insert(Name::new("Collect button"))
                                        .with_children(|commands| {
                                            commands.spawn(TextBundle {
                                                style: Style { ..default() },
                                                text: Text::from_section(
                                                    "COLLECT",
                                                    TextStyle {
                                                        font: asset_server.load("font.otf"),
                                                        font_size: physical_screen_height / 64.0,
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
                                    background_color: Color::PURPLE.into(),
                                    ..default()
                                })
                                .with_children(|commands| {
                                    commands.spawn(TextBundle {
                                        // style: Style { ..default() },
                                        text: Text::from_section(
                                            "Next Level Requirements",
                                            TextStyle {
                                                font: asset_server.load("font.otf"),
                                                font_size: physical_screen_height / 72.0,
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
                                    background_color: Color::PINK.into(),
                                    ..default()
                                })
                                .with_children(|commands| {
                                    for i in 0..=2 {
                                        commands
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(Val::Percent(30.0), Val::Percent(80.0)),
                                                    flex_direction: FlexDirection::Column,
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                background_color: Color::GREEN.into(),
                                                ..default()
                                            })
                                            .with_children(|commands| {
                                                commands.spawn(ImageBundle {
                                                    style: Style {
                                                        size: Size::new(Val::Percent(80.0), Val::Percent(45.0)),
                                                        ..default()
                                                    },
                                                    image: UiImage {
                                                        texture: {
                                                            if next_level_stats.is_none() {
                                                                item_icons.empty.clone()
                                                            } else {
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
                                                            }
                                                        },
                                                        ..default()
                                                    },
                                                    ..default()
                                                });
                                            });
                                    }
                                });
                        });
                });
        });
}

fn undraw_ui(mut commands: Commands, ui_root: Query<Entity, With<RootUINode>>) {
    for entity in ui_root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
