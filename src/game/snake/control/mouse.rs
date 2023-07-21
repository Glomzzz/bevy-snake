use bevy::prelude::*;

use crate::game::snake::components::{Body, Head};
use crate::game::{HEIGHT, WIDTH};

const ORIGIN: Vec3 = Vec3::new(-WIDTH / 2.0, HEIGHT / 2.0, 0.0);

pub fn cursor_movement(
    mut cursor_evr: EventReader<CursorMoved>,
    mut heads: Query<&mut Body, With<Head>>,
) {
    for mut body in heads.iter_mut().filter(|body| body.id == 1) {
        for cursor in cursor_evr.iter() {
            let cursor_position =
                ORIGIN + Vec3::new(cursor.position.x, -cursor.position.y, 0.000001);
            body.direction = cursor_position.normalize();
        }
    }
}
