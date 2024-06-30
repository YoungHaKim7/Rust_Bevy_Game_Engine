use bevy::{
    ecs::bundle::Bundle,
    math::Vec3,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;

const COLOR_PLATFORM: Color = Color::ANTIQUE_WHITE;

#[derive(Bundle)]
pub struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    pub fn new(x: f32, scale: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PLATFORM,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, WINDOW_BOTTOM_Y + (scale.y / 2.0), 0.0),
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}
