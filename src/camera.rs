use crate::*;
use bevy::{
    ecs::event::{Events, ManualEventReader},
    input::mouse::MouseMotion,
    window::PrimaryWindow,
};
use bevy_mod_picking::prelude::*;

#[derive(Resource)]
pub struct InputSettings {
    pub sense: f32,
    pub speed: f32,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            sense: 30.0,
            speed: 4.0,
        }
    }
}

#[derive(Resource, Default)]
struct MouseMotionState {
    motion_event_reader: ManualEventReader<MouseMotion>,
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseMotionState>()
            .init_resource::<InputSettings>()
            .add_state::<CameraState>()
            .add_system(setup.on_startup())
            .add_systems(
                (keys_move_camera, mouse_move_camera).in_set(OnUpdate(CameraState::CursorLocked)),
            )
            .add_system(handle_camera_state);
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum CameraState {
    #[default]
    CursorLocked, // Cursor is locked - camera is free to move
    CursorUnlocked, // Cursor is unlocked - camera is stationary
}

/// Marker component
#[derive(Component)]
pub struct GameCamera;

fn setup(mut commands: Commands, mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(GameCamera);

    // Set initial state of cursor to locked
    toggle_cursor_lock(&mut primary_window.single_mut());
}

/// Fundamental movement - translates the camera around according to keyboard input
fn keys_move_camera(
    mut camera: Query<&mut Transform, With<GameCamera>>,
    keybinds: Res<Keybinds>,
    settings: Res<InputSettings>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut camera_transform in camera.iter_mut() {
        let mut velocity = Vec3::ZERO;
        /*
         * Local z: Unit vector which is the "backwards" direction
         * Local x: Unit vector which is the "rightwards" direction
         * Nullify y component - rotating the camera rotates the direciton of local components
         */
        let local_z = camera_transform.local_z();
        let backward = Vec3::new(local_z.x, 0.0, local_z.z);
        let local_x = camera_transform.local_x();
        let right = Vec3::new(local_x.x, 0.0, local_x.z);

        for key in keys.get_pressed() {
            if *key == keybinds.move_forward {
                velocity -= backward;
            }
            if *key == keybinds.move_backward {
                velocity += backward;
            }
            if *key == keybinds.move_right {
                velocity += right;
            }
            if *key == keybinds.move_left {
                velocity -= right;
            }
            if *key == keybinds.move_up {
                velocity += Vec3::Y;
            }
            if *key == keybinds.move_down {
                velocity -= Vec3::Y;
            }
        }

        // Normalize velocity vector so that going forward + right does not make you faster than going forward
        camera_transform.translation +=
            velocity.normalize_or_zero() * time.delta_seconds() * settings.speed;
    }
}

/// Rotates the camera in the direction of the mouse
fn mouse_move_camera(
    mut camera: Query<&mut Transform, With<GameCamera>>,
    mut mouse_motion_state: ResMut<MouseMotionState>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<InputSettings>,
    mouse_motion: Res<Events<MouseMotion>>,
) {
    for mut camera_transform in camera.iter_mut() {
        for event in mouse_motion_state.motion_event_reader.iter(&mouse_motion) {
            let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

            let window = primary_window.single();
            // Using minimum dimension ensures equal horizontal and vertical sense
            let window_scale = window.height().min(window.width());
            pitch -= (settings.sense / 500000.0 * event.delta.y * window_scale).to_radians();
            yaw -= (settings.sense / 500000.0 * event.delta.x * window_scale).to_radians();

            pitch = pitch.clamp(-1.54, 1.54);

            camera_transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

/// When the `toggle_mouse_lock` keybind is pressed (default M), switch between the two camera modes
#[allow(clippy::too_many_arguments)]
fn handle_camera_state(
    mut commands: Commands,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    camera: Query<Entity, With<GameCamera>>,
    raycast_camera: Query<Entity, (With<GameCamera>, With<RaycastPickCamera>)>,
    keys: Res<Input<KeyCode>>,
    keybinds: Res<Keybinds>,
    camera_state: Res<State<CameraState>>,
    mut next_camera_state: ResMut<NextState<CameraState>>,
) {
    if keys.just_pressed(keybinds.toggle_mouse_lock) {
        let camera = camera.get_single();
        let raycast_camera = raycast_camera.get_single();

        // Disable camera raycast when switching to non cursor mode... enable otherwise
        match raycast_camera {
            Ok(entity) => commands.entity(entity).remove::<RaycastPickCamera>(),
            Err(_) => commands
                .entity(camera.unwrap())
                .insert(RaycastPickCamera::default()),
        };

        if camera_state.0 == CameraState::CursorLocked {
            next_camera_state.set(CameraState::CursorUnlocked);
        } else {
            next_camera_state.set(CameraState::CursorLocked);
        }

        toggle_cursor_lock(&mut primary_window.single_mut());
    }
}
