use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use cityidle::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "City Idle".into(),
                mode: WindowMode::BorderlessFullscreen,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            DefaultPickingPlugins.build(),
            // .disable::<DebugPickingPlugin>(),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GridPlugin)
        .add_plugin(GameCameraPlugin)
        .init_resource::<Keybinds>()
        .insert_resource(ClearColor(Color::hex("#87CDED").unwrap()))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        .add_startup_systems((spawn_plane, spawn_light))
        .run();
}

fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 40.0,
                ..default()
            })),
            material: materials.add(Color::GRAY.into()),
            ..default()
        })
        .insert(Name::new("Plane"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Cube"));
}

fn spawn_light(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 8.0, 0.0),
            ..default()
        })
        .insert(Name::new("Light"));
}
