use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            a: PI / 3.0,
            fov: PI / 3.0,
        }
    }
}
