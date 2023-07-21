use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeTimer(pub(crate) Timer);

#[derive(Component)]
pub struct Game;

#[derive(Component)]
pub struct Apple;
#[derive(Component)]
pub struct HasApple(pub(crate) Option<()>);

#[derive(Component)]
pub struct Head;

impl Head {
    pub(crate) fn new() -> Self {
        Head {}
    }
}

#[derive(Component, Clone)]
pub struct Body {
    pub id: usize,
    pub direction: Vec3,
}

impl Body {
    pub fn new(id: usize) -> Self {
        Body {
            id,
            direction: Vec3::ZERO,
        }
    }
}

#[derive(Bundle)]
pub struct HeadBundle {
    pub head: Head,
    #[bundle()]
    pub body_bundle: BodyBundle,
}

impl HeadBundle {
    pub fn new(head: Head, body_bundle: BodyBundle) -> Self {
        HeadBundle { head, body_bundle }
    }

    pub fn new_head(body: Body, translation: Vec3, assets_server: &Res<AssetServer>) -> Self {
        HeadBundle::new(
            Head::new(),
            BodyBundle::new(
                body,
                SpriteBundle {
                    transform: Transform::from_translation(translation),
                    texture: assets_server.load("textures/snake_head.png"),
                    ..default()
                },
            ),
        )
    }
}

#[derive(Bundle)]
pub struct BodyBundle {
    pub body: Body,
    game: Game,
    #[bundle()]
    pub sprite_bundle: SpriteBundle,
}

impl BodyBundle {
    pub fn new(body: Body, sprite_bundle: SpriteBundle) -> Self {
        BodyBundle {
            body,
            game: Game {},
            sprite_bundle,
        }
    }

    pub fn new_body(id: usize, translation: Vec3, assets_server: &Res<AssetServer>) -> Self {
        BodyBundle::new(
            Body::new(id),
            SpriteBundle {
                transform: Transform::from_translation(translation),
                texture: assets_server.load("textures/snake_body.png"),
                ..default()
            },
        )
    }
}

#[derive(Component)]
pub struct Next {
    pub entity: Entity,
}

impl Next {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Component)]
pub struct Previous {
    pub entity: Entity,
}

impl Previous {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
