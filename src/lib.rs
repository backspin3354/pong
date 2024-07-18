use std::sync::Arc;

mod gfx;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub struct State {
    window: Option<Arc<Window>>,
    renderer: Option<gfx::Renderer>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            window: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = WindowAttributes::default().with_title("Pong");
        let window = event_loop.create_window(attributes).unwrap();

        let window_arc = Arc::new(window);
        let renderer = gfx::Renderer::new(window_arc.clone());

        self.window = Some(window_arc);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = self.window.as_ref() {
            if window_id == window.id() {
                match event {
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }
                    WindowEvent::Resized(new_size) => {
                        if let Some(renderer) = self.renderer.as_mut() {
                            renderer.resize(new_size.width, new_size.height);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(renderer) = self.renderer.as_mut() {
            renderer.draw();
        }
    }
}
