use bevy_mod_picking::prelude::*;

use crate::*;

const TILE_SIZE: f32 = 10.0;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tile>()
            .register_type::<PickSelection>()
            .add_startup_system(spawn_grid);
    }
}

#[derive(Component, Reflect, Debug, Default)]
pub struct Tile {
    occupied: bool,
}

impl Tile {
    fn new() -> Self {
        Self::default()
    }
}

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
}

pub fn spawn_tile(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    default_material: Handle<StandardMaterial>,
    hover_material: Handle<StandardMaterial>,
    x_offset: u8,
    z_offset: u8,
) {
    commands
        .spawn((
            PbrBundle {
                mesh,
                material: default_material,
                transform: Transform::from_xyz(
                    TILE_SIZE * x_offset as f32,
                    0.0,
                    TILE_SIZE * z_offset as f32,
                ),
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
        .insert(Tile::new())
        .insert(Name::new("Tile"));
}
