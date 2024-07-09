use std::time::Duration;

use bevy::{
    app::{App, Plugin, Startup, Update},
    color::Color,
    ecs::system::{Commands, Query, Res},
    input::{keyboard::KeyCode, ButtonInput},
    math::{Vec2, Vec3},
    prelude::{Camera2dBundle, Component, Entity, TextureAtlas, With, Without},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};

use bevy_rapier2d::{
    control::{KinematicCharacterController, KinematicCharacterControllerOutput},
    prelude::{Collider, RigidBody},
};

use crate::WINDOW_WIDTH;
use crate::{animation::Animation, WINDOW_HEIGHT};

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;
const MAX_JUMP_HEIGHT: f32 = 230.0;
const FLOOR_THICKNESS: f32 = 10.0;

const SPRITESHEET_COLS: usize = 7;
const SPRITESHEET_ROWS: usize = 8;

const SPRITE_TILE_WIDTH: f32 = 128.0;
const SPRITE_TILE_HEIGHT: f32 = 256.0;

const SPRITE_RENDER_WIDTH: f32 = 64.0;
const SPRITE_RENDER_HEIGHT: f32 = 128.0;

const SPRITE_IDX_STAND: usize = 28;
const SPRITE_IDX_WALKING: &[usize] = &[7, 0];
const SPRITE_IDX_JUMP: usize = 35;

const CYCLE_DELAY: Duration = Duration::from_millis(70);

const COLOR_FLOOR: Color = Color::srgba(0.66275, 0.66275, 0.66275, 1.0);

const COLOR_PLAYER: Color = ORANGE_RED;

const LIME_GREEN_COLOR: Color = Color::srgba(0.19608, 0.80392, 0.01961, 1.0);
const AQUA_COLOR: Color = Color::hsl(180.0, 1.00, 0.5);
const ORANGE_RED: Color = Color::srgba(1.0, 0.26667, 0.20000, 1.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, movement)
            .add_systems(Update, jump)
            .add_systems(Update, rise)
            .add_systems(Update, fall)
            .add_systems(Update, apply_movement_animation)
            .add_systems(Update, apply_idle_sprite)
            .add_systems(Update, apply_jump_sprite)
            .add_systems(Update, update_direction)
            .add_systems(Update, update_sprite_direction);
    }
}

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

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
}

pub fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();

    let mut movement = 0.0;

    if input.pressed(KeyCode::ArrowRight) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::ArrowLeft) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)),
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }
}

#[derive(Component)]
struct Jump(f32);

fn jump(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if input.pressed(KeyCode::ArrowUp) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }
}

fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut jump) = query.single_mut();

    let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();
    let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 1.5) * -1.0;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn apply_movement_animation(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.desired_translation.x != 0.0 && output.grounded {
        commands
            .entity(player)
            .insert(Animation::new(SPRITE_IDX_WALKING, CYCLE_DELAY));
    }
}

fn apply_idle_sprite(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &KinematicCharacterControllerOutput,
        &mut TextureAtlas,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output, mut sprite) = query.single_mut();
    if output.desired_translation.x == 0.0 && output.grounded {
        commands.entity(player).remove::<Animation>();
        sprite.index = SPRITE_IDX_STAND
    }
}

fn apply_jump_sprite(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &KinematicCharacterControllerOutput,
        &mut TextureAtlas,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output, mut sprite) = query.single_mut();
    if !output.grounded {
        commands.entity(player).remove::<Animation>();
        sprite.index = SPRITE_IDX_JUMP
    }
}

fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if output.desired_translation.x > 0.0 {
        commands.entity(player).insert(Direction::Right);
    } else if output.desired_translation.x < 0.0 {
        commands.entity(player).insert(Direction::Left);
    }
}

fn update_sprite_direction(mut query: Query<(&mut TextureAtlas, &Direction)>) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, direction) = query.single_mut();

    match direction {
        Direction::Right => sprite.flip_y = false,
        Direction::Left => sprite.flip_x = true,
    }
}
