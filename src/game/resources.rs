use bevy::prelude::Resource;

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
