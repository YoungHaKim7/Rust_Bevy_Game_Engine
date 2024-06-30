use bevy::{
    app::{App, Startup},
    color::Color,
    ecs::system::Commands,
    math::Vec3,
    prelude::Camera2dBundle,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    DefaultPlugins,
};

fn setup(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(50.0, 100.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
