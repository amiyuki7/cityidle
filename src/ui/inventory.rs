use bevy::window::PrimaryWindow;

use crate::*;

use super::UiState;

#[derive(Resource)]
pub struct Inventory {
    // items: [Item; 30],
    items: [Item; 3],
    balance: u32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: [
                Item::new(ItemType::Taffy, 20),
                Item::new(ItemType::Nougat, 10),
                Item::new(ItemType::Marshmallow, 5),
            ],
            balance: 100,
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_system(toggle_inventory)
            .add_system(draw_inventory.in_schedule(OnEnter(UiState::Inventory)))
            .add_system(undraw_inventory.in_schedule(OnExit(UiState::Inventory)));
    }
}

pub fn toggle_inventory(
    keybinds: Res<Keybinds>,
    keys: Res<Input<KeyCode>>,
    camera_state: Res<State<CameraState>>,
    mut previous_camera_state: ResMut<PreviousCameraState>,
    mut send_change_camera_state_event: EventWriter<ChangeCameraStateEvent>,
    ui_state: Res<State<UiState>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
) {
    if !keys.just_pressed(keybinds.toggle_inventory) {
        return;
    }

    // if camera_state.0 == CameraState::Frozen {
    if ui_state.0 == UiState::Inventory {
        // TODO: Hide inventory UI
        next_ui_state.set(UiState::None);
        // Restore camera state
        send_change_camera_state_event.send(ChangeCameraStateEvent(
            previous_camera_state.0.clone().unwrap(),
        ));
    } else if ui_state.0 == UiState::None {
        // Save camera state
        previous_camera_state.0 = Some(camera_state.0.clone());
        // Freeze camera
        send_change_camera_state_event.send(ChangeCameraStateEvent(CameraState::Frozen));
        // TODO: Show inventory UI
        next_ui_state.set(UiState::Inventory);
    }
}

// Marker
#[derive(Component, Reflect)]
struct InventoryUIRoot;

// Marker
#[derive(Component, Reflect)]
struct InventoryItemButton;

fn draw_inventory(
    mut commands: Commands,
    inventory: Res<Inventory>,
    asset_server: Res<AssetServer>,
    buildings: Query<&Building>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    item_icons: Res<ItemIcons>,
) {
    let mut city_centre_level = 0u8;

    for building in buildings.iter() {
        if building.building_type == BuildingType::Bank {
            city_centre_level = building.level;
            break;
        }
    }

    let physical_screen_height = primary_window.single().resolution.physical_height() as f32;

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
            ..default()
        })
        .insert(Name::new("InventoryUIRoot"))
        .insert(InventoryUIRoot)
        .with_children(|commands| {
            // The inventory area
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
                    // Background: 0.13, 0.14, 0.26
                    // Box background: 0.17, 0.19, 0.36
                    // Selected: 0.55, 0.44, 0.95
                    ..default()
                })
                .insert(Name::new("Inventory Layout"))
                .with_children(|commands| {
                    // The left half
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(45.0), Val::Percent(90.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            // background_color: Color::RED.into(),
                            ..default()
                        })
                        .insert(Name::new("Left side container"))
                        .with_children(|commands| {
                            // Stats container
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    background_color: Color::rgba(0.17, 0.19, 0.36, 0.5).into(),
                                    ..default()
                                })
                                .insert(Name::new("Stats container"))
                                .with_children(|commands| {
                                    // Balance text
                                    commands
                                        .spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::all(Val::Percent(5.0)),
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                format!("Balance: ${}", inventory.balance),
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    // Font size 40 looked nice on my own screen height of 2880, which is a ratio of 1:72
                                                    font_size: physical_screen_height / 72.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(Name::new("Balance text"));
                                    // City centre text
                                    commands
                                        .spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::all(Val::Percent(5.0)),
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                format!("City Centre: Level {}", city_centre_level),
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    font_size: physical_screen_height / 72.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(Name::new("City centre text"));
                                });

                            // Inventory grid container
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                        flex_direction: FlexDirection::Row,
                                        // align_items: AlignItems::FlexEnd,
                                        justify_content: JustifyContent::SpaceAround,
                                        flex_wrap: FlexWrap::Wrap,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.17, 0.19, 0.36).into(),
                                    ..default()
                                })
                                .insert(Name::new("Inventory grid container"))
                                .with_children(|commands| {
                                    for i in 0..=29 {
                                        commands
                                            .spawn(ButtonBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0 / (5.0 / 0.9)),
                                                        Val::Percent(100.0 / (6.0 / 0.9)),
                                                    ),
                                                    align_content: AlignContent::Center,
                                                    ..default()
                                                },
                                                background_color: Color::rgb(0.22, 0.25, 0.48)
                                                    .into(),
                                                ..default()
                                            })
                                            .with_children(|commands| {
                                                // Item icon
                                                commands.spawn(ImageBundle {
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Percent(100.0),
                                                            Val::Percent(100.0),
                                                        ),
                                                        ..default()
                                                    },
                                                    image: UiImage {
                                                        texture: {
                                                            if i < inventory.items.len() {
                                                                match inventory.items[i].item_type {
                                                                    ItemType::Taffy => {
                                                                        item_icons.taffy.clone()
                                                                    }
                                                                    ItemType::Nougat => {
                                                                        item_icons.nougat.clone()
                                                                    }
                                                                    ItemType::Marshmallow => {
                                                                        item_icons
                                                                            .marshmallow
                                                                            .clone()
                                                                    }
                                                                }
                                                            } else {
                                                                item_icons.empty.clone()
                                                            }
                                                        },
                                                        flip_x: false,
                                                        flip_y: false,
                                                    },
                                                    transform: Transform::from_scale(Vec3::splat(
                                                        0.7,
                                                    )),
                                                    ..default()
                                                });
                                                // Quantity text
                                                commands
                                                    .spawn(TextBundle {
                                                        style: Style {
                                                            size: Size::new(
                                                                Val::Percent(25.0),
                                                                Val::Percent(30.0),
                                                            ),
                                                            position_type: PositionType::Absolute,
                                                            position: UiRect::new(
                                                                Val::Percent(5.0),
                                                                Val::Percent(0.0),
                                                                Val::Percent(65.0),
                                                                Val::Percent(0.0),
                                                            ),
                                                            ..default()
                                                        },
                                                        text: Text::from_section(
                                                            {
                                                                if i < inventory.items.len() {
                                                                    inventory.items[i]
                                                                        .qty
                                                                        .to_string()
                                                                } else {
                                                                    "-".to_string()
                                                                }
                                                            },
                                                            TextStyle {
                                                                font: asset_server.load("font.otf"),
                                                                font_size: physical_screen_height
                                                                    / 108.0,
                                                                color: Color::WHITE,
                                                            },
                                                        ),
                                                        ..default()
                                                    })
                                                    .insert(Name::new("Quantity text"));
                                            })
                                            .insert(InventoryItemButton);
                                    }
                                });
                        });

                    // Item stats container
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(45.0), Val::Percent(90.0)),
                                ..default()
                            },
                            background_color: Color::rgb(0.17, 0.19, 0.36).into(),
                            ..default()
                        })
                        .insert(Name::new("Item stats container"));
                });
        });
}

fn undraw_inventory(mut commands: Commands, ui_root: Query<Entity, With<InventoryUIRoot>>) {
    for entity in ui_root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
