use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_mod_picking::prelude::*;
use std::f32::consts::PI;

use crate::*;

const TILE_SIZE: f32 = 10.0;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SetupState>()
            .register_type::<SetupState>()
            .add_plugin(StateInspectorPlugin::<SetupState>::default())
            .register_type::<Tile>()
            .register_type::<PickSelection>()
            .add_startup_system(spawn_grid)
            .add_system(setup_buildings.run_if(state_exists_and_equals(SetupState::SpawnTileDone)));
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default, Reflect)]
pub enum SetupState {
    #[default]
    Idle,
    SpawnTileDone,
    SpawnBuildingDone,
}

#[derive(Component, Reflect, Debug)]
pub struct Tile {
    // pub building: Option<Building>,
    pub x: f32,
    pub z: f32,
}

impl Tile {
    fn new(x: f32, z: f32) -> Self {
        // Self { building: None, x, z }
        Self { x, z }
    }
}

#[derive(Reflect, FromReflect, Debug, PartialEq, Clone, Copy)]
pub enum BuildingType {
    CityCentre,
    Market,
    Construct,
    CandyShop,
    CoffeeShop,
    Tree,
    Factory,
}

impl BuildingType {
    pub fn get_transform(&self) -> Transform {
        use BuildingType::*;

        match self {
            CityCentre => {
                Transform::from_scale(Vec3::new(0.5, 0.8, 0.8)).with_rotation(Quat::from_rotation_y(PI / 2.0))
            }
            Market => Transform::from_scale(Vec3::new(1.5, 0.7, 1.2)).with_rotation(Quat::from_rotation_y(PI)),
            Construct => {
                Transform::from_scale(Vec3::new(10.0, 7.0, 20.0)).with_rotation(Quat::from_rotation_y(PI / 2.0))
            }
            CandyShop => Transform::from_scale(Vec3::new(1.2, 1.0, 1.2)),
            CoffeeShop => Transform::from_scale(Vec3::new(1.25, 1.0, 1.0)),
            Tree => Transform::from_scale(Vec3::new(3.0, 3.0, 3.0)),
            Factory => Transform::from_scale(Vec3::new(0.23, 0.33, 0.23)).with_rotation(Quat::from_rotation_y(PI)),
        }
    }
}

#[derive(Component, Reflect, FromReflect, Debug, PartialEq)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: u8,
}

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut next_setup_state: ResMut<NextState<SetupState>>,
) {
    let texture = asset_server.load("tile_texture.png");

    let default_highlight = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        // unlit: true,
        ..default()
    });

    let hover_highlight = materials.add(StandardMaterial {
        // base_color: Color::rgba(0.78, 0.0, 0.43, 0.5),
        base_color: Color::rgba(0.43, 0.28, 0.78, 0.5),
        // base_color_texture: Some(texture),
        // unlit: true,
        ..default()
    });

    let tile_plane = meshes.add(shape::Plane::from_size(TILE_SIZE).into());

    for i in 0..=10u8 {
        for j in 0..=10u8 {
            spawn_tile(
                &mut commands,
                tile_plane.clone(),
                default_highlight.clone(),
                hover_highlight.clone(),
                i,
                j,
            );
        }
    }
    debug!("Finished spawning tiles");
    next_setup_state.set(SetupState::SpawnTileDone);
}

#[derive(Component)]
struct ConstructPreviewSphere;

