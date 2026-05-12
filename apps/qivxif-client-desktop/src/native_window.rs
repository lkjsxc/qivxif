use crate::{
    native::{EDIT_BLOCK, NativeApp, NativeMode},
    native_controls::handle_key,
};
use qivxif_input::{ClientAction, command_for_action};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

impl ApplicationHandler for NativeApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attrs = Window::default_attributes().with_title("qivxif native client");
            self.window = Some(event_loop.create_window(attrs).expect("create window"));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorMoved { position, .. } => {
                self.target = self.camera.target_from_pointer(
                    position.x as f32,
                    position.y as f32,
                    800.0,
                    600.0,
                );
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                let action = match button {
                    MouseButton::Left => Some(ClientAction::PlaceSelected),
                    MouseButton::Right => Some(ClientAction::RemoveTarget),
                    _ => None,
                };
                if let Some(action) = action {
                    self.client
                        .send(command_for_action(action, self.target, EDIT_BLOCK));
                }
            }
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                handle_key(self, event.physical_key, event_loop);
            }
            WindowEvent::RedrawRequested => self.render_snapshot(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.drain_events();
        if matches!(self.mode, NativeMode::E2e(_)) {
            self.update_e2e(event_loop);
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
