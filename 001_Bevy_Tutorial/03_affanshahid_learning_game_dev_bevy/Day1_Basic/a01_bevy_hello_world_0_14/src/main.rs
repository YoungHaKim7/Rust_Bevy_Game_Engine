use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::Color,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, ResMut},
    math::{primitives::Rectangle, Vec3},
    prelude::{default, PluginGroup},
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::Transform,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

// 0.14스타일
const GREEN_COLOR: Color = Color::srgba(0.0, 0.50196, 0.0, 1.0);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(150.)),
        material: materials.add(GREEN_COLOR),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}
