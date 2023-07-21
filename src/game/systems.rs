use bevy::input::gamepad::GamepadConnectionEvent;
use bevy::winit::WinitWindows;
use bevy::{prelude::*, window::PrimaryWindow};
use winit::window::Icon;

use crate::game::resources::MyGamepad;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
    main_window: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(primary) = windows.get_window(main_window.single()) else {
        return;
    };

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("icon.ico")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}

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
