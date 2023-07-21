use bevy::app::AppExit;
use bevy::prelude::*;

use crate::game::menu::components::Menu;
use crate::game::state::GameState;

pub fn load_menu(mut commands: Commands, assets_server: Res<AssetServer>) {
    let section = |str: &str| -> TextSection {
        TextSection::new(
            str,
            TextStyle {
                font: assets_server.load("fonts/JetBrainsMono-Bold.ttf"),
                font_size: 45.0,
                color: Color::WHITE,
            },
        )
    };
    let texts = TextBundle::from_sections([
        section("Start / Continue (S/C)\n"),
        section("Options (O)\n"),
        section("Exit (ESC)"),
    ])
    .with_style(Style {
        align_self: AlignSelf::Center,
        position_type: PositionType::Absolute,
        left: Val::Percent(15.2),
        ..default()
    })
    .with_text_alignment(TextAlignment::Center);
    commands.spawn((Menu, texts));
}

pub fn handle_action(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::C) {
        game_state.set(GameState::InGame)
    }
    if keyboard_input.pressed(KeyCode::O) {
        game_state.set(GameState::Options)
    }
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit)
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
