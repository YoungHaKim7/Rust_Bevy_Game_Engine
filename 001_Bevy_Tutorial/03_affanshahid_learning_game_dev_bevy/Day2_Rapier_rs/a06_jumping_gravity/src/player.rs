use bevy::{
    asset::Assets,
    color::Color,
    ecs::system::{Commands, Query, Res, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    math::{primitives::Circle, Vec2, Vec3},
    prelude::{default, Camera2dBundle, ColorMaterial, Component, Entity, With, Without},
    render::mesh::Mesh,
    sprite::{MaterialMesh2dBundle, Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};

use bevy_rapier2d::{
    control::{KinematicCharacterController, KinematicCharacterControllerOutput},
    parry::shape,
    prelude::{Collider, RigidBody},
};

use crate::platforms::PlatformBundle;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;
const MAX_JUMP_HEIGHT: f32 = 230.0;
const FLOOR_THICKNESS: f32 = 10.0;

const COLOR_FLOOR: Color = Color::srgba(0.66275, 0.66275, 0.66275, 1.0);

const COLOR_PLAYER: Color = ORANGE_RED;

const LIME_GREEN_COLOR: Color = Color::srgba(0.19608, 0.80392, 0.01961, 1.0);
const AQUA_COLOR: Color = Color::hsl(180.0, 1.00, 0.5);
const ORANGE_RED: Color = Color::srgba(1.0, 0.26667, 0.20000, 1.0);

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

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
    // commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
    // commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
            transform: Transform {
                translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 1.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default());
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

// pub fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
//     commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
//     commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));

//     commands.spawn(Camera2dBundle::default());

//     commands
//         .spawn(MaterialMesh2dBundle {
//             // mesh: meshes.add(shape::Circle::default()).into(),
//             mesh: meshes.add(Circle::default()).into(),
//             material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
//             transform: Transform {
//                 translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
//                 scale: Vec3::new(30.0, 30.0, 1.0),
//                 ..Default::default()
//             },
//             ..default()
//         })
//         .insert(RigidBody::KinematicPositionBased)
//         .insert(Collider::ball(0.5))
//         .insert(KinematicCharacterController::default());

//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: COLOR_FLOOR,
//                 ..Default::default()
//             },
//             transform: Transform {
//                 translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
//                 scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert(RigidBody::Fixed)
//         .insert(Collider::cuboid(0.5, 0.5));
// }