pub fn spawn_tile(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    default_material: Handle<StandardMaterial>,
    hover_material: Handle<StandardMaterial>,
    x_offset: u8,
    z_offset: u8,
) {
    let x_pos = TILE_SIZE * x_offset as f32;
    let z_pos = TILE_SIZE * z_offset as f32;

    commands
        .spawn((
            PbrBundle {
                mesh,
                material: default_material.clone(),
                transform: Transform::from_xyz(x_pos, 0.0, z_pos),
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            // TODO: Transparent version of the building rather than a sphere
            OnPointer::<Over>::run_callback(
                |In(event): In<ListenedEvent<Over>>,
                 construct_state: Res<State<ConstructPhase>>,
                 mut callback_commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<StandardMaterial>>| {
                    if construct_state.0 == ConstructPhase::Preview {
                        // Spawn a transparent sphere indicator
                        let sphere = callback_commands
                            .spawn(PbrBundle {
                                mesh: meshes.add(
                                    Mesh::try_from(shape::Icosphere {
                                        radius: 2.5,
                                        subdivisions: 3,
                                    })
                                    .unwrap(),
                                ),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::rgba(0.0, 1.0, 0.0, 0.25),
                                    alpha_mode: AlphaMode::Blend,
                                    unlit: true,
                                    ..default()
                                }),
                                transform: Transform::from_xyz(0.0, 2.5, 0.0),
                                ..default()
                            })
                            .insert(ConstructPreviewSphere)
                            .id();

                        callback_commands.entity(event.target).add_child(sphere);
                    }
                    Bubble::Up
                },
            ),
            // Find and delete transparent preview sphere
            OnPointer::<Out>::run_callback(
                |In(event): In<ListenedEvent<Out>>,
                 construct_state: Res<State<ConstructPhase>>,
                 mut callback_commands: Commands,
                 preview_spheres: Query<(Entity, &Parent), With<ConstructPreviewSphere>>| {
                    if construct_state.0 == ConstructPhase::Preview {
                        for (sphere_entity, parent) in preview_spheres.iter() {
                            debug!("Sphere: {:?}", sphere_entity);
                            if parent.get() == event.target {
                                callback_commands.entity(sphere_entity).despawn_recursive();
                                break;
                            }
                        }
                    }
                    Bubble::Up
                },
            ),
            OnPointer::<Click>::run_callback(
                // This is a big closure lol
                |In(event): In<ListenedEvent<Click>>,
                 buildings: Query<(&Parent, &Building), With<Building>>,
                 mut next_ui_state: ResMut<NextState<UiState>>,
                 camera_state: Res<State<CameraState>>,
                 mut previous_camera_state: ResMut<PreviousCameraState>,
                 mut send_change_camera_state_event: EventWriter<ChangeCameraStateEvent>,
                 construct_state: Res<State<ConstructPhase>>,
                 mut next_construct_state: ResMut<NextState<ConstructPhase>>,
                 mut callback_commands: Commands,
                 preview_spheres: Query<(&Parent, Entity), With<ConstructPreviewSphere>>,
                 mut building_stash: ResMut<BuildingStash>,
                 models: Res<Models>| {
                    if construct_state.0 == ConstructPhase::Normal {
                        for (parent, building) in buildings.iter() {
                            if parent.get() == event.target {
                                // We have found the targetted building
                                previous_camera_state.0 = Some(camera_state.0.clone());
                                send_change_camera_state_event.send(ChangeCameraStateEvent(CameraState::Frozen));

                                match building.building_type {
                                    BuildingType::CityCentre => {
                                        //
                                        next_ui_state.set(UiState::CityCentreInfo)
                                    }
                                    BuildingType::Market => {
                                        //
                                        next_ui_state.set(UiState::Market)
                                    }
                                    BuildingType::Construct => {
                                        //
                                        next_ui_state.set(UiState::Construct)
                                    }
                                    _ => {
                                        next_ui_state.set(UiState::BuildingInfo);
                                    }
                                }

                                break;
                            }
                        }
                    } else if construct_state.0 == ConstructPhase::Preview {
                        // Check if the sphere is on a tile without a building
                        let mut can_spawn_here = true;
                        if let Some((sphere_parent, sphere_entity)) = preview_spheres.iter().next() {
                            for (building_parent, _) in buildings.iter() {
                                if sphere_parent.get() == event.target && building_parent.get() == event.target {
                                    can_spawn_here = false;
                                }
                            }

                            if can_spawn_here {
                                if let Some(building_type) = building_stash.0 {
                                    callback_commands.entity(sphere_entity).despawn_recursive();
                                    // TODO refactor this into a generic spawn building fn
                                    let building = callback_commands
                                        .spawn(SceneBundle {
                                            scene: match building_type {
                                                BuildingType::CandyShop => models.candy_shop_scene.clone(),
                                                BuildingType::CoffeeShop => models.coffee_shop_scene.clone(),
                                                BuildingType::Tree => models.tree_scene.clone(),
                                                BuildingType::Factory => models.factory_scene.clone(),
                                                // This wildcard case will never happen
                                                _ => models.city_centre_scene.clone(),
                                            },
                                            transform: building_type.get_transform(),
                                            ..default()
                                        })
                                        .insert(Building {
                                            building_type,
                                            level: 1,
                                        })
                                        .id();

                                    callback_commands.entity(event.target).add_child(building);
                                }

                                building_stash.0 = None;
                                next_construct_state.set(ConstructPhase::Normal);
                                send_change_camera_state_event.send(ChangeCameraStateEvent(CameraState::CursorLocked));
                            }
                        }
                    }

                    trace!("{:?}", event.target);
                    Bubble::Up
                },
            ),
        ))
        .insert(Highlight {
            hovered: Some(HighlightKind::Fixed(hover_material.clone())),
            pressed: Some(HighlightKind::Fixed(hover_material)),
            selected: Some(HighlightKind::Fixed(default_material)),
        })
        .insert(Tile::new(x_pos, z_pos))
        .insert(Name::new(format!("Tile ({x_pos},{z_pos})")));
}

fn setup_buildings(
    mut commands: Commands,
    mut tiles: Query<(Entity, &mut Tile)>,
    models: Res<Models>,
    mut next_setup_state: ResMut<NextState<SetupState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (tile_entity, mut tile) in tiles.iter_mut() {
        if tile.x == 70.0 && tile.z == 60.0 {
            let building = commands
                .spawn(SceneBundle {
                    scene: models.city_centre_scene.clone(),
                    transform: BuildingType::CityCentre.get_transform(),
                    ..default()
                })
                .insert(Building {
                    building_type: BuildingType::CityCentre,
                    level: 1,
                })
                .id();

            commands.entity(tile_entity).add_child(building);
        } else if tile.x == 70.0 && tile.z == 70.0 {
            let building = commands
                .spawn(SceneBundle {
                    scene: models.market_scene.clone(),
                    transform: BuildingType::Market.get_transform(),
                    ..default()
                })
                .insert(Building {
                    building_type: BuildingType::Market,
                    level: 1,
                })
                .id();

            commands.entity(tile_entity).add_child(building);
        } else if tile.x == 80.0 && tile.z == 40.0 {
            let building = commands
                .spawn(SceneBundle {
                    scene: models.construction_scene.clone(),
                    transform: BuildingType::Construct.get_transform(),
                    ..default()
                })
                .insert(Building {
                    building_type: BuildingType::Construct,
                    level: 1,
                })
                .id();

            commands.entity(tile_entity).add_child(building);
        } else if tile.x == 80.0 && tile.z == 50.0 {
            let building = commands
                .spawn(SceneBundle {
                    scene: models.factory_scene.clone(),
                    transform: BuildingType::Factory.get_transform(),
                    ..default()
                })
                .insert(Building {
                    building_type: BuildingType::Factory,
                    level: 1,
                })
                .id();

            commands.entity(tile_entity).add_child(building);
        }
    }

    debug!("Finished setting up buildings");
    next_setup_state.set(SetupState::SpawnBuildingDone);
}
