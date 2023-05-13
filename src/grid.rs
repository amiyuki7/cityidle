use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_mod_picking::prelude::*;

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
    pub building: Option<Building>,
    pub x: f32,
    pub z: f32,
}

impl Tile {
    fn new(x: f32, z: f32) -> Self {
        Self { building: None, x, z }
    }
}

#[derive(Reflect, FromReflect, Debug, PartialEq)]
pub enum BuildingType {
    Bank,
}

#[derive(Component, Reflect, FromReflect, Debug)]
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
        base_color_texture: Some(texture.clone()),
        unlit: true,
        ..default()
    });

    let hover_highlight = materials.add(StandardMaterial {
        base_color: Color::rgba(0.78, 0.0, 0.43, 0.8),
        base_color_texture: Some(texture),
        unlit: true,
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
                material: default_material,
                transform: Transform::from_xyz(x_pos, 0.0, z_pos),
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            OnPointer::<Click>::target_component_mut::<Tile>(|click, tile| {
                if click.button == PointerButton::Primary {
                    trace!("{tile:?}");
                }
            }),
        ))
        .insert(Highlight {
            hovered: Some(HighlightKind::Fixed(hover_material.clone())),
            pressed: Some(HighlightKind::Fixed(hover_material.clone())),
            selected: Some(HighlightKind::Fixed(hover_material)),
        })
        .insert(Tile::new(x_pos, z_pos))
        .insert(Name::new(format!("Tile ({x_pos},{z_pos})")));
}

fn setup_buildings(
    mut commands: Commands,
    mut tiles: Query<(Entity, &mut Tile)>,
    models: Res<Models>,
    mut next_setup_state: ResMut<NextState<SetupState>>,
) {
    for (tile_entity, mut tile) in tiles.iter_mut() {
        if tile.x == 70.0 && tile.z == 60.0 {
            let building = commands
                .spawn(SceneBundle {
                    scene: models.bank_scene.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..default()
                })
                .id();

            commands.entity(tile_entity).add_child(building);
            tile.building = Some(Building {
                building_type: BuildingType::Bank,
                level: 1,
            });
        }
    }

    debug!("Finished setting up buildings");
    next_setup_state.set(SetupState::SpawnBuildingDone);
}
