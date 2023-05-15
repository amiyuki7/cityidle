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

fn draw_ui(
    mut commands: Commands,
    inventory: Res<Inventory>,
    asset_server: Res<AssetServer>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    item_icons: Res<ItemIcons>,
    mut buildings: Query<(Entity, &mut Building)>,
    mut upgrade_target_events: EventReader<UpgradeTarget>,
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
                                    background_color: Color::RED.into(),
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

                                    commands.spawn(NodeBundle { ..default() });
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
