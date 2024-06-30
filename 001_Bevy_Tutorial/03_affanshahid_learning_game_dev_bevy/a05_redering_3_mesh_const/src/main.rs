use bevy::{
    app::{App, Startup},
    color::Color,
    ecs::system::Commands,
    math::Vec3,
    prelude::{Camera2dBundle, PluginGroup},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
// const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const LIME_GREEN_COLOR: Color = Color::srgba(0.19608, 0.80392, 0.01961, 1.0);

fn setup(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: LIME_GREEN_COLOR,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-100.0, WINDOW_BOTTOM_Y + (200.0 / 2.0), 0.0),
            scale: Vec3::new(75.0, 200.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(10.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(100.0, WINDOW_BOTTOM_Y + (350.0 / 2.0), 0.0),
            scale: Vec3::new(50.0, 350.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(10.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(350.0, WINDOW_BOTTOM_Y + (250.0 / 2.0), 0.0),
            scale: Vec3::new(150.0, 250.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(Camera2dBundle::default());
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}
