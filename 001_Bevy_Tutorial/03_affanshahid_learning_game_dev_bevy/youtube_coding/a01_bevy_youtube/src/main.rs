use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::Color,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, ResMut},
    math::{primitives::Circle, Vec3},
    prelude::{default, PluginGroup},
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Sprite, SpriteBundle},
    transform::components::Transform,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;

const GREEN_COLOR: Color = Color::srgba(0.0, 0.50196, 0.0, 1.0);
const COLOR_PLAYER: Color = Color::srgba(0.78431, 0.39216, 0.39216, 1.0);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.39216, 0.39216, 0.03922, 1.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-100.0, WINDOW_BOTTOM_Y + (200.0 / 2.0), 0.),
            scale: Vec3::new(75.0, 200.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.03922, 0.39216, 0.0, 1.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(100.0, WINDOW_BOTTOM_Y + (350.0 / 2.0), 0.),
            scale: Vec3::new(50.0, 350.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.78431, 0.39216, 0.03922, 1.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(350.0, WINDOW_BOTTOM_Y + (250.0 / 2.0), 0.),
            scale: Vec3::new(150.0, 250.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Circle::default()).into(),
        material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
        transform: Transform {
            translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
            scale: Vec3::new(30.0, 30.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Tutorial".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: true,
                ..Default::default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}
