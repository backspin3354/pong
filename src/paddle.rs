use glam::{Vec2, Vec3};

use crate::gfx;

#[derive(Default)]
pub struct Paddle {
    pub pos: Vec2,

    pub moving_up: bool,
    pub moving_down: bool,
}

impl Paddle {
    pub const WIDTH: f32 = 0.02;
    pub const HEIGHT: f32 = 0.2;

    pub const SPEED: f32 = 0.03;

    pub fn rect(&self) -> gfx::Rect {
        gfx::Rect {
            position: Vec3::new(self.pos.x, self.pos.y, 0.0),
            size: Vec2 {
                x: Self::WIDTH,
                y: Self::HEIGHT,
            },

            color: gfx::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        }
    }

    pub fn update(&mut self) {
        if self.moving_up {
            self.pos.y += Self::SPEED;
        }

        if self.moving_down {
            self.pos.y -= Self::SPEED;
        }
    }
}
