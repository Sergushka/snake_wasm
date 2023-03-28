use bevy::prelude::*;

pub(crate) const ARENA_WIDTH: u32 = 10;
pub(crate) const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Component)]
pub struct Size {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl Size {
    pub(crate) fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
