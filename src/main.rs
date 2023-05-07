use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use cityidle::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920.0, 1080.0),
                title: "City Idle".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GridPlugin)
        .insert_resource(ClearColor(Color::hex("#87CDED").unwrap()))
        .add_startup_systems((spawn_camera, spawn_plane, spawn_light))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(RaycastPickCamera::default());
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
}

fn spawn_light(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        })
        .insert(Name::new("Light"));
}
