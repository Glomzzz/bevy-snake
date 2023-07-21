use std::collections::LinkedList;

use bevy::prelude::*;

#[derive(Component)]
pub struct Snake {
    pub id: String,
    pub list: LinkedList<Entity>,
}
impl Snake {
    pub fn new(id: String) -> Self {
        Self {
            id,
            list: LinkedList::new(),
        }
    }
    pub fn add(&mut self, entity: Entity) {
        self.list.push_front(entity);
    }
}

#[derive(Component, Clone)]
pub struct Body {
    id: String,
    pub direction: Vec3,
}

impl Body {
    pub fn new(id: String) -> Self {
        Body {
            id,
            direction: Vec3::ZERO,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Component)]
pub struct FuseTimer {
    pub timer: Timer,
}

impl FuseTimer {
    pub fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.02, bevy::time::TimerMode::Repeating),
        }
    }
}

#[derive(Bundle)]
pub struct BodyBundle {
    pub body: Body,
    #[bundle()]
    pub sprite_bundle: SpriteBundle,
}

impl BodyBundle {
    pub fn new(body: Body, sprite_bundle: SpriteBundle) -> BodyBundle {
        BodyBundle {
            body,
            sprite_bundle,
        }
    }

    pub fn new_with_body(
        body: Body,
        translation: Vec3,
        assets_server: &Res<AssetServer>,
    ) -> BodyBundle {
        BodyBundle::new(
            body,
            SpriteBundle {
                transform: Transform::from_translation(translation),
                texture: assets_server.load("textures/snake_body.png"),
                ..default()
            },
        )
    }

    pub fn new_with_id(
        id: String,
        translation: Vec3,
        assets_server: &Res<AssetServer>,
    ) -> BodyBundle {
        BodyBundle::new(
            Body::new(id),
            SpriteBundle {
                transform: Transform::from_translation(translation),
                texture: assets_server.load("textures/snake_body.png"),
                ..default()
            },
        )
    }
    pub fn texture(mut self, texture: Handle<Image>) -> Self {
        self.sprite_bundle.texture = texture;
        self
    }
}
