use bevy::prelude::*;

use crate::game::menu::components::Menu;
use crate::game::menu::systems::{despawn_screen, handle_action, load_menu};
use crate::game::state::GameState;

mod components;
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), load_menu)
            .add_systems(Update, (handle_action).run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_screen::<Menu>);
    }
}
