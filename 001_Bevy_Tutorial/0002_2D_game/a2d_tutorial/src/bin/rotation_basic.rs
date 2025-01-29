use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    math::Vec2,
    prelude::{Camera2d, Commands, Component, Res},
    sprite::Sprite,
    time::Time,
    DefaultPlugins,
};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}

//fn player_movement_system(
//    time: Res<Time>,
//)

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("textures/simplespace/ship_C.png");

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(ship_handle),
        Player {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(360.0),
        },
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
