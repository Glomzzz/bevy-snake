use std::ops::Add;

use crate::game::{
    resources::Options,
    snake::components::{Body, Snake},
};
use bevy::prelude::*;

// use super::move_forward;

pub fn key_board_movement(
    mut snake: Query<&Snake>,
    mut bodies: Query<(&mut Transform, &mut Body)>,
    keyboard_input: Res<Input<KeyCode>>,
    // time: Res<Time>,
    // options: Res<Options>,
) {
    // let speed = options.speed;
    // let delta_time = time.delta_seconds();
    for snake in snake.iter_mut() {
        if snake.id != "player" {
            return;
        }
        if snake.list.front().is_none() {
            continue;
        }
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
        let Some(entity) = snake.list.front() else{ return };
        let Ok(mut body) = bodies.get_component_mut::<Body>(*entity) else{ return };
        body.direction = direction;
        // let Ok(mut transform) = bodies.get_component_mut::<Transform>(*entity) else{ return };
        // move_forward(&mut transform, direction, speed, delta_time);
    }
}
