use bevy_mod_picking::prelude::*;

use crate::*;

const TILE_SIZE: f32 = 1000.0;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tile>()
            .register_type::<PickSelection>()
            .add_startup_system(spawn_grid);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Tile {
    occupied: bool,
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
                    0.0 + TILE_SIZE * x_offset as f32,
                    0.0,
                    0.0 + TILE_SIZE * z_offset as f32,
                ),
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            OnPointer::<Click>::target_component_mut::<Transform>(|click, transform| {
                if click.button == PointerButton::Primary {
                    let translation = transform.translation;
                    warn!("{translation:?}");
                }
            }),
        ))
        .insert(Highlight {
            hovered: Some(HighlightKind::Fixed(hover_material.clone())),
            pressed: Some(HighlightKind::Fixed(hover_material.clone())),
            selected: Some(HighlightKind::Fixed(hover_material)),
        })
        .insert(Tile { occupied: false })
        .insert(Name::new("Tile"));
}
