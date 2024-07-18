use std::sync::Arc;

mod gfx;

use gfx::{Color, Rect, Renderer};
use glam::{Vec2, Vec3};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
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
        let size = PhysicalSize::new(256, 256);

        let attributes = WindowAttributes::default()
            .with_title("Pong")
            .with_min_inner_size(size)
            .with_inner_size(size);

        let window = event_loop.create_window(attributes).unwrap();

        let window_arc = Arc::new(window);
        let mut renderer = Renderer::new(window_arc.clone());

        renderer.batch_rects(&[
            Rect {
                position: Vec3::new(-0.8, 0.0, 0.0),
                size: Vec2::new(0.02, 0.1),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                },
            },
            Rect {
                position: Vec3::new(0.8, 0.0, 0.0),
                size: Vec2::new(0.02, 0.1),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                },
            },
            Rect {
                position: Vec3::new(0.0, 0.0, 0.0),
                size: Vec2::new(0.02, 0.02),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                },
            },
        ]);

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

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer = None;
    }
}
