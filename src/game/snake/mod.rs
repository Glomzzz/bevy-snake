use self::{control::ControllerPlugin, systems::on_game_move_body};
use bevy::prelude::*;

mod components;
pub mod control;
mod systems;

// snake plugin

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ControllerPlugin)
            .add_systems(Startup, systems::on_start_spawn_snake)
            .add_systems(Update, on_game_move_body);
    }
}
