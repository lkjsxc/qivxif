use crate::native::{EDIT_BLOCK, NativeApp};
use qivxif_input::{CameraAction, ClientAction, command_for_action};
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

pub(crate) fn handle_key(app: &mut NativeApp, key: PhysicalKey, event_loop: &ActiveEventLoop) {
    match key {
        PhysicalKey::Code(KeyCode::Escape) => event_loop.exit(),
        PhysicalKey::Code(KeyCode::KeyW) => {
            app.camera.apply(CameraAction::Pan { dx: 0.0, dz: -8.0 });
        }
        PhysicalKey::Code(KeyCode::KeyS) => {
            app.camera.apply(CameraAction::Pan { dx: 0.0, dz: 8.0 });
        }
        PhysicalKey::Code(KeyCode::KeyA) => {
            app.camera.apply(CameraAction::Pan { dx: -8.0, dz: 0.0 });
        }
        PhysicalKey::Code(KeyCode::KeyD) => {
            app.camera.apply(CameraAction::Pan { dx: 8.0, dz: 0.0 });
        }
        PhysicalKey::Code(KeyCode::Equal) => {
            app.camera.apply(CameraAction::Zoom { delta: 2.0 });
        }
        PhysicalKey::Code(KeyCode::Minus) => {
            app.camera.apply(CameraAction::Zoom { delta: -2.0 });
        }
        PhysicalKey::Code(KeyCode::Space) => {
            app.client.send(command_for_action(
                ClientAction::PlaceSelected,
                app.target,
                EDIT_BLOCK,
            ));
        }
        PhysicalKey::Code(KeyCode::Backspace) => {
            app.client.send(command_for_action(
                ClientAction::RemoveTarget,
                app.target,
                EDIT_BLOCK,
            ));
        }
        _ => {}
    }
}
