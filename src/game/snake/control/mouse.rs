use bevy::prelude::*;

use crate::game::snake::components::{Body, Snake};
use crate::game::{HEIGHT, WIDTH};

const origin: Vec3 = Vec3::new(-WIDTH / 2.0, HEIGHT / 2.0, 0.0);

pub fn move_cursor(
    mut cursor_evr: EventReader<CursorMoved>,
    mut snake: Query<&Snake>,
    mut bodies: Query<(Entity, &mut Body, &Transform)>,
) {
    for snake in snake.iter().filter(|snake| snake.id == "player") {
        let head_entity = snake.list.front().unwrap();
        for cursor in cursor_evr.iter() {
            let Ok(mut snake_head) = bodies.get_component_mut::<Body>(*head_entity) else {
                return;
            };

            let cursor_position =
                origin + Vec3::new(cursor.position.x, -cursor.position.y, 0.000001);

            snake_head.direction = cursor_position.normalize();
        }
    }
}
