use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_kira_audio::AudioPlugin;

use crate::game::menu::MenuPlugin;
use crate::game::state::GameState;
use crate::game::systems::gamepad_connections;

use self::{
    resources::Options,
    snake::SnakePlugin,
    systems::{setup, setup_icon},
};

// Plugins
mod menu;
mod snake;
// Game
mod events;
mod resources;
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
        .add_state::<GameState>()
        .insert_resource(Options::default())
        .add_systems(Startup, (setup, setup_icon))
        .add_systems(Update, gamepad_connections)
        .add_plugins((AudioPlugin, MenuPlugin, SnakePlugin));
    }
}
