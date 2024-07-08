use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::Color,
    ecs::system::{Commands, ResMut},
    math::{primitives::Circle, Vec3},
    prelude::{default, Camera2dBundle, ClearColor, ColorMaterial, PluginGroup},
    render::mesh::Mesh,
    sprite::{MaterialMesh2dBundle, Sprite, SpriteBundle},
    transform::components::Transform,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

use bevy_rapier2d::prelude::{
    Collider, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin, RigidBody,
};
use platforms::PlatformBundle;

mod platforms;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const COLOR_BACKGROUND: Color = Color::BLACK;
const COLOR_PLATFORM: Color = Color::WHITE;
const COLOR_PLAYER: Color = ORANGE_RED;

const FLOOR_THICKNESS: f32 = 10.0;
const COLOR_FLOOR: Color = Color::srgba(0.66275, 0.66275, 0.66275, 1.0);

const LIME_GREEN_COLOR: Color = Color::srgba(0.19608, 0.80392, 0.01961, 1.0);

const AQUA_COLOR: Color = Color::hsl(180.0, 1.00, 0.5);
const ORANGE_RED: Color = Color::srgba(1.0, 0.26667, 0.20000, 1.0);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
    commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
    commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Circle::default()).into(),
        material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
        transform: Transform {
            translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
            scale: Vec3::new(30.0, 30.0, 1.0),
            ..Default::default()
        },
        ..default()
    });
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: LIME_GREEN_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-100.0, 0.0, 0.0),
                scale: Vec3::new(75.0, 200.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: AQUA_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(100.0, 0.0, 0.0),
                scale: Vec3::new(50.0, 350.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: ORANGE_RED,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(350.0, 0.0, 0.0),
                scale: Vec3::new(150.0, 250.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
                scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Circle::default()).into(),
        material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
        transform: Transform {
            translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
            scale: Vec3::new(30.0, 30.0, 1.0),
            ..Default::default()
        },
        ..default()
    });
    commands.spawn(Camera2dBundle::default());
}
fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND)) // resource added
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0)) // Physics plugin
        .add_plugins(RapierDebugRenderPlugin::default()) // Debug plugin
        .add_systems(Startup, setup)
        .run();
}
