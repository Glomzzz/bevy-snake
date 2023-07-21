use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;
use bevy_kira_audio::{Audio, AudioControl};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::{random, thread_rng};

use crate::game::resources::Options;
use crate::game::snake::control::{move_forward, move_forward_player};
use crate::game::{HEIGHT, WIDTH};

use super::components::*;

const HALF_POSITION: Vec3 = Vec3::ZERO;

pub fn on_game_remove_tail(
    mut commands: Commands,
    mut tails: Query<(Entity, &Previous), Without<Next>>,
) {
    for (entity, previous) in tails.iter_mut() {
        if let Some(mut entity) = commands.get_entity(previous.entity) {
            entity.remove::<Next>();
        }
        commands.entity(entity).despawn();
    }
}

fn random_direction() -> Vec3 {
    let mut rng = thread_rng();
    let uniform = Uniform::from(-1.0..1.0);
    Vec3::new(uniform.sample(&mut rng), uniform.sample(&mut rng), 0.0001).normalize()
}

pub fn snake_random_direction(
    mut heads: Query<(&mut Body, &mut SnakeTimer), With<Head>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mut body, mut timer) in heads.iter_mut() {
        timer.0.tick(Duration::from_secs_f32(delta_time));
        if timer.0.finished() {
            body.direction = random_direction();
        }
    }
}

pub fn on_game_move_head(
    mut commands: Commands,
    mut heads: Query<(Entity, &mut Transform, &Body, &mut Next), With<Head>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Body>)>,
    assets_server: Res<AssetServer>,
    time: Res<Time>,
    options: Res<Options>,
) {
    let speed = options.speed;
    let delta_time = time.delta_seconds();
    let mut camera = camera.single_mut();
    for (head, mut transform, body, mut next) in heads.iter_mut() {
        let translation = transform.translation;
        let new_body = commands
            .spawn((
                BodyBundle::new_body(body.id, translation, &assets_server),
                Previous::new(head),
                Next::new(next.entity),
            ))
            .id();
        commands.entity(next.entity).insert(Previous::new(new_body));
        next.entity = new_body;
        if body.id == 1 {
            move_forward_player(
                &mut transform,
                &mut camera,
                body.direction,
                speed,
                delta_time,
            );
        } else {
            move_forward(&mut transform, body.direction, speed, delta_time);
        }
    }
}

fn spawn_snake(
    commands: &mut Commands,
    player: bool,
    start_length: u32,
    assets_server: &Res<AssetServer>,
    pos: Vec3,
) {
    let id: usize = if player { 1 } else { random() };
    let head_bundle = HeadBundle::new_head(Body::new(id), pos, assets_server);
    let mut vec = Vec::with_capacity(start_length as usize);
    if player {
        vec.push(commands.spawn((head_bundle, HasApple(None))).id());
    } else {
        vec.push(
            commands
                .spawn((
                    head_bundle,
                    HasApple(None),
                    SnakeTimer(Timer::from_seconds(0.5, Repeating)),
                ))
                .id(),
        );
    };
    for _ in 2..=start_length {
        let body = commands.spawn(BodyBundle::new_body(id, pos, assets_server));
        vec.push(body.id());
    }
    for (index, this) in vec.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let previous = vec.get(index - 1).unwrap();
        commands.entity(*this).insert(Previous::new(*previous));
        commands.entity(*previous).insert(Next::new(*this));
    }
}

pub fn on_start_spawn_snake(
    mut commands: Commands,
    mut query: Query<&mut Visibility, With<Game>>,
    assets_server: Res<AssetServer>,
    options: Res<Options>,
) {
    if !query.is_empty() {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Visible
        }
        return;
    }
    let start_length = options.start_length;
    spawn_snake(
        &mut commands,
        true,
        start_length,
        &assets_server,
        HALF_POSITION,
    );
    spawn_snake(
        &mut commands,
        false,
        start_length,
        &assets_server,
        Vec3::new(100.0, 100.0, 0.0),
    );
}

pub fn on_exit_invisible_snake(mut query: Query<&mut Visibility, With<Game>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden
    }
}

pub fn eat_apple(
    mut commands: Commands,
    apples: Query<(Entity, &Transform), (With<Apple>, Without<Head>)>,
    assets_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut heads: Query<(Entity, &mut Transform, &mut Next, &Body, &mut HasApple), With<Head>>,
) {
    for (apple_entity, apple_transform) in apples.iter() {
        for (head_entity, transform, mut next, body, mut has_apple) in heads.iter_mut() {
            if apple_transform.translation.distance(transform.translation) >= 12.0 {
                continue;
            }
            audio.play(assets_server.load("sounds/eat.mp3"));
            has_apple.0 = None;
            commands.entity(apple_entity).despawn();
            let new_body =
                BodyBundle::new_body(body.id, apple_transform.translation, &assets_server);
            let new_body = commands
                .spawn((new_body, Next::new(next.entity), Previous::new(head_entity)))
                .id();
            commands.entity(next.entity).insert(Previous::new(new_body));
            next.entity = new_body;
        }
    }
}

pub fn apple_spawner(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut heads: Query<(&Transform, &mut HasApple), With<Head>>,
) {
    let mut rng = thread_rng();
    let uniform = Uniform::from(-1.0..1.0);
    for (head_pos, mut has_apple) in heads.iter_mut() {
        if has_apple.0.is_some() {
            continue;
        }
        let x = uniform.sample(&mut rng) * WIDTH / 2.0;
        let y = uniform.sample(&mut rng) * HEIGHT / 2.0;
        let translation: Vec3 = head_pos.translation + Vec3::new(x, y, 0.0);

        has_apple.0 = Some(());
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(translation),
                texture: assets_server.load("textures/apple.png"),
                ..default()
            },
            Game,
            Apple,
        ));
    }
}

pub fn play_bgm(audio: Res<Audio>, assets_server: Res<AssetServer>) {
    audio.play(assets_server.load("sounds/bgm.mp3")).looped();
}

pub fn snake_hit(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    audio: Res<Audio>,
    heads: Query<(Entity, &Transform, &Body), With<Head>>,
    bodies: Query<(Entity, &Transform, &Body), Without<Head>>,
) {
    for (head, head_pos, head_body) in heads.iter() {
        if bodies
            .iter()
            .filter(|(.., body)| body.id != head_body.id)
            .any(|(.., pos, _)| pos.translation.distance(head_pos.translation) <= 17.0)
        {
            audio.play(assets_server.load("sounds/dead.wav"));
            commands.entity(head).despawn()
        }
    }
}
