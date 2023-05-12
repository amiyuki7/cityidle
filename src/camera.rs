use crate::*;
use bevy::{
    ecs::event::{Events, ManualEventReader},
    input::mouse::MouseMotion,
    window::PrimaryWindow,
};
// use bevy_mod_picking::prelude::*;
use bevy_mod_picking::prelude::RaycastPickCamera;

#[derive(Resource)]
pub struct InputSettings {
    pub sense: f32,
    pub speed: f32,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            sense: 30.0,
            // speed: 1000.0,
            speed: 10.0,
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
            .init_resource::<PreviousCameraState>()
            .add_state::<CameraState>()
            .add_event::<ChangeCameraStateEvent>()
            .add_system(setup.on_startup())
            .add_systems(
                (keys_move_camera, mouse_move_camera).in_set(OnUpdate(CameraState::CursorLocked)),
            )
            .add_system(
                toggle_camera_state.run_if(not(state_exists_and_equals(CameraState::Frozen))),
            )
            .add_system(on_change_camera_state);
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default, Reflect)]
pub enum CameraState {
    #[default]
    CursorLocked, // Cursor is locked - camera is free to move
    CursorUnlocked, // Cursor is unlocked - camera is stationary
    Frozen,         // Everything is disabled - set to this state when using inventory, shop, etc
}

#[derive(Resource, Default)]
pub struct PreviousCameraState(pub Option<CameraState>);

/// Marker component
#[derive(Component)]
pub struct GameCamera;

fn setup(mut commands: Commands, mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let window = primary_window.single();
    debug!(
        "Physical height: {}, Logic height: {}",
        window.resolution.physical_height(),
        window.resolution.height()
    );

    debug!(
        "Physical width: {}, Logic width: {}",
        window.resolution.physical_width(),
        window.resolution.width()
    );

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(80.0, 10.0, 80.0)
                .looking_at(Vec3::new(70.0, 4.0, 70.0), Vec3::Y),
            ..default()
        })
        .insert(GameCamera);

    // Set initial state of cursor to locked
    // toggle_cursor_lock(&mut primary_window.single_mut());
    set_cursor_lock(&mut primary_window.single_mut(), true);
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

            if camera_transform.translation.y < 30.0 && *key == keybinds.move_up {
                velocity += Vec3::Y;
            }

            if camera_transform.translation.y > 5.0 && *key == keybinds.move_down {
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

pub struct ChangeCameraStateEvent(pub CameraState);

fn toggle_camera_state(
    keys: Res<Input<KeyCode>>,
    keybinds: Res<Keybinds>,
    mut send_change_camera_state_event: EventWriter<ChangeCameraStateEvent>,
    camera_state: Res<State<CameraState>>,
) {
    if keys.just_pressed(keybinds.toggle_mouse_lock) {
        // send_change_camera_state_event.send(ChangeCameraStateEvent(()))
        if camera_state.0 == CameraState::CursorLocked {
            send_change_camera_state_event
                .send(ChangeCameraStateEvent(CameraState::CursorUnlocked));
        } else {
            send_change_camera_state_event.send(ChangeCameraStateEvent(CameraState::CursorLocked));
        }
    }
}

/// When the `toggle_mouse_lock` keybind is pressed (default M), switch between the two camera modes
fn on_change_camera_state(
    mut commands: Commands,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    camera: Query<Entity, With<GameCamera>>,
    raycast_camera: Query<Entity, (With<GameCamera>, With<RaycastPickCamera>)>,
    mut next_camera_state: ResMut<NextState<CameraState>>,
    mut change_camera_state_events: EventReader<ChangeCameraStateEvent>,
) {
    let camera = camera.get_single();
    let raycast_camera = raycast_camera.get_single();
    let mut window = primary_window.single_mut();

    for event in change_camera_state_events.iter() {
        // Change to CursorLock
        if event.0 == CameraState::CursorLocked {
            // Disable camera raycast if it exists
            if let Ok(entity) = raycast_camera {
                commands.entity(entity).remove::<RaycastPickCamera>();
            }

            next_camera_state.set(CameraState::CursorLocked);
            set_cursor_lock(&mut window, true);
        }
        // Change the CursorUnlock
        else if event.0 == CameraState::CursorUnlocked {
            // Enable camera raycast if it doesn't already exist
            if raycast_camera.is_err() {
                commands
                    .entity(*camera.as_ref().unwrap())
                    .insert(RaycastPickCamera::default());
            }

            next_camera_state.set(CameraState::CursorUnlocked);
            set_cursor_lock(&mut window, false);
        }
        // Change to Frozen
        else if event.0 == CameraState::Frozen {
            // Disable camera raycast if it exists
            if let Ok(entity) = raycast_camera {
                commands.entity(entity).remove::<RaycastPickCamera>();
            }

            next_camera_state.set(CameraState::Frozen);
            set_cursor_lock(&mut window, false);
        }
    }
}
