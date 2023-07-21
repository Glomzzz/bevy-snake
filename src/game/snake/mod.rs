use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::game::snake::systems::*;
use crate::game::state::GameState::InGame;

use self::control::ControllerPlugin;

mod components;
pub mod control;
mod systems;

// snake plugin

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .insert_resource(FixedTime::new_from_secs(1.0 / 50.0))
            .add_systems(OnEnter(InGame), (play_bgm, on_start_spawn_snake))
            .add_systems(
                FixedUpdate,
                (
                    on_game_remove_tail,
                    on_game_move_head,
                    eat_apple,
                    apple_spawner,
                    snake_hit,
                    snake_random_direction,
                )
                    .chain()
                    .run_if(in_state(InGame)),
            )
            .add_systems(OnExit(InGame), on_exit_invisible_snake)
            .add_plugins(ControllerPlugin);
    }
}
