use std::ops::{Deref, DerefMut};

/**
* Copyright (c) AWildDevAppears
*/
use raylib::{
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::{Color, Rectangle, Vector2},
};

pub struct GroupDrawHandle<'a, 'b> {
    d: &'a mut RaylibDrawHandle<'b>,
    origin: Vector2,
}

impl<'a, 'b> GroupDrawHandle<'a, 'b> {
    pub fn new(d: &'a mut RaylibDrawHandle<'b>, origin: Vector2) -> Self {
        Self { d, origin }
    }

    pub fn draw_circle(&mut self, local_center: Vector2, radius: f32, color: Color) {
        let world_center = self.origin + local_center;
        self.d.draw_circle_v(world_center, radius, color);
    }

    pub fn draw_rectangle(&mut self, local_pos: Vector2, size: Vector2, color: Color) {
        let world_pos = self.origin + local_pos;
        self.d.draw_rectangle_v(world_pos, size, color);
    }
}

impl<'a, 'b> Deref for GroupDrawHandle<'a, 'b> {
    type Target = RaylibDrawHandle<'b>;

    fn deref(&self) -> &Self::Target {
        self.d
    }
}

impl<'a, 'b> DerefMut for GroupDrawHandle<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.d
    }
}
