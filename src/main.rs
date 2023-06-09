use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::WindowMode,
};
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin};
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
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::TRACE,
                    filter: "wgpu=warn,bevy_ecs=info,winit=info,naga=info,bevy_app=info,bevy_winit=info,bevy_render=info,bevy_core=info,gilrs=info,bevy_picking_core=warn"
                        .to_string(),
                }),
        )
        .add_plugins(
            DefaultPickingPlugins.build().disable::<DebugPickingPlugin>(),
        )
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GridPlugin)
        .add_state::<CameraState>()
        .register_type::<CameraState>()
        .add_state::<UiState>()
        // .add_plugin(StateInspectorPlugin::<UiState>::default())
        // .add_plugin(StateInspectorPlugin::<CameraState>::default())
        .add_plugin(GameCameraPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(MarketPlugin)
        .add_plugin(ConstructPlugin)
        .add_plugin(UpgradePlugin)
        .add_plugin(TimerPlugin)
        .add_plugin(ResourceInspectorPlugin::<Timers>::default())
        .add_plugin(AutoSavePlugin)
        .init_resource::<Keybinds>()
        .init_resource::<UpgradeData>()
        .insert_resource(Msaa::default())
        .insert_resource(SelectionSettings {
            click_nothing_deselect_all: true,
            use_multiselect_default_inputs: false,
        })
        .insert_resource(ClearColor(Color::hex("#87CDED").unwrap()))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        .add_startup_systems((load_models, load_item_icons).in_base_set(StartupSet::PreStartup))
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
