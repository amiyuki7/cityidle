use bevy::prelude::*;

pub mod camera;
pub mod grid;

pub use camera::*;
pub use grid::*;

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
        }
    }
}

#[derive(Resource, Default)]
pub struct Models {
    pub bank_scene: Handle<Scene>,
}

pub fn load_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Models {
        bank_scene: asset_server.load("bank_low_poly.glb#Scene0"),
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
