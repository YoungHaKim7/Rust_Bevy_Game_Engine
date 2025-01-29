use bevy::{
    app::Startup,
    asset::Assets,
    color::{palettes::css::PURPLE, Color},
    math::Vec3,
    prelude::{App, Camera2d, Commands, Mesh, Mesh2d, Rectangle, ResMut, Transform},
    sprite::{ColorMaterial, MeshMaterial2d},
    DefaultPlugins,
};

const OFFSET_Y: f32 = 75.;

enum Shape {
    Rectangle(Rectangle),
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((Transform::from_xyz(0., OFFSET_Y, 0.)));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
