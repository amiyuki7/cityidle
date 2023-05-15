use crate::*;
use bevy::window::PrimaryWindow;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(draw_ui.in_schedule(OnEnter(UiState::Upgrade)))
            .add_system(undraw_ui.in_schedule(OnExit(UiState::Upgrade)))
            .add_system(exit_uistate.in_set(OnUpdate(UiState::Upgrade)));
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

#[allow(clippy::complexity)]
fn draw_ui(
    mut commands: Commands,
    inventory: Res<Inventory>,
    asset_server: Res<AssetServer>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    item_icons: Res<ItemIcons>,
    mut buildings: Query<(Entity, &mut Building)>,
    mut upgrade_target_events: EventReader<UpgradeTarget>,
    upgrade_data: Res<UpgradeData>,
) {
    let physical_screen_height = primary_window.single().resolution.physical_height() as f32;

    let mut target_building = None;

    if let Some(target) = upgrade_target_events.iter().next() {
        // target.target_entity
        for (entity, building) in buildings.iter_mut() {
            if entity == target.target_entity {
                target_building = Some(building);
                break;
            }
        }
    }

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
                            commands.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                                    flex_direction: FlexDirection::Row,
                                    ..default()
                                },
                                background_color: Color::PINK.into(),
                                ..default()
                            });
                            // .with_children(|commands| commands.spawn(););
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
                        .insert(Name::new("Right side container"));
                });
        });
}

fn undraw_ui(mut commands: Commands, ui_root: Query<Entity, With<RootUINode>>) {
    for entity in ui_root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
