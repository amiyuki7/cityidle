use crate::*;

pub struct Item {
    pub name: String,
    pub qty: u32,
}

#[derive(Resource, Default)]
pub struct Inventory(Vec<Item>);

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_system(toggle_inventory);
    }
}

pub fn toggle_inventory(
    keybinds: Res<Keybinds>,
    keys: Res<Input<KeyCode>>,
    camera_state: Res<State<CameraState>>,
    mut previous_camera_state: ResMut<PreviousCameraState>,
    mut send_change_camera_state_event: EventWriter<ChangeCameraStateEvent>,
) {
    if !keys.just_pressed(keybinds.toggle_inventory) {
        return;
    }

    if camera_state.0 == CameraState::Frozen {
        // TODO: Hide inventory UI
        // Restore camera state
        send_change_camera_state_event.send(ChangeCameraStateEvent(
            previous_camera_state.0.clone().unwrap(),
        ));
    } else {
        // Save camera state
        previous_camera_state.0 = Some(camera_state.0.clone());
        // Freeze camera
        send_change_camera_state_event.send(ChangeCameraStateEvent(CameraState::Frozen));
        // TODO: Show inventory UI
    }
}
