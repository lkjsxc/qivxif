use std::error::Error;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct NativeApp {
    window: Option<Window>,
}

pub fn run_native() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = NativeApp::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}

impl ApplicationHandler for NativeApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attrs = Window::default_attributes().with_title("qivxif");
            self.window = Some(event_loop.create_window(attrs).expect("create window"));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        if matches!(event, WindowEvent::CloseRequested) {
            event_loop.exit();
        }
    }
}
