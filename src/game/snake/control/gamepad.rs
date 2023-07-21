use std::ops::Add;

use bevy::prelude::*;

use crate::game::resources::MyGamepad;
use crate::game::snake::components::{Body, Head};

// use super::move_forward;

pub fn gamepad_movement(
    mut heads: Query<&mut Body, With<Head>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    // time: Res<Time>,
    // options: Res<Options>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    let axis_lx = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickX,
    };
    let axis_ly = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickY,
    };

    // let speed = options.speed;
    // let delta_time = time.delta_seconds();
    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        if x == 0.0 && y == 0.0 {
            return;
        }
        let direction = Vec3::new(x, y, 0.0)
            .normalize()
            .add(Vec3::new(0.0, 0.0, 0.0000001));
        for mut body in heads.iter_mut().filter(|body| body.id == 1) {
            body.direction = direction;
        }
        // // Example: check if the stick is pushed up
        // if left_stick_pos.length() > 0.9 && left_stick_pos.y > 0.5 {
        //     // do something
        // }
    }

    // // In a real game, the buttons would be configurable, but here we hardcode them
    // let jump_button = GamepadButton { gamepad, button_type: GamepadButtonType:: };
    // let heal_button = GamepadButton(gamepad, GamepadButtonType::East);
    //
    // if buttons.just_pressed(jump_button) {
    //     // button just pressed: make the player jump
    // }
    //
    // if buttons.pressed(heal_button) {
    //     // button being held down: heal the player
    // }
}
