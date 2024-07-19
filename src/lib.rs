use std::sync::Arc;

mod gfx;

mod ball;
mod paddle;

use ball::Ball;
use gfx::{Color, Rect, Renderer};
use glam::{Vec2, Vec3};
use paddle::Paddle;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

pub struct State {
    window: Option<Arc<Window>>,
    renderer: Option<gfx::Renderer>,

    ball: ball::Ball,

    left_paddle: paddle::Paddle,
    right_paddle: paddle::Paddle,
}

impl Default for State {
    fn default() -> Self {
        Self {
            window: None,
            renderer: None,

            left_paddle: paddle::Paddle {
                pos: Vec2::new(-0.8, 0.0),
                ..Default::default()
            },
            right_paddle: paddle::Paddle {
                pos: Vec2::new(0.8, 0.0),
                ..Default::default()
            },

            ball: ball::Ball::new(),
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
        let renderer = Renderer::new(window_arc.clone());

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
                    WindowEvent::KeyboardInput { event, .. } => {
                        if let PhysicalKey::Code(code) = event.physical_key {
                            match code {
                                KeyCode::KeyW => {
                                    self.left_paddle.moving_up = event.state.is_pressed();
                                }
                                KeyCode::KeyS => {
                                    self.left_paddle.moving_down = event.state.is_pressed();
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.left_paddle.update();
        self.right_paddle.update();
        self.ball.update();

        if self.ball.dir.x > 0.0 {
            self.right_paddle.moving_up =
                self.right_paddle.pos.y + Paddle::HEIGHT < self.ball.pos.y;
            self.right_paddle.moving_down =
                self.right_paddle.pos.y - Paddle::HEIGHT > self.ball.pos.y;
        } else {
            self.right_paddle.moving_up = false;
            self.right_paddle.moving_down = false;
        }

        {
            if self.ball.pos.y >= 1.0 - Ball::SIZE || self.ball.pos.y <= -1.0 + Ball::SIZE {
                if self.ball.pos.y.signum() == self.ball.dir.y.signum() {
                    self.ball.dir.y *= -1.0;
                }
            }

            fn collide(paddle: &Paddle, ball: &mut Ball) {
                if if paddle.pos.x < 0.0 {
                    ball.pos.x <= paddle.pos.x + Paddle::WIDTH + Ball::SIZE
                        && ball.pos.x >= paddle.pos.x - Paddle::WIDTH
                } else {
                    ball.pos.x >= paddle.pos.x - (Paddle::WIDTH + Ball::SIZE)
                        && ball.pos.x <= paddle.pos.x + Paddle::WIDTH
                } {
                    if ball.pos.y <= paddle.pos.y + Paddle::HEIGHT + Ball::SIZE
                        && ball.pos.y >= paddle.pos.y - Paddle::HEIGHT - Ball::SIZE
                    {
                        ball.dir = (ball.pos - paddle.pos).normalize();
                        ball.speed += 0.002;
                    }
                }
            }

            collide(&self.left_paddle, &mut self.ball);

            collide(&self.right_paddle, &mut self.ball);
        }

        if self.ball.pos.x.abs() > 1.0 + Ball::SIZE {
            self.ball.pos = Vec2::ZERO;
            self.ball.dir = Vec2::new(-self.ball.dir.x.signum(), 0.0);
            self.ball.speed = 0.01;
        }

        if let Some(renderer) = self.renderer.as_mut() {
            renderer.batch_rects(&[
                gfx::Rect {
                    position: Vec3::ZERO,
                    size: Vec2::new(0.005, 1.0),
                    color: gfx::Color {
                        r: 0.5,
                        g: 0.5,
                        b: 0.5,
                    },
                },
                self.left_paddle.rect(),
                self.right_paddle.rect(),
                self.ball.rect(),
            ]);

            renderer.draw();
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer = None;
    }
}
