use std::ops::Add;

use bevy::prelude::*;

use crate::game::snake::components::{Body, Head};
use crate::game::state::GameState;

// use super::move_forward;

pub fn key_board_movement(
    mut heads: Query<&mut Body, With<Head>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::P) {
        game_state.set(GameState::Menu);
        return;
    }
    for mut body in heads.iter_mut().filter(|body| body.id == 1) {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        direction = if direction.length() > 0.0 {
            direction
                .normalize()
                .add(Vec3::new(0.0, 0.0, 0.00000000001))
        } else {
            return;
        };
        body.direction = direction;
    }
}
