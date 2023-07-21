use std::ops::{Add, Sub};

use crate::game::state::GameState;
use bevy::prelude::*;

use self::{gamepad::*, keyboard::key_board_movement, mouse::cursor_movement};

mod gamepad;
mod keyboard;
mod mouse;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                //Gamepad
                gamepad_movement,
                //Key_board
                key_board_movement,
                //Mouse
                cursor_movement,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

pub fn move_forward(
    transform: &mut Transform,
    direction: Vec3,
    speed: f32,
    delta_time: f32,
) {
    if transform.translation.z >= 1.0 {
        transform.translation.z = 0.0;
    }
    let delta = direction * speed * delta_time;
    transform.translation += delta;
}

pub fn move_forward_player(
    transform: &mut Transform,
    mut camera: &mut Transform,
    direction: Vec3,
    speed: f32,
    delta_time: f32,
) {
    move_forward(transform,direction,speed,delta_time);
    camera.translation = transform.translation;
    camera.translation.z = 1.1;
}
