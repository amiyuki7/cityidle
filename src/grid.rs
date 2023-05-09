use bevy_mod_picking::prelude::*;

use crate::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tile>()
            .register_type::<PickSelection>()
            .add_startup_system(spawn_tile);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Tile {
    occupied: bool,
}

pub fn spawn_tile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Grey
    let default_highlight = materials.add(Color::rgba(0.85, 0.85, 0.85, 0.5).into());
    // Magenta
    let hover_highlight = materials.add(Color::rgba(0.78, 0.0, 0.43, 0.5).into());

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(4.0).into()),
            material: default_highlight,
            transform: Transform::from_xyz(0.0, 0.001, 0.0),
            ..default()
        })
        .insert(PickableBundle::default())
        .insert(RaycastPickTarget::default())
        .insert(Highlight {
            hovered: Some(HighlightKind::Fixed(hover_highlight.clone())),
            pressed: Some(HighlightKind::Fixed(hover_highlight.clone())),
            selected: Some(HighlightKind::Fixed(hover_highlight)),
        })
        .insert(Tile { occupied: false })
        .insert(Name::new("Tile"));
}
