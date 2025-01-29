use bevy::{
    app::Startup,
    asset::Assets,
    color::{palettes::css::PURPLE, Color},
    math::Vec3,
    prelude::{App, Camera2d, Commands, Mesh, Mesh2d, Rectangle, ResMut, Transform},
    sprite::{ColorMaterial, MeshMaterial2d},
    DefaultPlugins,
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
