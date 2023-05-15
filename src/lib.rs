use bevy::prelude::*;

mod camera;
mod grid;
mod ui;

pub use camera::*;
pub use grid::*;
pub use ui::*;

use bevy::window::CursorGrabMode;

#[derive(Resource)]
pub struct Keybinds {
    pub move_forward: KeyCode,
    pub move_left: KeyCode,
    pub move_backward: KeyCode,
    pub move_right: KeyCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub toggle_mouse_lock: KeyCode,
    pub toggle_inventory: KeyCode,
    pub exit_uistate: KeyCode,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::W,
            move_left: KeyCode::A,
            move_backward: KeyCode::S,
            move_right: KeyCode::D,
            move_up: KeyCode::Space,
            move_down: KeyCode::LShift,
            toggle_mouse_lock: KeyCode::F,
            toggle_inventory: KeyCode::E,
            exit_uistate: KeyCode::Escape,
        }
    }
}

#[derive(Resource, Default)]
pub struct Models {
    pub city_centre_scene: Handle<Scene>,
    pub market_scene: Handle<Scene>,
    pub construction_scene: Handle<Scene>,
    pub candy_shop_scene: Handle<Scene>,
    pub coffee_shop_scene: Handle<Scene>,
    pub tree_scene: Handle<Scene>,
    pub factory_scene: Handle<Scene>,
    pub cabin_scene: Handle<Scene>,
}

pub fn load_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Models {
        city_centre_scene: asset_server.load("bank_low_poly.glb#Scene0"),
        market_scene: asset_server.load("market.glb#Scene0"),
        construction_scene: asset_server.load("crane.glb#Scene0"),
        candy_shop_scene: asset_server.load("candy_shop.glb#Scene0"),
        coffee_shop_scene: asset_server.load("coffee_shop.glb#Scene0"),
        tree_scene: asset_server.load("tree.glb#Scene0"),
        factory_scene: asset_server.load("factory.glb#Scene0"),
        cabin_scene: asset_server.load("cabin.glb#Scene0"),
    });
}

pub fn set_cursor_lock(window: &mut Window, lock: bool) {
    if lock {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    } else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

pub struct AutoSavePlugin;

impl Plugin for AutoSavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AutoSaver>().add_system(auto_save);
    }
}

// TODO: Hook this up to a serializer and save to file
#[derive(Resource)]
pub struct AutoSaver {
    save_timer: Timer,
}

impl Default for AutoSaver {
    fn default() -> Self {
        Self {
            save_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

pub fn auto_save(
    time: Res<Time>,
    mut autosaver: ResMut<AutoSaver>,
    inventory: Res<Inventory>,
    buildings: Query<&Building>,
) {
    autosaver.save_timer.tick(time.delta());

    if autosaver.save_timer.just_finished() {
        debug!("Should be autosaving now");
    }
}
