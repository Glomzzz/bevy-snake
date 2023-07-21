use std::ops::Add;

use crate::{
    game::resources::Options,
    game::snake::components::{Body, Snake},
};

use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};

// use super::move_forward;

#[derive(Resource)]
pub struct MyGamepad(Gamepad);

pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    for event in gamepad_evr.iter() {
        let gamepad = event.gamepad;
        if event.connected() {
            println!("New gamepad connected with ID: {:?}", gamepad);

            // if we don't have any gamepad yet, use this one
            if my_gamepad.is_none() {
                commands.insert_resource(MyGamepad(gamepad));
            }
        } else {
            println!("Lost gamepad connection with ID: {:?}", gamepad);

            // if it's the one we previously associated with the player,
            // disassociate it:
            if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                if old_id == &gamepad {
                    commands.remove_resource::<MyGamepad>();
                }
            }
        }
    }
}

pub fn gamepad_movement(
    mut snake: Query<&Snake>,
    mut bodies: Query<(&mut Transform, &mut Body)>,
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

        for snake in snake.iter_mut() {
            if snake.id != "player" {
                return;
            }
            if snake.list.front().is_none() {
                continue;
            }
            let Some(entity) = snake.list.front() else{ return };
            let Ok(mut body) = bodies.get_component_mut::<Body>(*entity) else{ return };
            body.direction = direction;
            // let Ok(mut transform) = bodies.get_component_mut::<Transform>(*entity) else{ return };
            // move_forward(&mut transform, direction, speed, delta_time)
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
