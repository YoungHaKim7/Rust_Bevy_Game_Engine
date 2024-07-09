use bevy::{
    app::{App, Plugin, Startup},
    ecs::{bundle::Bundle, system::Commands},
    math::Vec3,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{COLOR_PLATFORM, WINDOW_HEIGHT};

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;

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

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
    commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
    commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));
}
