use bevy::{
    app::{App, Startup, Update},
    prelude::{ClearColor, PluginGroup},
    render::color::Color,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use player::{movement, setup};

mod platforms;
mod player;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const COLOR_BACKGROUND: Color = Color::BLACK;

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
        .add_systems(Update, movement) // new system added
        .run();
}
