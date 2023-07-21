use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;
use std::cell::{Cell, RefCell, UnsafeCell};
use std::pin::Pin;

#[derive(Resource)]
pub struct Options {
    pub start_length: u32,
    pub speed: f32,
    pub snake_radius: f32,
    pub apple_radius: f32,
}

impl Options {
    pub fn new(start_length: u32, speed: f32, snake_radius: f32, apple_radius: f32) -> Self {
        Options {
            start_length,
            speed,
            snake_radius,
            apple_radius,
        }
    }
    pub fn default() -> Self {
        Self::new(15, 500.0, 8.0, 8.0)
    }
}

#[derive(Resource)]
pub struct BackgroundHandle {
    pub(crate) handle: Handle<Image>,
}

#[derive(Resource)]
pub struct BackgroundSpriteSize {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);
