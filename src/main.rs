use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::WindowMode,
};
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
use bevy_mod_picking::{prelude::*, selection::SelectionSettings};
use cityidle::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "City Idle".into(),
                        mode: WindowMode::BorderlessFullscreen,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::TRACE,
                    filter: "wgpu=warn,bevy_ecs=info,winit=info,naga=info,bevy_app=info,bevy_winit=info,bevy_render=info,bevy_core=info,gilrs=info"
                        .to_string(),
                }),
        )
        .add_plugins(
            DefaultPickingPlugins.build(),
            // .disable::<DebugPickingPlugin>(),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GridPlugin)
        .add_state::<CameraState>()
        .register_type::<CameraState>()
        .add_state::<UiState>()
        .add_plugin(StateInspectorPlugin::<CameraState>::default())
        .add_plugin(GameCameraPlugin)
        .add_plugin(InventoryPlugin)
        .init_resource::<Keybinds>()
        .insert_resource(SelectionSettings {
            click_nothing_deselect_all: true,
            use_multiselect_default_inputs: false,
        })
        .insert_resource(ClearColor(Color::hex("#87CDED").unwrap()))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        .add_startup_system(load_models.in_base_set(StartupSet::PreStartup))
        .add_startup_system(spawn_light)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                range: 1000.0,
                shadows_enabled: true,
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(50.0, 10.0, 50.0),
            ..default()
        })
        .insert(Name::new("Light"));
}
