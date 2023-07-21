use bevy::prelude::Event;

#[derive(Event)]
pub struct GameStart;

#[derive(Event)]
pub struct GamePause;

#[derive(Event)]
pub struct GameOver {
    length: u32,
}

#[derive(Event)]
pub struct SnakeEatApple;