use bevy::prelude::*;

use self::{
    resources::Options,
    snake::SnakePlugin,
    systems::{setup, setup_icon},
};

mod events;
mod resources;
mod snake;
mod state;
mod systems;

pub const HEIGHT: f32 = 640.0;
pub const WIDTH: f32 = 640.0;

// Game Plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake !".to_string(),
                resolution: (WIDTH, HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Options::default())
        .add_systems(Startup, (setup, setup_icon))
        .add_plugins(SnakePlugin);
    }
}
