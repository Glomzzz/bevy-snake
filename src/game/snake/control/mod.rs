use std::ops::{Add, Sub};

use bevy::prelude::*;

use self::{gamepad::*, keyboard::key_board_movement, mouse::move_cursor};

mod gamepad;
mod keyboard;
mod mouse;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (gamepad_connections, gamepad_movement))
            .add_systems(Update, key_board_movement)
            .add_systems(Update, move_cursor);
    }
}

pub fn move_forward(
    transform: &mut Transform,
    mut camera: Mut<Transform>,
    direction: Vec3,
    speed: f32,
    delta_time: f32,
) {
    if transform.translation.z >= 1.0 {
        transform.translation.z = 0.0;
    }
    let delta = direction * speed * delta_time;
    camera.translation = transform.translation - delta;
    transform.translation += delta;
    camera.translation.z = 1.1;
}
