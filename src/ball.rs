use glam::{Vec2, Vec3};

use crate::gfx;

pub struct Ball {
    pub pos: Vec2,
    pub dir: Vec2,
    pub speed: f32,
}

impl Ball {
    pub const SIZE: f32 = 0.02;

    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            dir: Vec2::new(1.0, 0.0),
            speed: 0.01,
        }
    }

    pub fn update(&mut self) {
        self.pos += self.dir * self.speed;
    }

    pub fn rect(&self) -> gfx::Rect {
        gfx::Rect {
            position: Vec3::new(self.pos.x, self.pos.y, 0.0),
            size: Vec2::splat(Self::SIZE),
            color: gfx::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        }
    }
}
