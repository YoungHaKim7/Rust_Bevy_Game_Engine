use bevy::{
    app::Startup,
    asset::AssetServer,
    color::Color,
    math::Vec2,
    prelude::{App, Camera2d, Commands, Component, Res},
    sprite::Sprite,
    text::{JustifyText, Text2d, TextFont, TextLayout},
    utils::default,
    DefaultPlugins,
};

#[derive(Component)]
struct AnimateTranslation;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    let text_justfication = JustifyText::Center;

    // 2d camera
    commands.spawn(Camera2d);

    // Demonstrate changing translation
    commands.spawn((
        Text2d::new("translation"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justfication),
        AnimateTranslation,
    ));

    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands.spawn((Sprite::from_color(Color::srgb(0.95, 0.25, 0.75), box_size)));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
