use bevy::prelude::*;

use crate::game::resources::Options;

use super::{components::*, control::move_forward};

const HALF_POSITION: Vec3 = Vec3::ZERO;

pub fn on_game_move_body(
    mut snake: Query<(&mut Snake, &mut FuseTimer)>,
    mut bodies: Query<(Entity, &Transform, &Body, &mut Handle<Image>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Body>)>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    time: Res<Time>,
    options: Res<Options>,
) {
    let speed = options.speed;
    let delta_time = time.delta_seconds();
    for (mut snake, mut timer) in snake.iter_mut() {
        timer.timer.tick(time.delta());
        if !timer.timer.finished() {
            continue;
        }
        if let Some(entity) = snake.list.pop_back() {
            commands.entity(entity).despawn();
        }
        if snake.list.front().is_none() {
            continue;
        }
        let Some(entity) = snake.list.front() else {
            return;
        };
        let Ok(body) = bodies.get_component::<Body>(*entity) else {
            return;
        };
        let Ok(transform) = bodies.get_component::<Transform>(*entity) else {
            return;
        };
        let mut new_head_bundle =
            BodyBundle::new_with_body(body.clone(), transform.translation, &assets_server)
                .texture(assets_server.load("textures/snake_head.png"));
        let direction = body.direction.clone();
        let Ok(mut texture) = bodies.get_component_mut::<Handle<Image>>(*entity) else {
            return;
        };
        *texture = assets_server.load("textures/snake_body.png");
        move_forward(
            &mut new_head_bundle.sprite_bundle.transform,
            camera.single_mut(),
            direction,
            speed,
            delta_time,
        );
        snake.add(commands.spawn(new_head_bundle).id());
    }
}

pub fn on_start_spawn_snake(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    options: Res<Options>,
    // mut start: EventReader<GameStart>,
) {
    let id = "player".to_owned();
    // for _ in start.iter() {
    let mut snake = Snake::new(id.clone());
    let head_bundle = BodyBundle::new_with_id(id.clone(), HALF_POSITION, &assets_server);
    snake.add(commands.spawn(head_bundle).id());
    let start_length = options.start_length;
    for index in 1..=start_length {
        let mut body_bundle = BodyBundle::new_with_id(id.clone(), HALF_POSITION, &assets_server);
        snake.add(commands.spawn(body_bundle).id());
    }
    commands.spawn((snake, FuseTimer::default()));
    // }
}
